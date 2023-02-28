use bybit::ws::response::PrivateResponse;
use bybit::WebSocketApiClient;
use std::env;

fn main() {
    env_logger::init();

    let api_key: String = env::var("BYBIT_API_KEY").unwrap();
    let secret: String = env::var("BYBIT_SECRET").unwrap();

    let mut client = WebSocketApiClient::private()
        .testnet()
        .build_with_credentials(api_key, secret);

    client.subscribe_position();
    client.subscribe_execution();
    client.subscribe_order();
    client.subscribe_wallet();
    client.subscribe_greek();

    if let Err(e) = client.run(|res| match res {
        PrivateResponse::Position(res) => println!("Position: {:?}", res),
        PrivateResponse::Execution(res) => println!("Execution: {:?}", res),
        PrivateResponse::Order(res) => println!("Order: {:?}", res),
        PrivateResponse::Wallet(res) => println!("Wallet: {:?}", res),
        PrivateResponse::Greek(res) => println!("Greek: {:?}", res),
        PrivateResponse::Pong(res) => println!("Pong: {:?}", res),
        PrivateResponse::Op(res) => println!("Op: {:?}", res),
    }) {
        eprintln!("Error: {e}")
    }
}
