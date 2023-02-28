use bybit::ws::response::SpotPublicResponse;
use bybit::ws::spot;
use bybit::KlineInterval;
use bybit::WebSocketApiClient;

fn main() {
    env_logger::init();

    let mut client = WebSocketApiClient::spot().build();

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

    let callback = |res: SpotPublicResponse| match res {
        SpotPublicResponse::Orderbook(res) => println!("Orderbook: {:?}", res),
        SpotPublicResponse::Trade(res) => println!("Trade: {:?}", res),
        SpotPublicResponse::Ticker(res) => println!("Ticker: {:?}", res),
        SpotPublicResponse::Kline(res) => println!("Kline: {:?}", res),
        SpotPublicResponse::LtTicker(res) => println!("LtTicker: {:?}", res),
        SpotPublicResponse::LtNav(res) => println!("LtNav: {:?}", res),
        SpotPublicResponse::Op(res) => println!("Op: {:?}", res),
    };

    if let Err(e) = client.run(callback) {
        eprintln!("Error: {e}")
    }
}
