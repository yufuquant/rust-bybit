use crate::callback::{Callback, On};
use crate::error::Result;
use crate::{future_ping, run, Credentials};

use serde::{Deserialize, Serialize};
use serde_json;

const MAINNET_PUBLIC: &str = "wss://stream.bybit.com/realtime_public";
const MAINNET_PRIVATE: &str = "wss://stream.bybit.com/realtime_private";
const TESTNET_PUBLIC: &str = "wss://stream-testnet.bybit.com/realtime_public";
const TESTNET_PRIVATE: &str = "wss://stream-testnet.bybit.com/realtime_private";

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

pub struct OnPublicResponse;
impl<'a> On<'a> for OnPublicResponse {
    type Arg = PublicResponse<'a>;
}

#[derive(Serialize, Debug)]
struct Subscription {
    op: String,
    args: Vec<String>,
}

pub struct PublicWebSocketApiClientBuilder {
    uri: String,
}

impl Default for PublicWebSocketApiClientBuilder {
    fn default() -> Self {
        Self {
            uri: MAINNET_PUBLIC.to_owned(),
        }
    }
}

impl PublicWebSocketApiClientBuilder {
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET_PUBLIC.to_owned();
        self
    }

    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = uri.to_owned();
        self
    }

    pub fn build(self) -> PublicWebSocketApiClient {
        PublicWebSocketApiClient {
            uri: self.uri,
            subscriptions: Vec::new(),
        }
    }
}

pub struct PublicWebSocketApiClient {
    pub uri: String,
    subscriptions: Vec<String>,
}

impl PublicWebSocketApiClient {
    pub fn new() -> Self {
        return Self::builder().build();
    }

    pub fn builder() -> PublicWebSocketApiClientBuilder {
        PublicWebSocketApiClientBuilder::default()
    }

    pub fn subscribe_order_book_l2_25<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("orderBookL2_25", symbols);
    }

    pub fn subscribe_order_book_l2_200<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("orderBook_200.100ms", symbols);
    }

    pub fn subscribe_trade<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("trade", symbols);
    }

    pub fn subscribe_instrument_info<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("instrument_info.100ms", symbols);
    }

    pub fn subscribe_kline<S: AsRef<str>>(&mut self, symbols: &[S], interval: &str) {
        let topic = format!("candle.{}", interval);
        self.subscribe(&topic, symbols);
    }

    pub fn subscribe_liquidation<S: AsRef<str>>(&mut self, symbols: &[S]) {
        self.subscribe("liquidation", symbols);
    }

    pub fn run<T: Callback<OnPublicResponse>>(&self, callback: T) -> Result<()> {
        run(&self.uri, None, &self.subscriptions, future_ping, callback)
    }

    fn subscribe<S: AsRef<str>>(&mut self, topic: &str, symbols: &[S]) {
        let args = symbols
            .iter()
            .map(|symbol| format!("{}.{}", topic, symbol.as_ref()))
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

pub struct OnPrivateResponse;
impl<'a> On<'a> for OnPrivateResponse {
    type Arg = PrivateResponse<'a>;
}

pub struct PrivateWebSocketApiClientBuilder {
    pub uri: String,
}

impl Default for PrivateWebSocketApiClientBuilder {
    fn default() -> Self {
        Self {
            uri: MAINNET_PRIVATE.to_owned(),
        }
    }
}

impl PrivateWebSocketApiClientBuilder {
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET_PRIVATE.to_owned();
        self
    }

    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = uri.to_owned();
        self
    }

    pub fn build_with_credentials<S: AsRef<str>>(
        self,
        api_key: S,
        secret: S,
    ) -> PrivateWebSocketApiClient {
        PrivateWebSocketApiClient {
            uri: self.uri,
            api_key: api_key.as_ref().to_owned(),
            secret: secret.as_ref().to_owned(),
            subscriptions: Vec::new(),
        }
    }
}

pub struct PrivateWebSocketApiClient {
    pub uri: String,
    pub api_key: String,
    pub secret: String,
    subscriptions: Vec<String>,
}

impl PrivateWebSocketApiClient {
    pub fn new<S: AsRef<str>>(api_key: S, secret: S) -> Self {
        Self::builder().build_with_credentials(api_key, secret)
    }

    pub fn builder() -> PrivateWebSocketApiClientBuilder {
        PrivateWebSocketApiClientBuilder::default()
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

    pub fn run<T: Callback<OnPrivateResponse>>(&self, callback: T) -> Result<()> {
        let credentials = Credentials {
            api_key: self.api_key.clone(),
            secret: self.secret.clone(),
        };
        run(
            &self.uri,
            Some(&credentials),
            &Vec::new(),
            future_ping,
            callback,
        )
    }
}
