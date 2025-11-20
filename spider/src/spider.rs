// 爬虫核心逻辑
use crate::config::SpiderConfig;
use crate::error::{SpiderError, SpiderResult};
use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

pub struct Spider {
    config: SpiderConfig,
    client: Client,
}

impl Spider {
    /// 创建新的爬虫实例
    pub fn new(config: SpiderConfig) -> Self {
        let mut client_builder = Client::builder()
            .user_agent(&config.user_agent)
            .timeout(Duration::from_secs(config.timeout));

        // 如果启用代理，设置代理
        if config.use_proxy {
            if let Some(proxy_url) = &config.proxy_url {
                if let Ok(proxy) = reqwest::Proxy::http(proxy_url) {
                    client_builder = client_builder.proxy(proxy);
                }
            }
        }

        let client = client_builder.build().expect("创建HTTP客户端失败");

        Self { config, client }
    }

    /// 运行爬虫 - 爬取 sportbet.one
    pub async fn run(&self) -> Result<()> {
        info!("🚀 开始爬取 sportbet.one...");
        
        let url = "https://sportbet.one/zh-CN/sports";
        
        // 使用 headless_chrome 获取页面
        info!("📄 正在使用浏览器加载页面: {}", url);
        self.extract_data_with_browser(url).await?;
        
        Ok(())
    }

    /// 使用浏览器提取页面数据
    async fn extract_data_with_browser(&self, url: &str) -> Result<()> {
        use thirtyfour::prelude::*;
        use thirtyfour::{DesiredCapabilities, WebDriver};
        
        info!("正在启动浏览器 (使用 thirtyfour)...");
        
        // 创建 Chrome 能力配置，兼容 Chrome 110
        let mut caps = DesiredCapabilities::chrome();
        
        // 设置 User-Agent
        caps.add_chrome_arg("user-agent=Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.77 Safari/537.36")?;
        
        // 非 headless 模式，更容易绕过 Cloudflare
        // caps.add_chrome_arg("--headless")?; // 注释掉，使用非 headless 模式
        
        // 其他有用的参数
        caps.add_chrome_arg("--no-sandbox")?;
        caps.add_chrome_arg("--disable-dev-shm-usage")?;
        caps.add_chrome_arg("--disable-blink-features=AutomationControlled")?;
        
        // 连接到 WebDriver（默认 localhost:9515）
        // 需要先启动 ChromeDriver: chromedriver --port=9515
        info!("尝试连接 ChromeDriver (http://localhost:9515)...");
        info!("提示: 如果连接失败，请先安装并启动 ChromeDriver:");
        info!("  1. 安装: brew install chromedriver (macOS) 或从 https://chromedriver.chromium.org/ 下载");
        info!("  2. 启动: chromedriver --port=9515");
        info!("  3. Chrome 110 需要 ChromeDriver 110.x.x.x 版本");
        
        let driver = WebDriver::new("http://localhost:9515", caps).await
            .map_err(|e| {
                anyhow::anyhow!(
                    "连接 WebDriver 失败: {}\n\n请先安装并启动 ChromeDriver:\n1. 安装: brew install chromedriver (macOS)\n2. 启动: chromedriver --port=9515\n3. 确保 ChromeDriver 版本与 Chrome 110 兼容",
                    e
                )
            })?;
        
        info!("✅ 浏览器连接成功");
        
        // 先访问首页建立会话
        info!("先访问首页以建立会话...");
        let home_url = "https://sportbet.one";
        driver.goto(home_url).await
            .map_err(|e| anyhow::anyhow!("访问首页失败: {}", e))?;
        sleep(Duration::from_secs(5)).await;
        
        // 导航到目标页面
        info!("导航到目标页面: {}", url);
        driver.goto(url).await
            .map_err(|e| anyhow::anyhow!("导航失败: {}", e))?;
        
        // 等待页面加载完成
        info!("等待页面加载...");
        sleep(Duration::from_secs(10)).await;
        
        // 检查页面是否加载完成
        let ready_state: String = driver.execute("return document.readyState;", vec![]).await
            .map_err(|e| anyhow::anyhow!("检查页面状态失败: {}", e))?
            .convert()
            .map_err(|e| anyhow::anyhow!("解析页面状态失败: {}", e))?;
        info!("页面状态: {}", ready_state);
        
        // 打印页面渲染后的HTML内容
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📄 打印页面渲染后的HTML内容...");
        let html: String = driver.execute("return document.documentElement.outerHTML;", vec![]).await
            .map_err(|e| anyhow::anyhow!("获取HTML失败: {}", e))?
            .convert()
            .map_err(|e| anyhow::anyhow!("解析HTML失败: {}", e))?;
        
        let preview = if html.len() > 5000 {
            format!("{}...\n(共{}字符，已截断)", &html[..5000], html.len())
        } else {
            html
        };
        info!("页面HTML内容:\n{}", preview);
        
        // 打印页面文本内容
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📝 打印页面文本内容...");
        let text: String = driver.execute("return document.body.innerText;", vec![]).await
            .map_err(|e| anyhow::anyhow!("获取文本失败: {}", e))?
            .convert()
            .map_err(|e| anyhow::anyhow!("解析文本失败: {}", e))?;
        
        let text_preview = if text.len() > 2000 {
            format!("{}...\n(共{}字符，已截断)", &text[..2000], text.len())
        } else {
            text
        };
        info!("页面文本内容:\n{}", text_preview);
        
        // 1. 提取 group-event-item 数据
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📊 提取 group-event-item 数据...");
        
        let event_js = r#"
            (() => {
                try {
                    const items = document.querySelectorAll('.group-events-body.card-body .group-event-item');
                    const result = Array.from(items).map(item => item.innerText.trim()).filter(t => t);
                    return result.length > 0 ? result : [];
                } catch (e) {
                    return [];
                }
            })()
        "#;
        
        // 先获取原始返回值，便于调试
        let raw_result = driver.execute(event_js, vec![]).await
            .map_err(|e| anyhow::anyhow!("执行JS失败: {}", e))?;
        
        // 尝试解析为数组
        match raw_result.convert::<Vec<String>>() {
            Ok(events) => {
                if events.is_empty() {
                    warn!("未找到 group-event-item 元素");
                } else {
                    for (i, text) in events.iter().enumerate() {
                        if !text.is_empty() {
                            info!("  [{}] {}", i + 1, text);
                        }
                    }
                    info!("✅ 共找到 {} 个 group-event-item", events.len());
                }
            }
            Err(e) => {
                // 如果解析失败，尝试获取原始值
                warn!("解析事件数据失败: {}，尝试获取原始值...", e);
                if let Ok(raw_value) = raw_result.convert::<serde_json::Value>() {
                    warn!("原始返回值: {:?}", raw_value);
                }
            }
        }
        
        // 2. 使用 XPath 提取链接的 href
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("🔗 使用 XPath 提取链接 href 属性...");
        
        let link_js = r#"
            (() => {
                try {
                    // 使用 XPath 查找链接
                    const xpath = "//*[@id='root']/div[1]/div/main/div[4]/div/div/div[2]/div[1]/a";
                    const result = document.evaluate(
                        xpath,
                        document,
                        null,
                        XPathResult.ORDERED_NODE_SNAPSHOT_TYPE,
                        null
                    );
                    
                    const links = [];
                    for (let i = 0; i < result.snapshotLength; i++) {
                        const link = result.snapshotItem(i);
                        if (link && link.tagName === 'A') {
                            const href = link.getAttribute('href');
                            if (href) {
                                links.push({
                                    text: link.innerText.trim(),
                                    href: href,
                                    classes: link.className
                                });
                            }
                        }
                    }
                    
                    // 如果 XPath 没找到，尝试 CSS 选择器
                    if (links.length === 0) {
                        const selectors = [
                            'a.event-short.nav-link',
                            'a.event-short',
                            'a[href^="/zh-CN/sports/"]'
                        ];
                        
                        for (const selector of selectors) {
                            const elements = document.querySelectorAll(selector);
                            if (elements.length > 0) {
                                for (const link of elements) {
                                    const href = link.getAttribute('href');
                                    if (href) {
                                        links.push({
                                            text: link.innerText.trim(),
                                            href: href,
                                            classes: link.className
                                        });
                                    }
                                }
                                if (links.length > 0) break;
                            }
                        }
                    }
                    
                    return links;
                } catch (e) {
                    return [{ error: e.toString() }];
                }
            })()
        "#;
        
        #[derive(serde::Deserialize, Debug)]
        struct LinkData {
            #[serde(default)]
            text: String,
            #[serde(default)]
            href: String,
            #[serde(default)]
            classes: String,
            #[serde(default)]
            error: Option<String>,
        }
        
        let raw_links_result = driver.execute(link_js, vec![]).await
            .map_err(|e| anyhow::anyhow!("执行JS失败: {}", e))?;
        
        // 尝试解析链接数据
        let links: Vec<LinkData> = match raw_links_result.convert::<Vec<LinkData>>() {
            Ok(links) => links,
            Err(e) => {
                warn!("解析链接数据失败: {}，尝试获取原始值...", e);
                if let Ok(raw_value) = raw_links_result.convert::<serde_json::Value>() {
                    warn!("原始返回值: {:?}", raw_value);
                }
                Vec::new()
            }
        };
        
        // 检查是否有错误
        if let Some(link) = links.iter().find(|l| l.error.is_some()) {
            warn!("执行JS错误: {:?}", link.error);
        }
        
        // 过滤出有 href 的链接
        let valid_links: Vec<&LinkData> = links.iter().filter(|l| !l.href.is_empty() && l.error.is_none()).collect();
        
        for (i, link) in valid_links.iter().enumerate() {
            // 直接输出 href 字符串
            info!("  [{}] href: {}", i + 1, link.href);
            
            if !link.classes.is_empty() {
                info!("      类: {}", link.classes);
            }
            
            if !link.text.is_empty() {
                info!("      文本: {}", link.text);
            }
            
            // 完整 URL
            let full_url = if link.href.starts_with("http") {
                link.href.clone()
            } else if link.href.starts_with("/") {
                format!("https://sportbet.one{}", link.href)
            } else {
                format!("https://sportbet.one/{}", link.href)
            };
            info!("      完整URL: {}", full_url);
        }
        
        if valid_links.is_empty() {
            warn!("未找到有效的链接");
        } else {
            info!("✅ 共找到 {} 个有效链接", valid_links.len());
        }
        
        // 关闭浏览器
        driver.quit().await.ok();
        
        Ok(())
    }

    /// 获取网页内容并打印请求/响应信息
    pub async fn fetch_page_with_logging(&self, url: &str) -> SpiderResult<String> {
        let mut retries = 0;

        loop {
            info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            info!("📤 HTTP 请求");
            info!("  URL: {}", url);
            info!("  Method: GET");
            info!("  User-Agent: {}", self.config.user_agent);
            
            let request = self.client.get(url);
            
            match request.send().await {
                Ok(response) => {
                    let status = response.status();
                    let headers = response.headers();
                    
                    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    info!("📥 HTTP 响应");
                    info!("  Status: {} {}", status.as_u16(), status.as_str());
                    info!("  Headers:");
                    for (key, value) in headers.iter() {
                        if let Ok(value_str) = value.to_str() {
                            info!("    {}: {}", key, value_str);
                        }
                    }
                    
                    if status.is_success() {
                        match response.text().await {
                            Ok(text) => {
                                info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                info!("📦 响应数据");
                                info!("  长度: {} 字节", text.len());
                                
                                // 尝试解析为JSON
                                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                                    info!("  类型: JSON");
                                    match serde_json::to_string_pretty(&json) {
                                        Ok(json_str) => info!("  内容:\n{}", json_str),
                                        Err(e) => warn!("JSON格式化失败: {}", e),
                                    }
                                } else {
                                    // 如果不是JSON，只显示前500个字符
                                    let preview = if text.len() > 500 {
                                        format!("{}...", &text[..500])
                                    } else {
                                        text.clone()
                                    };
                                    info!("  类型: HTML/Text");
                                    info!("  预览:\n{}", preview);
                                }
                                
                                return Ok(text);
                            }
                            Err(e) => {
                                error!("解析响应失败: {}", e);
                                return Err(SpiderError::ParseError(e.to_string()));
                            }
                        }
                    } else {
                        warn!("HTTP错误状态码: {}", status);
                        if retries >= self.config.max_retries {
                            return Err(SpiderError::HttpError(
                                reqwest::Error::from(response.error_for_status().unwrap_err()),
                            ));
                        }
                    }
                }
                Err(e) => {
                    error!("请求失败: {}", e);
                    if retries >= self.config.max_retries {
                        return Err(SpiderError::HttpError(e));
                    }
                }
            }

            retries += 1;
            warn!("重试第 {} 次...", retries);
            sleep(Duration::from_millis(self.config.delay_ms)).await;
        }
    }

    /// 从HTML中提取API端点
    async fn extract_api_endpoints(&self, html: &str) -> Result<()> {
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("🔍 正在分析页面，查找API接口...");
        
        let document = Html::parse_document(html);
        let mut api_urls = Vec::new();
        
        // 1. 查找script标签中的API调用
        let script_selector = Selector::parse("script").unwrap();
        let mut all_js_content = String::new();
        
        for script in document.select(&script_selector) {
            // 获取内联脚本内容
            let script_text: String = script.text().collect();
            if !script_text.is_empty() {
                all_js_content.push_str(&script_text);
                all_js_content.push('\n');
            }
            
            // 获取外部脚本URL
            if let Some(src) = script.value().attr("src") {
                if src.starts_with("http") || src.starts_with("//") {
                    let full_url = if src.starts_with("//") {
                        format!("https:{}", src)
                    } else {
                        src.to_string()
                    };
                    if !api_urls.contains(&full_url) {
                        api_urls.push(full_url);
                    }
                }
            }
        }
        
        // 2. 在JavaScript代码中查找API模式
        let patterns = [
            // API端点模式
            (r#"https?://[^"'\s\)]+/api/[^"'\s\)]+"#, "API端点"),
            (r#"https?://[^"'\s\)]+/v\d+/[^"'\s\)]+"#, "版本化API"),
            (r#"https?://[^"'\s\)]+/graphql"#, "GraphQL"),
            (r#""baseURL"\s*:\s*"([^"]+)""#, "Base URL"),
            (r#"'baseURL'\s*:\s*'([^']+)'"#, "Base URL"),
            (r#""apiUrl"\s*:\s*"([^"]+)""#, "API URL"),
            (r#"'apiUrl'\s*:\s*'([^']+)'"#, "API URL"),
            (r#"fetch\(["']([^"']+)["']"#, "Fetch调用"),
            (r#"axios\.(get|post|put|delete)\(["']([^"']+)["']"#, "Axios调用"),
            (r#"\.get\(["']([^"']+)["']"#, "GET请求"),
            (r#"\.post\(["']([^"']+)["']"#, "POST请求"),
        ];
        
        for (pattern, desc) in &patterns {
            let re = regex::Regex::new(pattern).unwrap();
            for cap in re.captures_iter(&all_js_content) {
                let url = cap.get(1).or_else(|| cap.get(2)).or_else(|| cap.get(0));
                if let Some(url_match) = url {
                    let url_str = url_match.as_str().trim_matches('"').trim_matches('\'');
                    // 处理相对URL
                    let full_url = if url_str.starts_with("/") {
                        format!("https://sportbet.one{}", url_str)
                    } else {
                        url_str.to_string()
                    };
                    if full_url.starts_with("http") && !api_urls.contains(&full_url) {
                        info!("  📌 发现{}: {}", desc, full_url);
                        api_urls.push(full_url);
                    }
                }
            }
        }
        
        // 3. 查找JSON数据（可能包含API配置）
        if let Some(json_match) = regex::Regex::new(r#"window\.__INITIAL_STATE__\s*=\s*(\{.*?\})"#)
            .unwrap()
            .captures(&all_js_content)
        {
            if let Some(json_str) = json_match.get(1) {
                info!("  📌 发现初始状态数据");
                if let Ok(json) = serde_json::from_str::<Value>(json_str.as_str()) {
                    info!("  初始状态内容:\n{}", serde_json::to_string_pretty(&json)?);
                }
            }
        }
        
        if !api_urls.is_empty() {
            info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            info!("📡 发现 {} 个可能的API接口:", api_urls.len());
            for (i, api_url) in api_urls.iter().enumerate() {
                info!("  {}. {}", i + 1, api_url);
            }
            
            // 尝试请求这些API
            for api_url in api_urls {
                info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                info!("🔗 正在请求API: {}", api_url);
                let _ = self.fetch_page_with_logging(&api_url).await;
                sleep(Duration::from_millis(self.config.delay_ms)).await;
            }
        } else {
            info!("⚠️  未发现明显的API接口");
            info!("💡 提示: 页面可能是通过客户端JavaScript动态加载数据的");
        }
        
        Ok(())
    }

    /// 从HTML中提取WebSocket URL
    async fn extract_websocket_urls(&self, html: &str) -> Result<()> {
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("🔍 正在分析页面，查找WebSocket连接...");
        
        let document = Html::parse_document(html);
        let script_selector = Selector::parse("script").unwrap();
        let mut all_js_content = String::new();
        
        // 收集所有JavaScript内容
        for script in document.select(&script_selector) {
            let script_text: String = script.text().collect();
            if !script_text.is_empty() {
                all_js_content.push_str(&script_text);
                all_js_content.push('\n');
            }
        }
        
        // 查找WebSocket URL模式
        let ws_patterns = [
            (r#"ws://[^"'\s\)]+"#, "WebSocket"),
            (r#"wss://[^"'\s\)]+"#, "Secure WebSocket"),
            (r#""ws://[^"]+""#, "WebSocket字符串"),
            (r#"'ws://[^']+'"#, "WebSocket字符串"),
            (r#""wss://[^"]+""#, "Secure WebSocket字符串"),
            (r#"'wss://[^']+'"#, "Secure WebSocket字符串"),
            (r#"new WebSocket\(["']([^"']+)["']"#, "WebSocket构造函数"),
            (r#"WebSocket\(["']([^"']+)["']"#, "WebSocket调用"),
            (r#"socket\.io\(["']([^"']+)["']"#, "Socket.IO"),
        ];
        
        let mut ws_urls = Vec::new();
        
        for (pattern, desc) in &ws_patterns {
            let re = regex::Regex::new(pattern).unwrap();
            for cap in re.captures_iter(&all_js_content) {
                let url = cap.get(1).or_else(|| cap.get(0));
                if let Some(url_match) = url {
                    let url_str = url_match.as_str().trim_matches('"').trim_matches('\'');
                    // 处理相对URL
                    let full_url = if url_str.starts_with("/") {
                        format!("wss://sportbet.one{}", url_str)
                    } else {
                        url_str.to_string()
                    };
                    if (full_url.starts_with("ws://") || full_url.starts_with("wss://")) 
                        && !ws_urls.contains(&full_url) {
                        info!("  📌 发现{}: {}", desc, full_url);
                        ws_urls.push(full_url);
                    }
                }
            }
        }
        
        if !ws_urls.is_empty() {
            info!("🔌 发现 {} 个WebSocket连接:", ws_urls.len());
            for (i, ws_url) in ws_urls.iter().enumerate() {
                info!("  {}. {}", i + 1, ws_url);
            }
            
            // 连接并监听WebSocket
            for ws_url in ws_urls {
                info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                info!("🔌 正在连接WebSocket: {}", ws_url);
                if let Err(e) = self.connect_websocket(&ws_url).await {
                    error!("WebSocket连接失败: {}", e);
                }
            }
        } else {
            info!("⚠️  未发现WebSocket连接，尝试监听常见的WebSocket端点...");
            // 尝试常见的WebSocket端点
            let common_ws_endpoints = [
                "wss://sportbet.one/ws",
                "wss://sportbet.one/websocket",
                "wss://sportbet.one/socket.io",
            ];
            
            for endpoint in &common_ws_endpoints {
                info!("🔌 尝试连接: {}", endpoint);
                if let Err(e) = self.connect_websocket(endpoint).await {
                    warn!("连接失败: {}", e);
                }
            }
        }
        
        Ok(())
    }

    /// 连接WebSocket并监听消息
    async fn connect_websocket(&self, url: &str) -> Result<()> {
        use tokio_tungstenite::{connect_async, tungstenite::Message};
        use futures_util::StreamExt;
        
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("🔌 WebSocket 连接信息");
        info!("  URL: {}", url);
        
        match connect_async(url).await {
            Ok((ws_stream, _)) => {
                info!("✅ WebSocket 连接成功");
                info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                info!("📡 开始监听WebSocket消息...");
                info!("  (按 Ctrl+C 停止监听)");
                
                let (mut _write, mut read) = ws_stream.split();
                
                // 监听消息（最多30秒）
                let timeout = Duration::from_secs(30);
                let start = std::time::Instant::now();
                
                while start.elapsed() < timeout {
                    match tokio::time::timeout(Duration::from_secs(1), read.next()).await {
                        Ok(Some(Ok(message))) => {
                            match message {
                                Message::Text(text) => {
                                    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                    info!("📨 WebSocket 消息 (Text)");
                                    // 尝试解析为JSON
                                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                                        info!("  类型: JSON");
                                        match serde_json::to_string_pretty(&json) {
                                            Ok(json_str) => info!("  内容:\n{}", json_str),
                                            Err(e) => warn!("JSON格式化失败: {}", e),
                                        }
                                    } else {
                                        info!("  内容: {}", text);
                                    }
                                }
                                Message::Binary(data) => {
                                    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                    info!("📨 WebSocket 消息 (Binary)");
                                    info!("  长度: {} 字节", data.len());
                                    if let Ok(text) = String::from_utf8(data.clone()) {
                                        info!("  内容: {}", text);
                                    } else {
                                        info!("  十六进制: {:?}", &data[..data.len().min(100)]);
                                    }
                                }
                                Message::Ping(data) => {
                                    info!("📨 WebSocket Ping: {:?}", data);
                                }
                                Message::Pong(data) => {
                                    info!("📨 WebSocket Pong: {:?}", data);
                                }
                                Message::Close(frame) => {
                                    info!("📨 WebSocket Close: {:?}", frame);
                                    break;
                                }
                                _ => {}
                            }
                        }
                        Ok(Some(Err(e))) => {
                            error!("WebSocket错误: {}", e);
                            break;
                        }
                        Ok(None) => {
                            info!("WebSocket连接已关闭");
                            break;
                        }
                        Err(_) => {
                            // 超时，继续等待
                        }
                    }
                }
                
                info!("⏱️  监听超时，停止接收消息");
            }
            Err(e) => {
                return Err(anyhow::anyhow!("WebSocket连接失败: {}", e));
            }
        }
        
        Ok(())
    }

    /// 获取网页内容（原始方法，保留兼容性）
    pub async fn fetch_page(&self, url: &str) -> SpiderResult<String> {
        self.fetch_page_with_logging(url).await
    }

    /// 解析HTML并提取数据
    pub fn parse_html(&self, html: &str, selector: &str) -> SpiderResult<Vec<String>> {
        let document = Html::parse_document(html);
        let sel = Selector::parse(selector)
            .map_err(|e| SpiderError::ParseError(format!("选择器解析失败: {}", e)))?;

        let results: Vec<String> = document
            .select(&sel)
            .map(|element| element.text().collect::<String>())
            .collect();

        Ok(results)
    }
}
