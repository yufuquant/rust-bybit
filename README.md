# rust-bybit

English | [简体中文](README-zh_CN.md)

Unofficial Rust API connector for Bybit's WebSockets APIs.

## Disclaimer

This is an **unofficial** Rust API connector for Bybit's APIs and the user assumes all responsibility and risk for the use of this project.

## Installation

Add this to Cargo.toml

```toml
[dependencies]
rust-bybit = { git = "https://github.com/yufuquant/rust-bybit.git" }
```

## Basic Usage

Create a WebSocket client for specific topics:

```rust
use bybit::spot::ws::{PublicResponse, PublicWebSocketApiClient};

let mut client = PublicWebSocketApiClient::new("wss://stream.bybit.com/spot/quote/ws/v1");
```

Subscribe to topics you are interested in. The following code will subscribe to all topics with symbol=BTCUSDT and binary=false (for all available topics, check [Bybit APIs documentation](https://bybit-exchange.github.io/docs/spot/)). Note that the subscriptions will not be sent until `client.run` is called:

```rust
client.subscribe_trade("BTCUSDT", false);
client.subscribe_realtimes("BTCUSDT", false);
client.subscribe_kline("BTCUSDT", "1m", false);
client.subscribe_depth("BTCUSDT", false);
client.subscribe_merged_depth("BTCUSDT", false, 1);
client.subscribe_diff_depth("BTCUSDT", false);
client.subscribe_lt("BTC3LUSDTNAV", false);
```

Pass a callback to `client.run` to start the client. The callback must accept exactly one parameter: the `Enum` which variants are WebSocket responses. The callback will be called whenever a WebSocket response is received:

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

This is a simple example to just print the received WebSocket responses. There are some more complex [examples](https://github.com/yufuquant/rust-bybit/tree/main/examples) for real usage demonstration, such as maintaining a local order book by subscribing [diffDepth topic](https://bybit-exchange.github.io/docs/zh-cn/spot/#t-websocketmergeddepth). You can run `cargo run --example spot_local_order_book` to see how it works.

## Donate

You can donate to following cryptocurrency wallet addresses to help this project going further.

| Network                 | Address                                    |
| ----------------------- | ------------------------------------------ |
| Ethereum (ERC20)        | 0x2ef22ed84D6b57496dbb95257C4eb8F02cE9b7A6 |
| BNB Smart Chain (BEP20) | 0x869F8F9A78a18818F93061A02B233507b5F64151 |
| Tron (TRC20)            | TPvqJYHFQ7iqEgtEcYrSLTjpGsAq41dhFt         |
| Bitcoin                 | 3C6o4ADGFXyuf6TUXKL6YyMyRfhek6zxzx         |
