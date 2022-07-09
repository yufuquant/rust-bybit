use bybit::inverse::{PrivateResponse, PrivateWebSocketApiClient};
use std::env;

fn main() {
    env_logger::init();

    let api_key: String = env::var("BYBIT_API_KEY").unwrap();
    let secret: String = env::var("BYBIT_SECRET").unwrap();
    let mut client = PrivateWebSocketApiClient::new(api_key, secret);

    client.subscribe_position();
    client.subscribe_execution();
    client.subscribe_order();
    client.subscribe_stop_order();
    client.subscribe_wallet();

    let callback = |res: PrivateResponse| match res {
        PrivateResponse::PositionMessage(res) => println!("Position: {:?}", res),
        PrivateResponse::ExecutionMessage(res) => println!("Execution: {:?}", res),
        PrivateResponse::OrderMessage(res) => println!("Order: {:?}", res),
        PrivateResponse::StopOrderMessage(res) => println!("Stop Order: {:?}", res),
        PrivateResponse::WalletMessage(res) => println!("Wallet: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
