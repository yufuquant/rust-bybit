use bybit::inverse::{PrivateResponse, PrivateWebSocketApiClient};
use std::env;

fn main() {
    env_logger::init();

    let api_key: String = env::var("BYBIT_API_KEY").unwrap();
    let secret: String = env::var("BYBIT_SECRET").unwrap();
    let mut client =
        PrivateWebSocketApiClient::new("wss://stream.bybit.com/realtime", &api_key, &secret);

    client.subscribe_position();
    client.subscribe_execution();
    client.subscribe_order();
    client.subscribe_stop_order();
    client.subscribe_wallet();

    let callback = |res: PrivateResponse| match res {
        PrivateResponse::Position(res) => println!("Position: {:?}", res),
        PrivateResponse::Execution(res) => println!("Execution: {:?}", res),
        PrivateResponse::Order(res) => println!("Order: {:?}", res),
        PrivateResponse::StopOrder(res) => println!("Stop Order: {:?}", res),
        PrivateResponse::Wallet(res) => println!("Wallet: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
