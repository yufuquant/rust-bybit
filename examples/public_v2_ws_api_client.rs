use bybit::spot::ws::{PublicV2Response, PublicV2WebSocketApiClient};

fn main() {
    env_logger::init();

    let mut client = PublicV2WebSocketApiClient::new();

    client.subscribe_depth("BTCUSDT", false);
    client.subscribe_kline("BTCUSDT", false, "1m");
    client.subscribe_trade("BTCUSDT", false);
    client.subscribe_book_ticker("BTCUSDT", false);
    client.subscribe_realtimes("BTCUSDT", false);

    let callback = |res: PublicV2Response| match res {
        PublicV2Response::Depth(res) => println!("Depth: {:?}", res),
        PublicV2Response::Kline(res) => println!("Kline: {:?}", res),
        PublicV2Response::Trade(res) => println!("Trade: {:?}", res),
        PublicV2Response::BookTicker(res) => println!("BookTicker: {:?}", res),
        PublicV2Response::Realtimes(res) => println!("Realtimes: {:?}", res),
        PublicV2Response::Pong(res) => println!("Pong: {:?}", res),
        PublicV2Response::Ping(res) => println!("Ping: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
