# ChromeDriver 安装和启动指南

## 问题
使用 `thirtyfour` 需要 ChromeDriver 运行在 `localhost:9515`。

## 安装 ChromeDriver

### macOS (使用 Homebrew)
```bash
brew install chromedriver
```

### 手动安装
1. 访问 https://chromedriver.chromium.org/downloads
2. 下载与您的 Chrome 版本匹配的 ChromeDriver
   - Chrome 110 需要 ChromeDriver 110.x.x.x
3. 解压并将 `chromedriver` 放到 PATH 中

## 启动 ChromeDriver

在运行爬虫之前，需要先启动 ChromeDriver：

```bash
chromedriver --port=9515
```

或者让它在后台运行：

```bash
chromedriver --port=9515 &
```

## 验证安装

```bash
chromedriver --version
```

应该显示类似：`ChromeDriver 110.0.5481.77`

## 运行爬虫

1. 在一个终端启动 ChromeDriver：
   ```bash
   chromedriver --port=9515
   ```

2. 在另一个终端运行爬虫：
   ```bash
   cargo run -p football-spider
   ```

## 注意事项

- ChromeDriver 版本必须与 Chrome 浏览器版本匹配
- 当前 Chrome 版本：110.0.5481.77
- 需要 ChromeDriver 版本：110.x.x.x

