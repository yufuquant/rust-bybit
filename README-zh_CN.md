# rust-bybit

[![Build Status]](https://github.com/yufuquant/rust-bybit/actions/workflows/ci.yaml)

[build status]: https://github.com/yufuquant/rust-bybit/actions/workflows/ci.yaml/badge.svg?branch=main

[English](./README.md) | 简体中文

Rust 实现的 Bybit WebSocket 行情和交易接口**非官方** SDK。

## 免责声明

本项目是 Rust 实现的 Bybit 行情和交易接口**非官方** SDK。加密货币市场风险巨大，使用本项目前请仔细评估风险，如因使用本项目而造成的一切损失均由使用者自行承担。

## 安装：

将以下依赖加到 Cargo.toml

```toml
[dependencies]
rust-bybit = { git = "https://github.com/yufuquant/rust-bybit.git" }
```

## 基础用法

根据需订阅的消息类型，创建对应的 client：

```rust
use bybit::spot::ws::{PublicResponse, PublicWebSocketApiClient};

let mut client = PublicWebSocketApiClient::new("wss://stream.bybit.com/spot/quote/ws/v1");
```

订阅感兴趣的消息。例如下面的代码将订阅 BTCUSDT 交易对的全部消息（关于有哪些消息类型可供订阅，请参考 [Bybit APIs 官方文档](https://bybit-exchange.github.io/docs/zh-cn/spot/)）。注意订阅请求直到 `client.run` 被调用时才会被发送：

```rust
client.subscribe_trade("BTCUSDT", false);
client.subscribe_realtimes("BTCUSDT", false);
client.subscribe_kline("BTCUSDT", "1m", false);
client.subscribe_depth("BTCUSDT", false);
client.subscribe_merged_depth("BTCUSDT", false, 1);
client.subscribe_diff_depth("BTCUSDT", false);
client.subscribe_lt("BTC3LUSDTNAV", false);
```

调用 `client.run` 方法并传入一个回调函数以启动 client。回调函数接受一个 WebSocket 应答枚举类型作为其唯一参数。每当收到一条 WebSocket 应答消息时，该回调函数都会被调用：

```rust
let callback = |res: PublicResponse| match res {
    PublicResponse::Trade(res) => println!("Trade: {:?}", res),
    PublicResponse::Realtimes(res) => println!("Realtimes: {:?}", res),
    PublicResponse::Kline(res) => println!("Kline: {:?}", res),
    PublicResponse::Depth(res) => println!("Depth: {:?}", res),
    PublicResponse::MergedDepth(res) => println!("Merged depth: {:?}", res),
    PublicResponse::DiffDepth(res) => println!("Diff depth: {:?}", res),
    PublicResponse::LT(res) => println!("LT: {:?}", res),
    PublicResponse::Pong(res) => println!("Pong: {:?}", res),
    PublicResponse::Ping(res) => println!("Ping: {:?}", res),
};

match client.run(callback) {
    Ok(_) => {}
    Err(e) => println!("{}", e),
}
```

以上是一个简单打印接收到的 WebSocket 应答消息的例子。[examples](https://github.com/yufuquant/rust-bybit/tree/main/examples) 中还有一些更为实际的例子可供参考，例如通过订阅 [diffDepth 消息](https://bybit-exchange.github.io/docs/zh-cn/spot/#t-websocketmergeddepth)维护一个本地订单薄。你可以运行 `cargo run --example spot_local_order_book` 启动此示例程序，程序启动后将在终端实时显示 BTCUSDT 10 档订单薄行情。

## 捐赠

您可以向下面的钱包地址进行捐赠以支持此项目的长远发展。

| 网络                    | 钱包地址                                   |
| ----------------------- | ------------------------------------------ |
| Ethereum (ERC20)        | 0x2ef22ed84D6b57496dbb95257C4eb8F02cE9b7A6 |
| BNB Smart Chain (BEP20) | 0x869F8F9A78a18818F93061A02B233507b5F64151 |
| Tron (TRC20)            | TPvqJYHFQ7iqEgtEcYrSLTjpGsAq41dhFt         |
| Bitcoin                 | 3C6o4ADGFXyuf6TUXKL6YyMyRfhek6zxzx         |
