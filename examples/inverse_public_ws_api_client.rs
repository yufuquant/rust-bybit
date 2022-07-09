use bybit::inverse::{PublicResponse, PublicWebSocketApiClient};

fn main() {
    env_logger::init();

    let mut client = PublicWebSocketApiClient::new();

    let symbols = vec!["BTCUSD", "ETHUSDU22"];
    client.subscribe_order_book_l2_25(&symbols);
    client.subscribe_order_book_l2_200(&symbols);
    client.subscribe_trade(&symbols);
    client.subscribe_insurance(&symbols);
    client.subscribe_instrument_info(&symbols);
    client.subscribe_kline(&symbols, "1");
    client.subscribe_liquidation(&symbols);

    let callback = |res: PublicResponse| match res {
        PublicResponse::OrderBookL2SnapshotMessage(res) => {
            println!("Order book L2 snapshot: {:?}", res)
        }
        PublicResponse::OrderBookL2DeltaMessage(res) => println!("Order book L2 delta: {:?}", res),
        PublicResponse::TradeMessage(res) => println!("Trade: {:?}", res),
        PublicResponse::InsuranceMessage(res) => println!("Insurance: {:?}", res),
        PublicResponse::PerpetualInstrumentInfoSnapshotMessage(res) => {
            println!("Perpetual instrument info snapshot: {:?}", res)
        }
        PublicResponse::PerpetualInstrumentInfoDeltaMessage(res) => {
            println!("Perpetual instrument info delta: {:?}", res)
        }
        PublicResponse::FuturesInstrumentInfoSnapshotMessage(res) => {
            println!("Futures instrument info snapshot: {:?}", res)
        }
        PublicResponse::FuturesInstrumentInfoDeltaMessage(res) => {
            println!("Futures instrument info delta: {:?}", res)
        }
        PublicResponse::KlineMessage(res) => println!("Kline: {:?}", res),
        PublicResponse::LiquidationMessage(res) => println!("Liquidation: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
