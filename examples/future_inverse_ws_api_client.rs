use bybit::ws::future;
use bybit::ws::response::FuturePublicResponse;
use bybit::KlineInterval;
use bybit::WebSocketApiClient;

fn main() {
    env_logger::init();

    let mut client = WebSocketApiClient::future_inverse().build();

    let symbol = "ETHUSD";

    client.subscribe_orderbook(symbol, future::OrderbookDepth::Level1);
    client.subscribe_orderbook(symbol, future::OrderbookDepth::Level50);
    client.subscribe_trade(symbol);
    client.subscribe_ticker(symbol);
    client.subscribe_kline(symbol, KlineInterval::Min1);
    client.subscribe_liquidation(symbol);

    if let Err(e) = client.run(|res| match res {
        FuturePublicResponse::Orderbook(res) => println!("Orderbook: {:?}", res),
        FuturePublicResponse::Trade(res) => println!("Trade: {:?}", res),
        FuturePublicResponse::Ticker(res) => println!("Ticker: {:?}", res),
        FuturePublicResponse::Kline(res) => println!("Kline: {:?}", res),
        FuturePublicResponse::Liquidation(res) => println!("Liquidation: {:?}", res),
        FuturePublicResponse::Op(res) => println!("Op: {:?}", res),
    }) {
        eprintln!("Error: {e}")
    }
}
