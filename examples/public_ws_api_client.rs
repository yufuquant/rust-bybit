use bybit::spot::ws::{PublicResponse, PublicWebSocketApiClient};

fn main() {
    env_logger::init();

    let mut client = PublicWebSocketApiClient::new("wss://stream.bybit.com/spot/quote/ws/v1");

    client.subscribe_trade("BTCUSDT", false);
    client.subscribe_realtimes("BTCUSDT", false);
    client.subscribe_kline("BTCUSDT", "1m", false);
    client.subscribe_depth("BTCUSDT", false);
    client.subscribe_merged_depth("BTCUSDT", false, 1);
    client.subscribe_diff_depth("BTCUSDT", false);
    client.subscribe_lt("BTC3LUSDTNAV", false);

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
}
