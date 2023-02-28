use super::callback::Arg;
use serde::Deserialize;

/// The pong/subscription response.
#[derive(Deserialize, Debug)]
pub struct OpResponse<'a> {
    pub success: bool,
    pub ret_msg: &'a str,
    pub conn_id: &'a str,
    pub req_id: Option<&'a str>,
    pub op: &'a str,
}

/// The option pong response of public channels.
#[derive(Deserialize, Debug)]
pub struct OptionPongResponse<'a> {
    pub args: [&'a str; 1],
    pub op: &'a str,
}

/// The data in option subscription response.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionSubscriptionData<'a> {
    #[serde(borrow)]
    pub fail_topics: Vec<&'a str>,
    pub success_topics: Vec<&'a str>,
}

/// The option subscription response.
#[derive(Deserialize, Debug)]
pub struct OptionSubscriptionResponse<'a> {
    pub success: bool,
    pub conn_id: &'a str,
    pub data: OptionSubscriptionData<'a>,
    #[serde(alias = "type")]
    pub type_: &'a str,
}

/// The pong response of private channels.
#[derive(Deserialize, Debug)]
pub struct PrivatePongResponse<'a> {
    pub req_id: Option<&'a str>,
    pub op: &'a str,
    pub args: [&'a str; 1],
    pub conn_id: &'a str,
}

/// The base response which contains common fields of public channels.
#[derive(Deserialize, Debug)]
pub struct BasePublicResponse<'a, Data> {
    /// Topic name.
    pub topic: &'a str,
    /// Data type. `snapshot`, `delta`.
    #[serde(alias = "type")]
    pub type_: &'a str,
    /// The timestamp (ms) that the system generates the data.
    pub ts: u64,
    /// The data vary on the topic.
    pub data: Data,
}

/// The base ticker response which contains common fields.
#[derive(Deserialize, Debug)]
pub struct BaseTickerPublicResponse<'a, Data> {
    /// Topic name.
    pub topic: &'a str,
    /// Data type. `snapshot`, `delta`.
    #[serde(alias = "type")]
    pub type_: &'a str,
    /// Cross sequence.
    pub cs: u64,
    /// The timestamp (ms) that the system generates the data.
    pub ts: u64,
    /// The spot/future ticker data.
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct BaseOptionPublicResponse<'a, Data> {
    /// message ID
    pub id: &'a str,
    /// Topic name.
    pub topic: &'a str,
    #[serde(alias = "type")]
    /// Data type. `snapshot`.
    pub type_: &'a str,
    /// The timestamp (ms) that the system generates the data.
    pub ts: u64,
    /// The data vary on the topic.
    pub data: Data,
}

/// The base response which contains common fields of private channels.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasePrivateResponse<'a, Data> {
    /// Message ID.
    pub id: &'a str,
    /// Topic name.
    pub topic: &'a str,
    /// Data created timestamp (ms).
    pub creation_time: u64,
    /// The data vary on the topic.
    pub data: Data,
}

/// The (price, size) pair of orderbook.
#[derive(Deserialize, Debug)]
pub struct OrderbookItem<'a>(pub &'a str, pub &'a str);

/// The orderbook data.
#[derive(Deserialize, Debug)]
pub struct Orderbook<'a> {
    /// Symbol name.
    pub s: &'a str,
    /// Bids. For `snapshot` stream, the element is sorted by price in descending order.
    pub b: Vec<OrderbookItem<'a>>,
    /// Asks. For `snapshot` stream, the element is sorted by price in ascending order.
    pub a: Vec<OrderbookItem<'a>>,
    /// Update ID. Is a sequence.
    /// Occasionally, you'll receive "u"=1, which is a snapshot data due to the restart of the service.
    /// So please overwrite your local orderbook.
    pub u: u64,
    /// Cross sequence. Option does not have this field.
    pub seq: Option<u64>,
}

/// The trade data.
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Trade<'a> {
    /// The timestamp (ms) that the order is filled.
    pub T: u64,
    /// Symbol name.
    pub s: &'a str,
    /// Side. `Buy`, `Sell`.
    pub S: &'a str,
    /// Trade size.
    pub v: &'a str,
    /// Trade price.
    pub p: &'a str,
    /// Direction of price change. Unique field for future.
    pub L: Option<&'a str>,
    /// Trade ID.
    pub i: &'a str,
    /// Whether it is a block trade order or not.
    pub BT: bool,
}

/// The spot ticker data. (`snapshot` only)
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker<'a> {
    /// Symbol name.
    pub symbol: &'a str,
    /// Last price.
    pub last_price: &'a str,
    /// The highest price in the last 24 hours.
    pub high_price_24h: &'a str,
    /// The lowest price in the last 24 hours.
    pub low_price_24h: &'a str,
    /// Percentage change of market price relative to 24h.
    pub prev_price_24h: &'a str,
    /// Volume for 24h.
    pub volume_24h: &'a str,
    /// Turnover for 24h.
    pub turnover_24h: &'a str,
    /// Percentage change of market price relative to 24h.
    pub price_24h_pcnt: &'a str,
    /// USD index price. It can be empty.
    pub usd_index_price: &'a str,
}

/// The option ticker data. (`snapshot` only)
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionTicker<'a> {
    /// Symbol name.
    pub symbol: &'a str,
    /// Best bid price.
    pub bid_price: &'a str,
    /// Best bid size.
    pub bid_size: &'a str,
    /// Best bid iv.
    pub bid_iv: &'a str,
    /// Best ask price.
    pub ask_price: &'a str,
    /// Best ask size.
    pub ask_size: &'a str,
    /// Best ask iv.
    pub ask_iv: &'a str,
    /// Last price.
    pub last_price: &'a str,
    /// The highest price in the last 24 hours.
    pub high_price_24h: &'a str,
    /// The lowest price in the last 24 hours.
    pub low_price_24h: &'a str,
    /// Market price.
    pub mark_price: &'a str,
    /// Index price.
    pub index_price: &'a str,
    /// Mark price iv.
    pub mark_price_iv: &'a str,
    /// Underlying price.
    pub underlying_price: &'a str,
    /// Open interest size.
    pub open_interest: &'a str,
    /// Turnover for 24h.
    pub turnover_24h: &'a str,
    /// Volume for 24h.
    pub volume_24h: &'a str,
    /// Total volume.
    pub total_volume: &'a str,
    /// Total turnover.
    pub total_turnover: &'a str,
    /// Delta.
    pub delta: &'a str,
    /// Gamma.
    pub gamma: &'a str,
    /// Vega.
    pub vega: &'a str,
    /// Theta.
    pub theta: &'a str,
    /// Predicated delivery price. It has value when 30 min before delivery.
    pub predicted_delivery_price: &'a str,
    /// The change in the last 24 hous.
    pub change_24h: &'a str,
}

/// The future ticker data.
///
/// This data utilises the snapshot field and delta field. `None` means field value has not changed.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FutureTicker<'a> {
    /// Symbol name.
    pub symbol: &'a str,
    /// Tick direction.
    pub tick_direction: Option<&'a str>,
    /// Percentage change of market price in the last 24 hours.
    pub price_24h_pcnt: Option<&'a str>,
    /// Last price.
    pub last_price: Option<&'a str>,
    /// Market price 24 hours ago.
    pub prev_price_24h: Option<&'a str>,
    /// The highest price in the last 24 hours.
    pub high_price_24h: Option<&'a str>,
    /// The lowest price in the last 24 hours.
    pub low_price_24h: Option<&'a str>,
    /// Market price an hour ago.
    pub prev_price_1h: Option<&'a str>,
    /// Mark price.
    pub mark_price: Option<&'a str>,
    /// Index price.
    pub index_price: Option<&'a str>,
    /// Open interest size.
    pub open_interest: Option<&'a str>,
    /// Open interest value.
    pub open_interest_value: Option<&'a str>,
    /// Turnover for 24h.
    pub turnover_24h: Option<&'a str>,
    /// Volume for 24h.
    pub volume_24h: Option<&'a str>,
    /// Next funding timestamp (ms).
    pub next_funding_time: Option<&'a str>,
    /// Funding rate.
    pub funding_rate: Option<&'a str>,
    /// Best bid price.
    pub bid1_price: Option<&'a str>,
    /// Best bid size.
    pub bid1_size: Option<&'a str>,
    /// Best ask price.
    pub ask1_price: Option<&'a str>,
    /// Best ask size.
    pub ask1_size: Option<&'a str>,
    /// Delivery date time (UTC+0). Unique field for inverse futures.
    pub delivery_time: Option<&'a str>,
    /// Delivery fee rate. Unique field for inverse futures.
    pub basis_rate: Option<&'a str>,
    /// Delivery fee rate. Unique field for inverse futures.
    pub delivery_fee_rate: Option<&'a str>,
    /// Predicated delivery price. Unique field for inverse futures.
    pub predicted_delivery_price: Option<&'a str>,
}

/// The (leveraged token) kline data.
#[derive(Deserialize, Debug)]
pub struct Kline<'a> {
    /// The start timestamp (ms)
    pub start: u64,
    /// The end timestamp (ms). It is current timestamp if it does not reach to the end time of candle.
    pub end: u64,
    /// Kline interval.
    pub interval: &'a str,
    /// Open price.
    pub open: &'a str,
    /// Close price.
    pub close: &'a str,
    /// Highest price.
    pub high: &'a str,
    /// Lowest price.
    pub low: &'a str,
    /// Trade volume. Leveraged token does not have this field.
    pub volume: Option<&'a str>,
    /// Turnover. Leveraged token does not have this field.
    pub turnover: Option<&'a str>,
    /// Weather the tick is ended or not.
    pub confirm: bool,
    /// The timestamp (ms) of the last matched order in the candle.
    pub timestamp: u64,
}

/// The liquidation data.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Liquidation<'a> {
    /// The updated timestamp (ms).
    pub updated_time: u64,
    /// Symbol name.
    pub symbol: &'a str,
    /// Order side. `Buy`, `Sell`.
    pub side: &'a str,
    /// Executed size.
    pub size: &'a str,
    /// Executed price.
    pub price: &'a str,
}

// The leveraged token ticker data.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LtTicker<'a> {
    /// Symbol name.
    pub symbol: &'a str,
    /// Market price change percentage in the past 24 hours.
    pub price_24h_pcnt: &'a str,
    /// The last price.
    pub last_price: &'a str,
    /// Market price 24 hours ago.
    pub prev_price_24h: &'a str,
    /// Highest price in the past 24 hours.
    pub high_price_24h: &'a str,
    /// Lowest price in the past 24 hours.
    pub low_price24h: &'a str,
}

/// The leveraged token nav data.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LtNav<'a> {
    /// The generated timestamp of nav.
    pub time: u64,
    /// Symbol name.
    pub symbol: &'a str,
    /// Net asset value.
    pub nav: &'a str,
    /// Total position value = basket value * total circulation.
    pub basket_position: &'a str,
    /// Leverage.
    pub leverage: &'a str,
    /// Basket loan.
    pub basket_loan: &'a str,
    /// Circulation.
    pub circulation: &'a str,
    /// Basket.
    pub basket: &'a str,
}

/// The position data.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position<'a> {
    /// Product type.
    /// - Unified account: does not have this field.
    /// - Normal account: `linear`, `inverse`.
    pub category: Option<&'a str>,
    /// Symbol name.
    pub symbol: &'a str,
    /// Position side: `Buy`, `Sell`.
    pub side: &'a str,
    /// Position size.
    pub size: &'a str,
    /// Used to identify positions in different position modes.
    /// - 0 one-way mode position.
    /// - 1 Buy side of hedge-mode position.
    /// - 2 Sell side of hedge-mode position.
    pub position_idx: u8,
    /// Trade mode. 0: cross margin, 1: isolated margin. Always 0 under unified margin account.
    pub trade_mode: u8,
    /// Position value.
    pub position_value: &'a str,
    /// Risk limit ID.
    /// _Note_: for portfolio margin mode, it returns 0, which the risk limit value is invalid.
    pub risk_id: u16,
    /// Risk limit value corresponding to riskId.
    /// _Note_: for portfolio margin mode, it returns "", which the risk limit value is invalid.
    pub risk_limit_value: &'a str,
    /// Entry price.
    pub entry_price: &'a str,
    /// Mark price
    pub mark_price: &'a str,
    /// Leverage.
    /// _Note_: for portfolio margin mode, it returns "", which the leverage value is invalid.
    pub leverage: &'a str,
    /// Position margin. Unified account does not have this field.
    pub position_balance: Option<&'a str>,
    /// Whether to add margin automatically. 0: false, 1: true. Unified account does not have this field.
    pub auto_add_margin: Option<u8>,
    /// Position maintenance margin.
    /// _Note_: for portfolio margin mode, it returns "".
    #[serde(alias = "positionMM")]
    pub position_mm: &'a str,
    /// Position initial margin.
    /// _Note_: for portfolio margin mode, it returns "".
    #[serde(alias = "positionIM")]
    pub position_im: &'a str,
    /// Est.liquidation price. "" for Unified trade(spot/linear/options).
    pub liq_price: &'a str,
    /// Est.bankruptcy price. "" for Unified trade(spot/linear/options).
    pub bust_price: &'a str,
    /// Tp/Sl mode: `Full`, `Partial`.
    pub tpsl_mode: &'a str,
    /// Take profit price.
    pub take_profit: &'a str,
    /// Stop loss price.
    pub stop_loss: &'a str,
    /// Trailing stop.
    pub trailing_stop: &'a str,
    /// Unrealised profit and loss.
    pub unrealised_pnl: &'a str,
    /// Cumulative realised PnL.
    pub cum_realised_pnl: &'a str,
    /// Position status.
    /// -`Normal`.
    /// - `Liq`: in the liquidation progress.
    /// - `Adl`: in the auto-deleverage progress.
    pub position_status: &'a str,
    /// Position created timestamp (ms).
    pub created_time: &'a str,
    /// Position data updated timestamp (ms).
    pub updated_time: &'a str,
}

/// The execution data.
///
/// You may have multiple executions for one order in a single message.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Execution<'a> {
    /// Product type.
    /// - Unified account: `spot`, `linear`, `option`.
    /// - Normal account: `linear`, `inverse`.
    pub category: &'a str,
    /// Symbol name.
    pub symbol: &'a str,
    /// Whether to borrow. Valid for `spot` only.
    /// - 0 (default): false.
    /// - 1: true.
    pub is_leverage: &'a str,
    /// Order ID.
    pub order_id: &'a str,
    /// User customized order ID.
    pub order_link_id: &'a str,
    /// Side. `Buy`, `Sell`.
    pub side: &'a str,
    /// Order price.
    pub order_price: &'a str,
    /// Order qty.
    pub order_qty: &'a str,
    /// The remaining qty not executed.
    pub leaves_qty: &'a str,
    /// Order type. `Market`, `Limit`.
    pub order_type: &'a str,
    /// Stop order type. If the order is not stop order, any type is not returned.
    pub stop_order_type: &'a str,
    /// Executed trading fee.
    pub exec_fee: &'a str,
    /// Execution ID.
    pub exec_id: &'a str,
    /// Execution price.
    pub exec_price: &'a str,
    /// Execution qty.
    pub exec_qty: &'a str,
    /// Executed type.
    pub exec_type: &'a str,
    /// Executed order value.
    pub exec_value: &'a str,
    /// Executed timestamp (ms).
    pub exec_time: &'a str,
    /// Is maker order. true: maker, false: taker.
    pub is_maker: bool,
    /// Trading fee rate.
    pub fee_rate: &'a str,
    /// Implied volatility. Valid for option.
    pub trade_iv: &'a str,
    /// Implied volatility of mark price. Valid for option.
    pub mark_iv: &'a str,
    /// The mark price of the symbol when executing.
    pub mark_price: &'a str,
    /// The index price of the symbol when executing.
    pub index_price: &'a str,
    /// The underlying price of the symbol when executing. Valid for option.
    pub underlying_price: &'a str,
    /// Paradigm block trade ID.
    pub block_trade_id: &'a str,
}

/// The order data.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order<'a> {
    /// Product type.
    /// - Unified account: `spot`, `linear`, `option`.
    /// - Normal account: `linear`, `inverse`.
    pub category: &'a str,
    /// Order ID.
    pub order_id: &'a str,
    /// User customised order ID.
    pub order_link_id: &'a str,
    /// Whether to borrow. `spot` returns this field only. 0 (default): false, 1: true.
    pub is_leverage: &'a str,
    /// Block trade ID.
    pub block_trade_id: &'a str,
    /// Symbol name.
    pub symbol: &'a str,
    /// Order price.
    pub price: &'a str,
    /// Order qty.
    pub qty: &'a str,
    /// Side. `Buy`, `Sell`.
    pub side: &'a str,
    /// Position index. Used to identify positions in different position modes.
    pub position_idx: u8,
    /// Order status.
    pub order_status: &'a str,
    /// Cancel type.
    pub cancel_type: &'a str,
    /// Reject reason.
    pub reject_reason: &'a str,
    /// Average filled price. If unfilled, it is "".
    pub avg_price: &'a str,
    /// The remaining qty not executed.
    pub leaves_qty: &'a str,
    /// The remaining value not executed.
    pub leaves_value: &'a str,
    /// Cumulative executed order qty.
    pub cum_exec_qty: &'a str,
    /// Cumulative executed order value.
    pub cum_exec_value: &'a str,
    /// Cumulative executed trading fee.
    pub cum_exec_fee: &'a str,
    /// Time in force.
    pub time_in_force: &'a str,
    /// Order type. `Market`, `Limit`.
    pub order_type: &'a str,
    /// Stop order type.
    pub stop_order_type: &'a str,
    /// Implied volatility.
    pub order_iv: &'a str,
    /// Trigger price. If stopOrderType=TrailingStop, it is activate price. Otherwise, it is trigger price.
    pub trigger_price: &'a str,
    /// Take profit price.
    pub take_profit: &'a str,
    /// Stop loss price.
    pub stop_loss: &'a str,
    /// The price type to trigger take profit.
    pub tp_trigger_by: &'a str,
    /// The price type to trigger stop loss.
    pub sl_trigger_by: &'a str,
    /// Trigger direction. 1: rise, 2: fall.
    pub trigger_direction: u8,
    /// The price type of trigger price.
    pub trigger_by: &'a str,
    /// Last price when place the order. For linear only.
    pub last_price_on_created: &'a str,
    /// Reduce only. `true` means reduce position size.
    pub reduce_only: bool,
    /// Close on trigger.
    pub close_on_trigger: bool,
    /// Order created timestamp (ms).
    pub created_time: &'a str,
    /// Order updated timestamp (ms).
    pub updated_time: &'a str,
}

/// The wallet coin data.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletCoin<'a> {
    /// Coin name, such as BTC, ETH, USDT, USDC.
    pub coin: &'a str,
    /// Equity of current coin.
    pub equity: &'a str,
    /// USD value of current coin. If this coin cannot be collateral, then it is 0.
    pub usd_value: &'a str,
    /// Wallet balance of current coin.
    pub wallet_balance: &'a str,
    /// Borrow amount of current coin.
    pub borrow_amount: &'a str,
    /// Available amount to borrow of current coin.
    pub available_to_borrow: &'a str,
    /// Available amount to withdraw of current coin.
    pub available_to_withdraw: &'a str,
    /// Accrued interest.
    pub accrued_interest: &'a str,
    /// Pre-occupied margin for order. For portfolio margin mode, it returns "".
    #[serde(alias = "totalOrderIM")]
    pub total_order_im: &'a str,
    /// Sum of initial margin of all positions + Pre-occupied liquidation fee. For portfolio margin mode, it returns "".
    #[serde(alias = "totalPositionIM")]
    pub total_position_im: &'a str,
    /// Sum of maintenance margin for all positions. For portfolio margin mode, it returns "".
    #[serde(alias = "totalPositionMM")]
    pub total_position_mm: &'a str,
    /// Unrealised P&L.
    pub unrealised_pnl: &'a str,
    /// Cumulative Realised P&L.
    pub cum_realised_pnl: &'a str,
}

/// The wallet data.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wallet<'a> {
    /// Account type.
    /// - Unified account: UNIFIED.
    /// - Normal account: CONTRACT.
    pub account_type: &'a str,
    /// Initial Margin Rate: Account Total Initial Margin Base Coin / Account Margin Balance Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    #[serde(alias = "accountIMRate")]
    pub account_im_rate: &'a str,
    /// Maintenance Margin Rate: Account Total Maintenance Margin Base Coin / Account Margin Balance Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    #[serde(alias = "accountMMRate")]
    pub account_mm_rate: &'a str,
    /// Equity of account converted to usd：Account Margin Balance Base Coin + Account Option Value Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_equity: &'a str,
    /// Wallet Balance of account converted to usd：∑ Asset Wallet Balance By USD value of each asset.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_wallet_balance: &'a str,
    /// Margin Balance of account converted to usd：totalWalletBalance + totalPerpUPL.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_margin_balance: &'a str,
    /// Available Balance of account converted to usd：Regular mode：totalMarginBalance - totalInitialMargin.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_available_balance: &'a str,
    /// Unrealised P&L of perpetuals of account converted to usd：∑ Each perp upl by base coin.
    /// In non-unified mode, the field will be returned as an empty string.
    #[serde(alias = "totalPerpUPL")]
    pub total_perp_upl: &'a str,
    /// Initial Margin of account converted to usd：∑ Asset Total Initial Margin Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_initial_margin: &'a str,
    /// Maintenance Margin of account converted to usd: ∑ Asset Total Maintenance Margin Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_maintenance_margin: &'a str,
    /// Coin.
    pub coin: Vec<WalletCoin<'a>>,
}

/// The greeks data.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Greek<'a> {
    /// Base coin.
    pub base_coin: &'a str,
    /// Delta value.
    pub total_delta: &'a str,
    /// Gamma value.
    pub total_gamma: &'a str,
    /// Vega value.
    pub total_vega: &'a str,
    /// Theta value.
    pub total_theta: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SpotPublicResponse<'a> {
    #[serde(borrow)]
    Orderbook(BasePublicResponse<'a, Orderbook<'a>>),
    Trade(BasePublicResponse<'a, Vec<Trade<'a>>>),
    Ticker(BaseTickerPublicResponse<'a, SpotTicker<'a>>),
    Kline(BasePublicResponse<'a, Vec<Kline<'a>>>),
    LtTicker(BasePublicResponse<'a, LtTicker<'a>>),
    LtNav(BasePublicResponse<'a, LtNav<'a>>),
    Op(OpResponse<'a>),
}

pub struct SpotPublicResponseArg;
impl Arg for SpotPublicResponseArg {
    type ValueType<'a> = SpotPublicResponse<'a>;
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum FuturePublicResponse<'a> {
    #[serde(borrow)]
    Orderbook(BasePublicResponse<'a, Orderbook<'a>>),
    Trade(BasePublicResponse<'a, Vec<Trade<'a>>>),
    Ticker(BaseTickerPublicResponse<'a, FutureTicker<'a>>),
    Kline(BasePublicResponse<'a, Vec<Kline<'a>>>),
    Liquidation(BasePublicResponse<'a, Liquidation<'a>>),
    Op(OpResponse<'a>),
}

pub struct FuturePublicResponseArg;
impl Arg for FuturePublicResponseArg {
    type ValueType<'a> = FuturePublicResponse<'a>;
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OptionPublicResponse<'a> {
    #[serde(borrow)]
    Orderbook(BaseOptionPublicResponse<'a, Orderbook<'a>>),
    Trade(BaseOptionPublicResponse<'a, Vec<Trade<'a>>>),
    Ticker(BaseOptionPublicResponse<'a, OptionTicker<'a>>),
    Pong(OptionPongResponse<'a>),
    Subscription(OptionSubscriptionResponse<'a>),
}

pub struct OptionPublicResponseArg;
impl Arg for OptionPublicResponseArg {
    type ValueType<'a> = OptionPublicResponse<'a>;
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PrivateResponse<'a> {
    #[serde(borrow)]
    Position(BasePrivateResponse<'a, Vec<Position<'a>>>),
    Execution(BasePrivateResponse<'a, Vec<Execution<'a>>>),
    Order(BasePrivateResponse<'a, Vec<Order<'a>>>),
    Wallet(BasePrivateResponse<'a, Vec<Wallet<'a>>>),
    Greek(BasePrivateResponse<'a, Vec<Greek<'a>>>),
    Pong(PrivatePongResponse<'a>),
    Op(OpResponse<'a>),
}

pub struct PrivateResponseArg;
impl Arg for PrivateResponseArg {
    type ValueType<'a> = PrivateResponse<'a>;
}
