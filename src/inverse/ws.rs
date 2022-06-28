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
pub struct PingRequest<'a> {
    op: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Pong<'a> {
    pub success: bool,
    pub ret_msg: &'a str,
    pub conn_id: &'a str,
    pub request: PingRequest<'a>,
}

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
    pub cross_seq: u64,
    pub timestamp_e6: u64,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookItem<'a> {
    pub price: &'a str,
    pub symbol: &'a str,
    pub id: u64,
    pub side: &'a str,
    pub size: u64,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookDeleteItem<'a> {
    pub price: &'a str,
    pub symbol: &'a str,
    pub id: u64,
    pub side: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookDelta<'a> {
    #[serde(borrow)]
    pub delete: Vec<OrderBookDeleteItem<'a>>,
    pub update: Vec<OrderBookItem<'a>>,
    pub insert: Vec<OrderBookItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Trade<'a> {
    // Symbol
    pub symbol: &'a str,
    // Direction of price change
    pub tick_direction: &'a str,
    // Order price
    pub price: f64,
    // Position qty
    pub size: u64,
    // UTC time
    pub timestamp: &'a str,
    // Millisecond timestamp
    pub trade_time_ms: u64,
    // Direction of taker
    pub side: &'a str,
    // Trade ID
    pub trade_id: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Insurance<'a> {
    // Symbol
    pub currency: &'a str,
    // UTC time
    pub timestamp: &'a str,
    // Wallet balance
    pub wallet_balance: i64,
}

#[derive(Deserialize, Debug)]
pub struct PerpetualInstrumentInfoSnapshot<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    pub last_price_e4: u64,
    // Latest transaction price
    pub last_price: &'a str,
    pub bid1_price_e4: u64,
    // Best bid price
    pub bid1_price: &'a str,
    pub ask1_price_e4: u64,
    // Best ask price
    pub ask1_price: &'a str,
    // Direction of price change
    pub last_tick_direction: &'a str,
    pub prev_price_24h_e4: u64,
    // Price of 24 hours ago
    pub prev_price_24h: &'a str,
    // Percentage change of market price relative to 24h * 10^4
    pub price_24h_pcnt_e6: i64,
    pub high_price_24h_e4: u64,
    // The highest price in the last 24 hours
    pub high_price_24h: &'a str,
    pub low_price_24h_e4: u64,
    // Lowest price in the last 24 hours
    pub low_price_24h: &'a str,
    pub prev_price_1h_e4: u64,
    // Hourly market price an hour ago
    pub prev_price_1h: &'a str,
    pub price_1h_pcnt_e6: i64,
    pub mark_price_e4: u64,
    // Mark price
    pub mark_price: &'a str,
    pub index_price_e4: u64,
    // Index_price
    pub index_price: &'a str,
    // Open interest. The update is not immediate - slowest update is 1 minute
    pub open_interest: u64,
    pub open_value_e8: u64,
    // Total turnover
    pub total_turnover_e8: u64,
    // Turnover for 24h * 10^8
    pub turnover_24h_e8: u64,
    // Total volume * 10^8
    pub total_volume: u64,
    // Trading volume in the last 24 hours
    pub volume_24h: u64,
    // Funding rate * 10^6
    pub funding_rate_e6: i32,
    // Predicted funding rate * 10^6
    pub predicted_funding_rate_e6: i32,
    // Cross sequence (internal value)
    pub cross_seq: u64,
    // Creation time (when the order_status was Created)
    pub created_at: &'a str,
    // Update time
    pub updated_at: &'a str,
    // Next settlement time of capital cost
    pub next_funding_time: &'a str,
    // Countdown of settlement capital cost
    pub countdown_hour: u32,
    // funding rate time interval, unit hour
    pub funding_rate_interval: u32,
    pub settle_time_e9: u64,
    pub delisting_status: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct PerpetualInstrumentInfoDeltaItem<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    pub last_tick_direction: Option<&'a str>,
    pub last_price_e4: Option<u64>,
    pub last_price: Option<&'a str>,
    pub bid1_price_e4: Option<u64>,
    pub bid1_price: Option<&'a str>,
    pub ask1_price_e4: Option<u64>,
    pub ask1_price: Option<&'a str>,
    pub price_24h_pcnt_e6: Option<i64>,
    pub price_1h_pcnt_e6: Option<i64>,
    pub mark_price_e4: Option<u64>,
    pub mark_price: Option<&'a str>,
    pub total_turnover_e8: Option<u64>,
    pub turnover_24h_e8: Option<u64>,
    pub total_volume: Option<u64>,
    pub volume_24h: Option<u64>,
    pub cross_seq: u64,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct PerpetualInstrumentInfoDelta<'a> {
    #[serde(borrow)]
    pub delete: Vec<PerpetualInstrumentInfoDeltaItem<'a>>,
    pub update: Vec<PerpetualInstrumentInfoDeltaItem<'a>>,
    pub insert: Vec<PerpetualInstrumentInfoDeltaItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct FuturesInstrumentInfoSnapshot<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    // Symbol name alias
    pub symbol_name: &'a str,
    // The year of symbol
    pub symbol_year: u32,
    // Contract type
    pub contract_type: &'a str,
    // Coin type
    pub coin: &'a str,
    // Quote symbol
    pub quote_symbol: &'a str,
    // Position Mode. 0: One-Way Mode; 3: Hedge Mode
    pub mode: &'a str,
    // Support float profit open position or not
    pub is_up_borrowable: i32,
    // Symbol import timestamp * 10^9
    pub import_time_e9: u64,
    // Enable trading timestamp for symbol * 10^9
    pub start_trading_time_e9: u64,
    // Rest time until settled in seconds
    pub time_to_settle: u64,
    // Delivery timestamp * 10^9
    pub settle_time_e9: u64,
    // Delivery fee rate * 10^8
    pub settle_fee_rate_e8: i32,
    // Contract status
    pub contract_status: &'a str,
    // Quantity of subsidy from trading platform in BTC * 10^8
    pub system_subsidy_e8: u64,
    // Latest transaction price
    pub last_price_e4: u64,
    // Latest transaction price
    pub last_price: &'a str,
    // Direction of price change
    pub last_tick_direction: &'a str,
    pub bid1_price_e4: u64,
    // Best bid price
    pub bid1_price: &'a str,
    pub ask1_price_e4: u64,
    // Best ask price
    pub ask1_price: &'a str,
    pub prev_price_24h_e4: u64,
    // Price of 24 hours ago
    pub prev_price_24h: &'a str,
    // Percentage change of market price relative to 24h * 10^4
    pub price_24h_pcnt_e6: i64,
    pub high_price_24h_e4: u64,
    // The highest price in the last 24 hours
    pub high_price_24h: &'a str,
    pub low_price_24h_e4: u64,
    // Lowest price in the last 24 hours
    pub low_price_24h: &'a str,
    pub prev_price_1h_e4: u64,
    // Hourly market price an hour ago
    pub prev_price_1h: &'a str,
    pub price_1h_pcnt_e6: i64,
    pub mark_price_e4: u64,
    // Mark price
    pub mark_price: &'a str,
    pub index_price_e4: u64,
    // Index_price
    pub index_price: &'a str,
    // Open interest. The update is not immediate - slowest update is 1 minute
    pub open_interest: u64,
    pub open_value_e8: u64,
    // Total turnover
    pub total_turnover_e8: u64,
    // Turnover for 24h * 10^8
    pub turnover_24h_e8: u64,
    pub fair_basis_e8: u64,
    pub fair_basis_rate_e8: u64,
    pub basis_in_year_e8: u64,
    pub expect_price_e4: u64,
    // Total volume * 10^8
    pub total_volume: u64,
    // Trading volume in the last 24 hours
    pub volume_24h: u64,
    // Cross sequence (internal value)
    pub cross_seq: u64,
    // Creation time (when the order_status was Created)
    pub created_at_e9: u64,
    // Update time
    pub updated_at_e9: u64,
}

#[derive(Deserialize, Debug)]
pub struct FuturesInstrumentInfoDeltaItem<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    pub symbol_name: &'a str,
    pub symbol_year: u32,
    pub contract_type: &'a str,
    pub coin: &'a str,
    pub quote_symbol: &'a str,
    pub mode: &'a str,
    pub start_trading_time_e9: u64,
    pub time_to_settle: u64,
    pub settle_time_e9: u64,

    pub is_up_borrowable: Option<i32>,
    pub settle_fee_rate_e8: Option<i32>,
    pub import_time_e9: Option<u64>,
    pub contract_status: Option<&'a str>,
    pub system_subsidy_e8: Option<u64>,
    pub last_price_e4: Option<u64>,
    // Latest transaction price
    pub last_price: Option<&'a str>,
    // Direction of price change
    pub last_tick_direction: Option<&'a str>,
    pub bid1_price_e4: Option<u64>,
    // Best bid price
    pub bid1_price: Option<&'a str>,
    pub ask1_price_e4: Option<u64>,
    // Best ask price
    pub ask1_price: Option<&'a str>,
    pub prev_price_24h_e4: Option<u64>,
    // Price of 24 hours ago
    pub prev_price_24h: Option<&'a str>,
    // Percentage change of market price relative to 24h * 10^4
    pub price_24h_pcnt_e6: Option<i64>,
    pub high_price_24h_e4: Option<u64>,
    // The highest price in the last 24 hours
    pub high_price_24h: Option<&'a str>,
    pub low_price_24h_e4: Option<u64>,
    // Lowest price in the last 24 hours
    pub low_price_24h: Option<&'a str>,
    pub prev_price_1h_e4: Option<u64>,
    // Hourly market price an hour ago
    pub prev_price_1h: Option<&'a str>,
    pub price_1h_pcnt_e6: Option<i64>,
    pub mark_price_e4: Option<u64>,
    // Mark price
    pub mark_price: Option<&'a str>,
    pub index_price_e4: Option<u64>,
    // Index_price
    pub index_price: Option<&'a str>,
    // Open interest. The update is not immediate - slowest update is 1 minute
    pub open_interest: Option<u64>,
    pub open_value_e8: Option<u64>,
    // Total turnover
    pub total_turnover_e8: Option<u64>,
    // Turnover for 24h * 10^8
    pub turnover_24h_e8: Option<u64>,
    pub fair_basis_e8: Option<u64>,
    pub fair_basis_rate_e8: Option<u64>,
    pub basis_in_year_e8: Option<u64>,
    pub expect_price_e4: Option<u64>,
    // Total volume * 10^8
    pub total_volume: Option<u64>,
    // Trading volume in the last 24 hours
    pub volume_24h: Option<u64>,
    // Cross sequence (internal value)
    pub cross_seq: Option<u64>,
    // Creation time (when the order_status was Created)
    pub created_at_e9: Option<u64>,
    // Update time
    pub updated_at_e9: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct FuturesInstrumentInfoDelta<'a> {
    #[serde(borrow)]
    pub update: Vec<FuturesInstrumentInfoDeltaItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct Kline {
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
    pub volume: f64,
    // Turnover
    pub turnover: f64,
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
    OrderBookL2Snapshot(BaseResponseWithTimestamp<'a, Vec<OrderBookItem<'a>>>),
    OrderBookL2Delta(BaseResponseWithTimestamp<'a, OrderBookDelta<'a>>),
    Trade(BaseResponse<'a, Vec<Trade<'a>>>),
    Insurance(BaseResponse<'a, Vec<Insurance<'a>>>),
    PerpetualInstrumentInfoSnapshot(Response<'a, PerpetualInstrumentInfoSnapshot<'a>>),
    PerpetualInstrumentInfoDelta(Response<'a, PerpetualInstrumentInfoDelta<'a>>),
    FuturesInstrumentInfoSnapshot(Response<'a, FuturesInstrumentInfoSnapshot<'a>>),
    FuturesInstrumentInfoDelta(Response<'a, FuturesInstrumentInfoDelta<'a>>),
    Kline(BaseResponseWithTimestamp<'a, Vec<Kline>>),
    Liquidation(BaseResponse<'a, Liquidation<'a>>),
}

pub struct PublicWebSocketApiClient {
    pub uri: String,
    subscriptions: Vec<String>,
}

#[derive(Serialize, Debug)]
struct Subscription {
    op: String,
    args: Vec<String>,
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

    pub fn subscribe_insurance(&mut self, symbols: &Vec<String>) {
        self.subscribe("insurance", symbols);
    }

    pub fn subscribe_instrument_info(&mut self, symbols: &Vec<String>) {
        self.subscribe("instrument_info.100ms", symbols);
    }

    pub fn subscribe_kline(&mut self, symbols: &Vec<String>, interval: &str) {
        let topic = format!("klineV2.{}", interval);
        self.subscribe(&topic, symbols);
    }

    pub fn subscribe_liquidation(&mut self, symbols: &Vec<String>) {
        self.subscribe("liquidation", symbols);
    }

    pub fn run<Callback: FnMut(PublicResponse)>(&self, mut callback: Callback) -> Result<()> {
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
                        if let Ok(_) = serde_json::from_str::<Pong>(&content) {
                            continue;
                        }
                        match serde_json::from_str::<PublicResponse>(&content) {
                            Ok(res) => callback(res),
                            Err(e) => error!("Error: {:?} {}", content, e),
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
