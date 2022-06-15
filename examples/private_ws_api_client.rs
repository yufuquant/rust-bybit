use bybit::spot::ws::{PrivateResponse, PrivateWebSocketApiClient};
use std::env;

fn main() {
    env_logger::init();

    let api_key: String = env::var("BYBIT_API_KEY").unwrap();
    let secret: String = env::var("BYBIT_SECRET").unwrap();
    let client =
        PrivateWebSocketApiClient::new("wss://stream-testnet.bybit.com/spot/ws", &api_key, &secret);

    let callback = |res: PrivateResponse| match res {
        PrivateResponse::ExecutionReportSequence(seq) => println!("Excution report: {:?}", seq),
        PrivateResponse::TicketInfoSequence(seq) => println!("Ticket info: {:?}", seq),
        PrivateResponse::OutboundAccountInfoSequence(seq) => {
            println!("Outbound account info: {:?}", seq)
        }
        PrivateResponse::Pong(res) => println!("Pong: {:?}", res),
        PrivateResponse::Ping(res) => println!("Ping: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
