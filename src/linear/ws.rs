use crate::error::Result;
use crate::util::*;
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
pub struct InstrumentInfoSnapshot<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    // (Deprecated) Latest transaction price 10^4
    pub last_price_e4: &'a str,
    // Latest transaction price
    pub last_price: &'a str,
    // (Deprecated) Best bid price * 10^4
    pub bid1_price_e4: &'a str,
    // Best bid price
    pub bid1_price: &'a str,
    // (Deprecated) Best ask price * 10^4
    pub ask1_price_e4: &'a str,
    // Best ask price
    pub ask1_price: &'a str,
    // Direction of price change
    pub last_tick_direction: &'a str,
    // (Deprecated) Price of 24 hours ago * 10^4
    pub prev_price_24h_e4: &'a str,
    // Price of 24 hours ago
    pub prev_price_24h: &'a str,
    // Percentage change of market price relative to 24h * 10^4
    pub price_24h_pcnt_e6: &'a str,
    // (Deprecated) The highest price in the last 24 hours * 10^4
    pub high_price_24h_e4: &'a str,
    // The highest price in the last 24 hours
    pub high_price_24h: &'a str,
    // (Deprecated) Lowest price in the last 24 hours * 10^4
    pub low_price_24h_e4: &'a str,
    // Lowest price in the last 24 hours
    pub low_price_24h: &'a str,
    // (Deprecated) Hourly market price an hour ago * 10^4
    pub prev_price_1h_e4: &'a str,
    // Hourly market price an hour ago
    pub prev_price_1h: &'a str,
    // Percentage change of market price relative to 1 hour ago * 10^6
    pub price_1h_pcnt_e6: &'a str,
    // (Deprecated) Mark price * 10^4
    pub mark_price_e4: &'a str,
    // Mark price
    pub mark_price: &'a str,
    // (Deprecated) Index_price * 10^4
    pub index_price_e4: &'a str,
    // Index_price
    pub index_price: &'a str,
    // Open interest * 10^8. The update is not immediate - slowest update is 1 minute
    pub open_interest_e8: &'a str,
    // Total turnover * 10^8
    pub total_turnover_e8: &'a str,
    // Turnover for 24h * 10^8
    pub turnover_24h_e8: &'a str,
    // Total volume * 10^8
    pub total_volume_e8: &'a str,
    // Trading volume in the last 24 hours * 10^8
    pub volume_24h_e8: &'a str,
    // Funding rate * 10^8
    pub funding_rate_e6: &'a str,
    // Predicted funding rate * 10^6
    pub predicted_funding_rate_e6: &'a str,
    // Cross sequence (internal value)
    pub cross_seq: &'a str,
    // Creation time (when the order_status was Created)
    pub created_at: &'a str,
    // Update time
    pub updated_at: &'a str,
    // Next settlement time of capital cost
    pub next_funding_time: &'a str,
    // Countdown of settlement capital cost
    pub count_down_hour: &'a str,
    // funding rate time interval, unit hour
    pub funding_rate_interval: &'a str,
    pub settle_time_e9: &'a str,
    pub delisting_status: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct InstrumentInfoDeltaItem<'a> {
    // id
    pub id: u64,
    // Symbol
    pub symbol: &'a str,
    pub last_price_e4: Option<&'a str>,
    pub last_price: Option<&'a str>,
    pub bid1_price_e4: Option<&'a str>,
    pub bid1_price: Option<&'a str>,
    pub ask1_price_e4: Option<&'a str>,
    pub ask1_price: Option<&'a str>,
    pub last_tick_direction: Option<&'a str>,
    pub prev_price_24h_e4: Option<&'a str>,
    pub prev_price_24h: Option<&'a str>,
    pub price_24h_pcnt_e6: Option<&'a str>,
    pub high_price_24h_e4: Option<&'a str>,
    pub high_price_24h: Option<&'a str>,
    pub low_price_24h_e4: Option<&'a str>,
    pub low_price_24h: Option<&'a str>,
    pub prev_price_1h_e4: Option<&'a str>,
    pub prev_price_1h: Option<&'a str>,
    pub price_1h_pcnt_e6: Option<&'a str>,
    pub mark_price_e4: Option<&'a str>,
    pub mark_price: Option<&'a str>,
    pub index_price_e4: Option<&'a str>,
    pub index_price: Option<&'a str>,
    pub open_interest_e8: Option<&'a str>,
    pub total_turnover_e8: Option<&'a str>,
    pub turnover_24h_e8: Option<&'a str>,
    pub total_volume_e8: Option<&'a str>,
    pub volume_24h_e8: Option<&'a str>,
    pub funding_rate_e6: Option<&'a str>,
    pub predicted_funding_rate_e6: Option<&'a str>,
    pub next_funding_time: Option<&'a str>,
    pub count_down_hour: Option<&'a str>,
    pub funding_rate_interval: Option<&'a str>,
    pub settle_time_e9: Option<&'a str>,
    pub delisting_status: Option<&'a str>,
    pub cross_seq: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct InstrumentInfoDelta<'a> {
    #[serde(borrow)]
    pub update: Vec<InstrumentInfoDeltaItem<'a>>,
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
    InstrumentInfoSnapshot(Response<'a, InstrumentInfoSnapshot<'a>>),
    InstrumentInfoDelta(Response<'a, InstrumentInfoDelta<'a>>),
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

    pub fn subscribe_instrument_info(&mut self, symbols: &Vec<String>) {
        self.subscribe("instrument_info.100ms", symbols);
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
    pub position_value: f64,
    // Average entry price
    pub entry_price: f64,
    // Liquidation price
    pub liq_price: f64,
    // Bankruptcy price
    pub bust_price: f64,
    // In Isolated Margin mode, the value is set by the user. In Cross Margin mode, the value is the max leverage at current risk level
    pub leverage: f64,
    // Pre-occupied order margin
    pub order_margin: f64,
    // Position margin
    pub position_margin: f64,
    // Position closing fee occupied (your opening fee + expected maximum closing fee)
    pub occ_closing_fee: f64,
    // Take profit price
    pub take_profit: f64,
    // Take profit trigger price type, default: LastPrice
    pub tp_trigger_by: &'a str,
    // Stop loss price
    pub stop_loss: f64,
    // Stop loss trigger price
    pub sl_trigger_by: &'a str,
    // Trailing stop (the distance from the current price)
    pub trailing_stop: f64,
    // Today's realised pnl
    pub realised_pnl: f64,
    // Auto add margin
    pub auto_add_margin: &'a str,
    // Accumulated realised pnl (all-time total)
    pub cum_realised_pnl: f64,
    // Position status: Normal, Liq, Adl
    pub position_status: &'a str,
    // Position id
    pub position_id: &'a str,
    // Position sequence
    pub position_seq: &'a str,
    // Adl rank indicator
    pub adl_rank_indicator: &'a str,
    // Qty which can be closed. (If you have a long position, free_qty is negative. vice versa)
    pub free_qty: f64,
    // TrailingProfit or StopLoss mode Full or Partial
    pub tp_sl_mode: &'a str,
    // Position idx, used to identify positions in different position modes:
    // 0-One-Way Mode
    // 1-Buy side of both side mode
    // 2-Sell side of both side mode
    pub position_idx: &'a str,
    // Position mode, MergedSingle or BothSide
    pub mode: &'a str,
    // true means isolated margin mode; false means cross margin mode
    pub isolated: bool,
    // Risk ID
    pub risk_id: &'a str,
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
    pub price: f64,
    // Order qty
    pub order_qty: f64,
    // Execution type (cannot be Funding)
    pub exec_type: &'a str,
    // Transaction qty
    pub exec_qty: f64,
    // Transaction fee
    pub exec_fee: f64,
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
    pub price: f64,
    // Transaction qty
    pub qty: f64,
    // Number of unfilled contracts from the order's size
    pub leaves_qty: f64,
    // Time in force
    pub time_in_force: &'a str,
    // Create type
    pub create_type: &'a str,
    // Cancel type
    pub cancel_type: &'a str,
    // Take profit price, only take effect upon opening the position
    pub take_profit: f64,
    // Stop loss price, only take effect upon opening the position
    pub stop_loss: f64,
    // Trailing stop (the distance from the current price)
    pub trailing_stop: f64,
    // Order status
    pub order_status: &'a str,
    // Last execution price
    pub last_exec_price: f64,
    // Cumulative qty of trading
    pub cum_exec_qty: f64,
    // Cumulative value of trading
    pub cum_exec_value: f64,
    // Cumulative trading fees
    pub cum_exec_fee: f64,
    // True means your position can only reduce in size if this order is triggered
    pub reduce_only: bool,
    // For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
    pub close_on_trigger: bool,
    // Position idx, used to identify positions in different position modes:
    // 0-One-Way Mode
    // 1-Buy side of both side mode
    // 2-Sell side of both side mode
    pub position_idx: &'a str,
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
    pub price: f64,
    // Transaction qty
    pub qty: f64,
    // Time in force
    pub time_in_force: &'a str,
    // Create type
    pub create_type: &'a str,
    // Cancel type
    pub cancel_type: &'a str,
    // Order status
    pub order_status: &'a str,
    // Conditional order type
    pub stop_order_type: &'a str,
    // Trigger price type. Default LastPrice
    pub tp_trigger_by: &'a str,
    // If stop_order_type is TrailingProfit, this field is the trailing stop active price.
    pub trigger_price: f64,
    // True means your position can only reduce in size if this order is triggered
    pub reduce_only: bool,
    // For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.
    pub close_on_trigger: bool,
    // Position idx, used to identify positions in different position modes:
    // 0-One-Way Mode
    // 1-Buy side of both side mode
    // 2-Sell side of both side mode
    pub position_idx: &'a str,
    // Creation time (when the order_status was Created)
    pub create_time: &'a str,
    // Update time
    pub update_time: &'a str,
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
    pub data: Vec<D>,
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

// todo: move to shared mod
#[derive(Serialize)]
struct AuthReq<'a> {
    op: &'a str,
    args: [&'a str; 3],
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

        // authentication
        let expires = millseconds()? + 10000;
        let val = format!("GET/realtime{}", expires);
        let signature = sign(&self.secret, &val);
        let auth_req = AuthReq {
            op: "auth",
            args: [&self.api_key, &expires.to_string(), &signature],
        };
        ws.write_message(Message::Text(serde_json::to_string(&auth_req)?))?;

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
