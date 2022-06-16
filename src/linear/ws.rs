use crate::error::Result;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;
use tungstenite::{connect, Message};
use url::Url;

#[derive(Deserialize, Debug)]
pub struct BaseResponse<'a, D> {
    pub topic: &'a str,
    pub data: D,
}

#[derive(Deserialize, Debug)]
pub struct BaseResponseWithTimestamp<'a, D> {
    pub topic: &'a str,
    pub data: D,
    pub timestamp_e6: u64,
}

#[derive(Deserialize, Debug)]
pub struct Response<'a, D> {
    pub topic: &'a str,
    #[serde(alias = "type")]
    pub res_type: &'a str,
    pub data: D,
    pub cross_seq: &'a str,
    pub timestamp_e6: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookItem<'a> {
    pub price: &'a str,
    pub symbol: &'a str,
    pub id: &'a str,
    pub side: &'a str,
    pub size: f64,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookDeleteItem<'a> {
    pub price: &'a str,
    pub symbol: &'a str,
    pub id: &'a str,
    pub side: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookSnapshot<'a> {
    #[serde(borrow)]
    pub order_book: Vec<OrderBookItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookDelta<'a> {
    #[serde(borrow)]
    pub delete: Vec<OrderBookDeleteItem<'a>>,
    pub update: Vec<OrderBookItem<'a>>,
    pub insert: Vec<OrderBookItem<'a>>,
    // #[serde(rename = "transactTimeE6")]
    // pub transact_time_e6: u64,
}

#[derive(Deserialize, Debug)]
pub struct Trade<'a> {
    // Symbol
    pub symbol: &'a str,
    // Direction of price change
    pub tick_direction: &'a str,
    // Order price
    pub price: &'a str,
    // Position qty
    pub size: f64,
    // UTC time
    pub timestamp: &'a str,
    // Millisecond timestamp
    pub trade_time_ms: &'a str,
    // Direction of taker
    pub side: &'a str,
    // Trade ID
    pub trade_id: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Kline<'a> {
    // Start timestamp point for result, in seconds
    pub start: u64,
    // End timestamp point for result, in seconds
    pub end: u64,
    // Starting price
    pub open: f64,
    // Closing price
    pub close: f64,
    // Maximum price
    pub high: f64,
    // Minimum price
    pub low: f64,
    // Trading volume
    pub volume: &'a str,
    // Turnover
    pub turnover: &'a str,
    // Is confirm
    pub confirm: bool,
    // Cross sequence (internal value)
    pub cross_seq: u64,
    // End timestamp point for result, in seconds
    pub timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct Liquidation<'a> {
    // Symbol
    pub symbol: &'a str,
    // Liquidated position's side
    pub side: &'a str,
    // Bankruptcy price
    pub price: &'a str,
    // Order quantity
    pub qty: &'a str,
    // Millisecond timestamp
    pub time: u64,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PublicResponse<'a> {
    #[serde(borrow)]
    OrderBookL2Snapshot(Response<'a, OrderBookSnapshot<'a>>),
    OrderBookL2Delta(Response<'a, OrderBookDelta<'a>>),
    Trade(BaseResponse<'a, Vec<Trade<'a>>>),
    // InstrumentInfo
    Kline(BaseResponseWithTimestamp<'a, Vec<Kline<'a>>>),
    Liquidation(BaseResponse<'a, Vec<Liquidation<'a>>>),
}

#[derive(Serialize, Debug)]
struct Subscription {
    op: String,
    args: Vec<String>,
}

pub struct PublicWebSocketApiClient {
    pub uri: String,
    subscriptions: Vec<String>,
}

impl PublicWebSocketApiClient {
    pub fn new(uri: &str) -> Self {
        return PublicWebSocketApiClient {
            uri: uri.to_string(),
            subscriptions: Vec::new(),
        };
    }

    pub fn subscribe_order_book_l2_25(&mut self, symbols: &Vec<String>) {
        self.subscribe("orderBookL2_25", symbols);
    }

    pub fn subscribe_order_book_l2_200(&mut self, symbols: &Vec<String>) {
        self.subscribe("orderBook_200.100ms", symbols);
    }

    pub fn subscribe_trade(&mut self, symbols: &Vec<String>) {
        self.subscribe("trade", symbols);
    }

    pub fn subscribe_kline(&mut self, symbols: &Vec<String>, interval: &str) {
        let topic = format!("candle.{}", interval);
        self.subscribe(&topic, symbols);
    }

    pub fn subscribe_liquidation(&mut self, symbols: &Vec<String>) {
        self.subscribe("liquidation", symbols);
    }

    pub fn run<Callback: Fn(PublicResponse)>(&self, callback: Callback) -> Result<()> {
        let req = Url::parse(&self.uri).unwrap();
        let (mut ws, _) = connect(req).expect("Can't connect");

        let (tx, rx) = mpsc::channel::<String>();
        spawn_ping_thread(tx);

        for subscription in &self.subscriptions {
            ws.write_message(Message::Text(subscription.clone()))
                .unwrap();
        }

        loop {
            if let Ok(ping) = rx.try_recv() {
                ws.write_message(Message::Text(ping)).unwrap();
            }

            if let Ok(msg) = ws.read_message() {
                match msg {
                    Message::Text(content) => {
                        debug!("Received: {}", content);
                        match serde_json::from_str::<PublicResponse>(&content) {
                            Ok(res) => callback(res),
                            Err(e) => error!("Error: {}", e),
                        }
                    }
                    Message::Binary(_) => {}
                    Message::Ping(_) => {}
                    Message::Pong(_) => {}
                    Message::Close(_) => break,
                }
            }
        }
        Ok(())
    }

    fn subscribe(&mut self, topic: &str, symbols: &Vec<String>) {
        let args = symbols
            .iter()
            .map(|symbol| format!("{}.{}", topic, symbol))
            .collect();
        let subscription = Subscription {
            op: "subscribe".into(),
            args,
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }
}

fn spawn_ping_thread(tx: Sender<String>) {
    thread::spawn(move || loop {
        let s30 = Duration::from_secs(30);
        tx.send("{\"op\":\"ping\"}".into()).unwrap();
        thread::sleep(s30);
    });
}
