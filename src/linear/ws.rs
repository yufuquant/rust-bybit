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
    Liquidation(BaseResponse<'a, Liquidation<'a>>),
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

#[derive(Deserialize, Debug)]
pub struct Position<'a> {
    // UserID
    pub user_id: &'a str,
    // Symbol
    pub symbol: &'a str,
    // Position qty
    pub size: f64,
    // Side
    pub side: &'a str,
    // Position value
    pub position_value: &'a str,
    // Average entry price
    pub entry_price: &'a str,
    // Liquidation price
    pub liq_price: &'a str,
    // Bankruptcy price
    pub bust_price: &'a str,
    // In Isolated Margin mode, the value is set by the user. In Cross Margin mode, the value is the max leverage at current risk level
    pub leverage: &'a str,
    // Pre-occupied order margin
    pub order_margin: &'a str,
    // Position margin
    pub position_margin: &'a str,
    // Position closing fee occupied (your opening fee + expected maximum closing fee)
    pub occ_closing_fee: &'a str,
    // Take profit price
    pub take_profit: &'a str,
    // Take profit trigger price type, default: LastPrice
    pub tp_trigger_by: f64,
    // Stop loss price
    pub stop_loss: &'a str,
    // Stop loss trigger price
    pub sl_trigger_by: &'a str,
    // Today's realised pnl
    pub realised_pnl: &'a str,
    // Accumulated realised pnl (all-time total)
    pub cum_realised_pnl: &'a str,
    // Position status: Normal, Liq, Adl
    pub position_status: &'a str,
    // Position sequence
    pub position_seq: &'a str,
    // TrailingProfit or StopLoss mode Full or Partial
    pub tp_sl_mode: &'a str,
    // Position idx, used to identify positions in different position modes:
    // 0-One-Way Mode
    // 1-Buy side of both side mode
    // 2-Sell side of both side mode
    pub position_idx: i32,
    // Position mode, MergedSingle or BothSide
    pub mode: &'a str,
    // true means isolated margin mode; false means cross margin mode
    pub isolated: bool,
    // Risk ID
    pub risk_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct Execution<'a> {
    // Symbol
    pub symbol: &'a str,
    // Side
    pub side: &'a str,
    // Order ID
    pub order_id: &'a str,
    // Transaction ID
    pub exec_id: &'a str,
    // Unique user-set order ID. Maximum length of 36 characters
    pub order_link_id: &'a str,
    // Transaction price
    pub price: &'a str,
    // Order qty
    pub order_qty: f64,
    // Execution type (cannot be Funding)
    pub exec_type: &'a str,
    // Transaction qty
    pub exec_qty: f64,
    // Transaction fee
    pub exec_fee: &'a str,
    // Number of unfilled contracts from the order's size
    pub leaves_qty: f64,
    // Is maker
    pub is_maker: bool,
    // Transaction timestamp
    pub trade_time: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Order<'a> {
    // Order ID
    pub order_id: &'a str,
    // Unique user-set order ID. Maximum length of 36 characters
    pub order_link_id: &'a str,
    // Symbol
    pub symbol: &'a str,
    // Side
    pub side: &'a str,
    // Conditional order type
    pub order_type: &'a str,
    // Order price
    pub price: &'a str,
    // Transaction qty
    pub qty: f64,
    // Time in force
    pub time_in_force: &'a str,
    // Order status
    pub order_status: &'a str,
    // Last execution price
    pub last_exec_price: f64,
    // Cumulative qty of trading
    pub cum_exec_qty: f64,
    // Cumulative value of trading
    pub cum_exec_value: &'a str,
    // True means your position can only reduce in size if this order is triggered
    pub reduce_only: bool,
    // For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
    pub close_on_trigger: bool,
    // Position idx, used to identify positions in different position modes:
    // 0-One-Way Mode
    // 1-Buy side of both side mode
    // 2-Sell side of both side mode
    pub position_idx: i32,
    // Timestamp (when the order_status was New)
    pub create_time: &'a str,
    // Update time
    pub update_time: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct StopOrder<'a> {
    // Conditional order ID. Once triggered, the conditional order creates an active order with the same ID (order_id)
    pub stop_order_id: &'a str,
    // Unique user-set order ID. Maximum length of 36 characters
    pub order_link_id: &'a str,
    // UserID
    pub user_id: &'a str,
    // Symbol
    pub symbol: &'a str,
    // Side
    pub side: &'a str,
    // Conditional order type
    pub order_type: &'a str,
    // Order price
    pub price: &'a str,
    // Transaction qty
    pub qty: f64,
    // Time in force
    pub time_in_force: &'a str,
    // Order status
    pub order_status: &'a str,
    // Conditional order type
    pub stop_order_type: &'a str,
    // Trigger price type. Default LastPrice
    pub trigger_by: &'a str,
    // If stop_order_type is TrailingProfit, this field is the trailing stop active price.
    pub trigger_price: &'a str,
    // True means your position can only reduce in size if this order is triggered
    pub reduce_only: bool,
    // For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
    pub close_on_trigger: bool,
    // Position idx, used to identify positions in different position modes:
    // 0-One-Way Mode
    // 1-Buy side of both side mode
    // 2-Sell side of both side mode
    pub position_idx: i32,
    // Creation time (when the order_status was Created)
    pub create_at: &'a str,
    // Update time
    pub updated_at: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Wallet {
    pub wallet_balance: f64,
    pub available_balance: f64,
}

#[derive(Deserialize, Debug)]
pub struct BasePrivateResponse<'a, D> {
    pub topic: &'a str,
    pub data: Vec<D>,
}

#[derive(Deserialize, Debug)]
pub struct BasePrivateResponseWithAction<'a, D> {
    pub topic: &'a str,
    pub action: &'a str,
    pub data: D,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PrivateResponse<'a> {
    #[serde(borrow)]
    Position(BasePrivateResponseWithAction<'a, Position<'a>>),
    Execution(BasePrivateResponse<'a, Execution<'a>>),
    Order(BasePrivateResponseWithAction<'a, Order<'a>>),
    StopOrder(BasePrivateResponse<'a, StopOrder<'a>>),
    Wallet(BasePrivateResponse<'a, Wallet>),
}

pub struct PrivateWebSocketApiClient {
    pub uri: String,
    pub api_key: String,
    pub secret: String,
    subscriptions: Vec<String>,
}

impl PrivateWebSocketApiClient {
    pub fn new(uri: &str, api_key: &str, secret: &str) -> Self {
        Self {
            uri: uri.to_string(),
            api_key: api_key.to_string(),
            secret: secret.to_string(),
            subscriptions: Vec::new(),
        }
    }

    pub fn subscribe_position(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["position".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_execution(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["execution".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_order(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["order".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_wallet(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["wallet".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_stop_order(&mut self) {
        let subscription = Subscription {
            op: "subscribe".into(),
            args: vec!["stop_order".into()],
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn run<Callback: FnMut(PrivateResponse)>(&self, mut callback: Callback) -> Result<()> {
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
                        match serde_json::from_str::<PrivateResponse>(&content) {
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
}

fn spawn_ping_thread(tx: Sender<String>) {
    thread::spawn(move || loop {
        let s30 = Duration::from_secs(30);
        tx.send("{\"op\":\"ping\"}".into()).unwrap();
        thread::sleep(s30);
    });
}
