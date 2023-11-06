mod callback;
pub mod future;
pub mod option;
pub mod private;
pub mod response;
pub mod spot;

use callback::Arg;
use callback::Callback;
use log::*;
use serde::Serialize;
use std::cmp;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::{sync::mpsc, thread, time::Duration};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

use crate::error::Result;
use crate::util::millis;
use crate::util::sign;
use crate::FutureRole;

use self::future::FutureWebSocketApiClientBuilder;
use self::option::OptionWebSocketApiClientBuilder;
use self::private::PrivateWebSocketApiClientBuilder;
use self::spot::SpotWebSocketApiClientBuilder;

/// A factory to create different kind of websocket api clients (spot / future / option / private).
pub struct WebSocketApiClient;

impl WebSocketApiClient {
    /// Get a builder for building spot websocket api client.
    pub fn spot() -> SpotWebSocketApiClientBuilder {
        SpotWebSocketApiClientBuilder::new()
    }

    /// Get a builder for building inverse future websocket api client.
    pub fn future_inverse() -> FutureWebSocketApiClientBuilder {
        FutureWebSocketApiClientBuilder::new(FutureRole::Inverse)
    }

    /// Get a builder for building linear future websocket api client.
    pub fn future_linear() -> FutureWebSocketApiClientBuilder {
        FutureWebSocketApiClientBuilder::new(FutureRole::Linear)
    }

    /// Get a builder for building option websocket api client.
    pub fn option() -> OptionWebSocketApiClientBuilder {
        OptionWebSocketApiClientBuilder::new()
    }

    /// Get a builder for building private websocket api client.
    pub fn private() -> PrivateWebSocketApiClientBuilder {
        PrivateWebSocketApiClientBuilder::new()
    }
}

struct Subscriber {
    topics: Vec<String>,
}

impl Subscriber {
    fn new() -> Self {
        Self { topics: Vec::new() }
    }

    fn topics(&self) -> &Vec<String> {
        &self.topics
    }

    fn sub_orderbook(&mut self, symbol: &str, depth: u16) {
        self.sub(format!("orderbook.{depth}.{symbol}"));
    }

    fn sub_ticker(&mut self, symbol: &str) {
        self.sub(format!("tickers.{symbol}"));
    }

    fn sub_trade(&mut self, symbol: &str) {
        self.sub(format!("publicTrade.{symbol}"));
    }

    fn sub_kline(&mut self, symbol: &str, interval: &str) {
        self.sub(format!("kline.{interval}.{symbol}"));
    }

    fn sub_liquidation(&mut self, symbol: &str) {
        self.sub(format!("liquidation.{symbol}"));
    }

    fn sub_lt_kline(&mut self, symbol: &str, interval: &str) {
        self.sub(format!("kline_lt.{interval}.{symbol}"));
    }

    fn sub_lt_ticker(&mut self, symbol: &str) {
        self.sub(format!("tickers_lt.{symbol}"));
    }

    fn sub_lt_nav(&mut self, symbol: &str) {
        self.sub(format!("lt.{symbol}"));
    }

    fn sub_position(&mut self) {
        self.sub("position".to_string());
    }

    fn sub_execution(&mut self) {
        self.sub("execution".to_string());
    }

    fn sub_order(&mut self) {
        self.sub("order".to_string());
    }

    fn sub_wallet(&mut self) {
        self.sub("wallet".to_string());
    }

    fn sub_greek(&mut self) {
        self.sub("greeks".to_string());
    }

    fn sub(&mut self, topic: String) {
        self.topics.push(topic);
    }
}

#[derive(Serialize)]
struct Op<'a> {
    op: &'a str,
    args: Vec<String>,
}

struct Credentials {
    api_key: String,
    secret: String,
}

fn run<A, C>(
    uri: &str,
    topics: &Vec<String>,
    credentials: Option<&Credentials>,
    mut callback: C,
) -> Result<()>
where
    A: Arg,
    C: Callback<A>,
{
    let (mut ws, _) = connect(uri)?;

    // Set read timeout to the underlying TCP stream.
    //
    // Read and write are both in the main thread loop. A blocking read call
    // will starve writing that causes ping op message can't be sent on time.
    // Read timeout mitigate this situation.
    set_read_timeout(&ws);

    // Authenticate
    if let Some(credentials) = credentials {
        let req = auth_req(credentials);
        ws.write_message(Message::Text(req))?;
    }

    // Subscribe
    const SUBSCRIBE_BATCH_SIZE: usize = 10;
    for i in (0..topics.len()).step_by(SUBSCRIBE_BATCH_SIZE) {
        let after_last = cmp::min(i + SUBSCRIBE_BATCH_SIZE, topics.len());
        let batch = &topics[i..after_last];
        ws.write_message(Message::Text(subscription(batch)))?;
    }

    let rx = ping();
    loop {
        // Ping
        if let Ok(ping) = rx.try_recv() {
            ws.write_message(Message::Text(ping.into()))?
        }

        match ws.read_message() {
            Ok(msg) => match msg {
                Message::Text(content) => {
                    debug!("Received: {}", content);
                    match serde_json::from_str(&content) {
                        Ok(res) => callback(res),
                        Err(e) => error!("Error: {}", e),
                    }
                }
                _ => {}
            },
            Err(e) => match e {
                tungstenite::Error::Io(ref ee) => {
                    if ee.kind() != std::io::ErrorKind::WouldBlock
                        && ee.kind() != std::io::ErrorKind::TimedOut
                    {
                        Err(e)?
                    }
                }
                _ => Err(e)?,
            },
        }
    }
}

fn set_read_timeout(ws: &WebSocket<MaybeTlsStream<TcpStream>>) {
    match ws.get_ref() {
        MaybeTlsStream::Plain(s) => {
            s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        }
        MaybeTlsStream::NativeTls(t) => {
            t.get_ref()
                .set_read_timeout(Some(Duration::from_secs(10)))
                .unwrap();
        }
        _ => unreachable!(),
    };
}

fn auth_req(credentials: &Credentials) -> String {
    let expires = millis() + 10000;
    let val = format!("GET/realtime{}", expires);
    let signature = sign(&credentials.secret, &val);
    let auth_req = Op {
        op: "auth",
        args: vec![credentials.api_key.clone(), expires.to_string(), signature],
    };
    serde_json::to_string(&auth_req).unwrap()
}

fn subscription(topics: &[String]) -> String {
    let sub = Op {
        op: "subscribe",
        args: topics.to_vec(),
    };
    serde_json::to_string(&sub).unwrap()
}

fn ping() -> Receiver<&'static str> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        if let Err(_) = tx.send("{\"op\":\"ping\"}") {
            break;
        };
        thread::sleep(Duration::from_secs(20));
    });
    rx
}
