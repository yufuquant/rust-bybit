# rust-bybit

[![Build Status]](https://github.com/yufuquant/rust-bybit/actions/workflows/ci.yaml)

[build status]: https://github.com/yufuquant/rust-bybit/actions/workflows/ci.yaml/badge.svg?branch=main

English | [简体中文](README-zh_CN.md)

Unofficial Rust API connector for Bybit's WebSockets V5 APIs.

## Disclaimer

This is an **unofficial** Rust API connector for Bybit's APIs and the user assumes all responsibility and risk for the use of this project.

## Installation

Add this to Cargo.toml

```toml
[dependencies]
rust-bybit = { git = "https://github.com/yufuquant/rust-bybit.git" }
```

## Basic Usage

Create a WebSocket client for specific channel:

```rust
use bybit::ws::response::SpotPublicResponse;
use bybit::ws::spot;
use bybit::KlineInterval;
use bybit::WebSocketApiClient;

let mut client = WebSocketApiClient::spot().build();
```

Subscribe to topics you are interested in. The following code will subscribe to all topics with symbol=ETHUSDT, or symbol=BTC3SUSDT for leveraged token (for all available topics, please check [Bybit V5 API](https://bybit-exchange.github.io/docs/v5/intro)). Note that the subscriptions will not be sent until `client.run` is called:

```rust
let symbol = "ETHUSDT";
let lt_symbol = "BTC3SUSDT";

client.subscribe_orderbook(symbol, spot::OrderbookDepth::Level1);
client.subscribe_orderbook(symbol, spot::OrderbookDepth::Level50);
client.subscribe_trade(symbol);
client.subscribe_ticker(symbol);
client.subscribe_kline(symbol, KlineInterval::Min1);
client.subscribe_lt_kline(lt_symbol, KlineInterval::Min5);
client.subscribe_lt_ticker(lt_symbol);
client.subscribe_lt_nav(lt_symbol);
```

Pass a callback function to `client.run` to start the client. The callback must accept exactly one parameter: the `Enum` which variants are WebSocket responses. The callback function will be called whenever a WebSocket response is received:

```rust
let callback = |res: SpotPublicResponse| match res {
    SpotPublicResponse::Orderbook(res) => println!("Orderbook: {:?}", res),
    SpotPublicResponse::Trade(res) => println!("Trade: {:?}", res),
    SpotPublicResponse::Ticker(res) => println!("Ticker: {:?}", res),
    SpotPublicResponse::Kline(res) => println!("Kline: {:?}", res),
    SpotPublicResponse::LtTicker(res) => println!("LtTicker: {:?}", res),
    SpotPublicResponse::LtNav(res) => println!("LtNav: {:?}", res),
    SpotPublicResponse::Op(res) => println!("Op: {:?}", res),
};

match client.run(callback) {
    Ok(_) => {}
    Err(e) => println!("{}", e),
}
```

This is a simple example that just print the received WebSocket responses. There are some more complex [examples](https://github.com/yufuquant/rust-bybit/tree/main/examples) for real usage demonstration, such as maintaining a local order book. You can run `cargo run --example local_orderbook` to see how it works.

## Donate

You can donate to following cryptocurrency wallet addresses to help this project going further.

| Network                 | Address                                    |
| ----------------------- | ------------------------------------------ |
| Ethereum (ERC20)        | 0x2ef22ed84D6b57496dbb95257C4eb8F02cE9b7A6 |
| BNB Smart Chain (BEP20) | 0x869F8F9A78a18818F93061A02B233507b5F64151 |
| Tron (TRC20)            | TPvqJYHFQ7iqEgtEcYrSLTjpGsAq41dhFt         |
| Bitcoin                 | 3C6o4ADGFXyuf6TUXKL6YyMyRfhek6zxzx         |
