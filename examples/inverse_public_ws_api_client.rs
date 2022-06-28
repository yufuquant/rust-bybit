use bybit::inverse::{PublicResponse, PublicWebSocketApiClient};

fn main() {
    env_logger::init();

    let mut client = PublicWebSocketApiClient::new("wss://stream.bybit.com/realtime");

    let symbols = vec!["BTCUSD".to_owned(), "ETHUSDU22".to_owned()];
    client.subscribe_order_book_l2_25(&symbols);
    client.subscribe_order_book_l2_200(&symbols);
    client.subscribe_trade(&symbols);
    client.subscribe_insurance(&symbols);
    client.subscribe_instrument_info(&symbols);
    client.subscribe_kline(&symbols, "1");
    client.subscribe_liquidation(&symbols);

    let callback = |res: PublicResponse| match res {
        PublicResponse::OrderBookL2Snapshot(res) => println!("Order book L2 snapshot: {:?}", res),
        PublicResponse::OrderBookL2Delta(res) => println!("Order book L2 delta: {:?}", res),
        PublicResponse::Trade(res) => println!("Trade: {:?}", res),
        PublicResponse::Insurance(res) => println!("Insurance: {:?}", res),
        PublicResponse::PerpetualInstrumentInfoSnapshot(res) => {
            println!("Perpetual instrument info snapshot: {:?}", res)
        }
        PublicResponse::PerpetualInstrumentInfoDelta(res) => {
            println!("Perpetual instrument info delta: {:?}", res)
        }
        PublicResponse::FuturesInstrumentInfoSnapshot(res) => {
            println!("Futures instrument info snapshot: {:?}", res)
        }
        PublicResponse::FuturesInstrumentInfoDelta(res) => {
            println!("Futures instrument info delta: {:?}", res)
        }
        PublicResponse::Kline(res) => println!("Kline: {:?}", res),
        PublicResponse::Liquidation(res) => println!("Liquidation: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
