use bybit::ws::option;
use bybit::ws::response::OptionPublicResponse;
use bybit::WebSocketApiClient;

fn main() {
    env_logger::init();

    let mut client = WebSocketApiClient::option().testnet().build();

    let symbol = "BTC-10MAR23-16000-C";
    let base_coin = "BTC";

    client.subscribe_orderbook(symbol, option::OrderbookDepth::Level25);
    client.subscribe_orderbook(symbol, option::OrderbookDepth::Level100);
    client.subscribe_trade(base_coin);
    client.subscribe_ticker(symbol);

    let callback = |res: OptionPublicResponse| match res {
        OptionPublicResponse::Orderbook(res) => println!("Orderbook: {:?}", res),
        OptionPublicResponse::Trade(res) => println!("Trade: {:?}", res),
        OptionPublicResponse::Ticker(res) => println!("Ticker: {:?}", res),
        OptionPublicResponse::Pong(res) => println!("Pong: {:?}", res),
        OptionPublicResponse::Subscription(res) => println!("Subscription: {:?}", res),
    };

    if let Err(e) = client.run(callback) {
        eprintln!("Error: {e}")
    }
}
