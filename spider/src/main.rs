// 爬虫项目主入口
mod config;
mod error;
mod spider;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志（使用更详细的格式）
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .init();
    
    info!("🚀 爬虫服务启动中...");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // 加载配置
    let config = config::SpiderConfig::load()?;
    info!("📋 配置加载完成");
    info!("  User-Agent: {}", config.user_agent);
    info!("  超时时间: {}秒", config.timeout);
    info!("  请求间隔: {}毫秒", config.delay_ms);
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // 创建爬虫实例
    let spider = spider::Spider::new(config);
    
    // 运行爬虫
    spider.run().await?;
    
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("✅ 爬虫任务完成");
    Ok(())
}

