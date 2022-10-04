use crate::callback::{Callback, On};
use crate::error::Result;
use crate::spot_ping;
use crate::{run, Credentials};

use serde::{Deserialize, Serialize};
use serde_json;

const MAINNET_PUBLIC: &str = "wss://stream.bybit.com/spot/quote/ws/v1";
const MAINNET_PUBLIC_V2: &str = "wss://stream.bybit.com/spot/quote/ws/v2";
const MAINNET_PRIVATE: &str = "wss://stream.bybit.com/spot/ws";
const TESTNET_PUBLIC: &str = "wss://stream-testnet.bybit.com/spot/quote/ws/v1";
const TESTNET_PUBLIC_V2: &str = "wss://stream-testnet.bybit.com/spot/quote/ws/v2";
const TESTNET_PRIVATE: &str = "wss://stream-testnet.bybit.com/spot/ws";

#[derive(Deserialize, Debug)]
pub struct Ping {
    pub ping: u64,
}

#[derive(Deserialize, Debug)]
pub struct Pong {
    pub pong: u64,
}

#[derive(Deserialize, Debug)]
pub struct Trade<'a> {
    // Trade ID
    pub v: &'a str,
    // Timestamp (trading time in the match box)
    pub t: u64,
    // Price
    pub p: &'a str,
    // Quantity
    pub q: &'a str,
    // True indicates buy side is taker, false indicates sell side is taker
    pub m: bool,
}

#[derive(Deserialize, Debug)]
pub struct Realtimes<'a> {
    // Timestamp (trading time in the match box)
    pub t: u64,
    // Trading pair
    pub s: &'a str,
    // Close price
    pub c: &'a str,
    // High price
    pub h: &'a str,
    // Low price
    pub l: &'a str,
    // Open price
    pub o: &'a str,
    // Trading volume
    pub v: &'a str,
    // Trading quote volume
    pub qv: &'a str,
    // Change
    pub m: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct Kline<'a> {
    // Starting time
    pub t: u64,
    // Trading pair
    pub s: &'a str,
    // Trading pair
    pub sn: &'a str,
    // Close price
    pub c: &'a str,
    // High price
    pub h: &'a str,
    // Low price
    pub l: &'a str,
    // Open price
    pub o: &'a str,
    // Trading volume
    pub v: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct OrderBookItem<'a>(pub &'a str, pub &'a str);

#[derive(Deserialize, Debug)]
pub struct Depth<'a> {
    // Timestamp (last update time of the order book)
    pub t: u64,
    // Trading pair
    pub s: &'a str,
    // Version
    pub v: &'a str,
    // Best bid price, quantity
    pub b: Vec<OrderBookItem<'a>>,
    // Best ask price, quantity
    pub a: Vec<OrderBookItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct DiffDepth<'a> {
    // Timestamp (last update time of the order book)
    pub t: u64,
    // Version
    pub v: &'a str,
    // Best bid price, quantity
    pub b: Vec<OrderBookItem<'a>>,
    // Best ask price, quantity
    pub a: Vec<OrderBookItem<'a>>,
}

#[derive(Deserialize, Debug)]
pub struct LT<'a> {
    // Timestamp
    pub t: u64,
    // Please make sure to add "NAV" as a suffix to the name of the pair you're querying
    pub s: &'a str,
    // Net asset value
    pub nav: &'a str,
    // Basket value
    pub b: &'a str,
    // Real Leverage calculated by last traded price
    pub l: &'a str,
    // Basket loan
    pub loan: &'a str,
    // Circulating supply in the secondary market
    pub ti: &'a str,
    // Total position value = basket value * total circulation
    pub n: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResCommonParams<'a> {
    pub binary: &'a str,
    pub realtime_interval: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResKlineParams<'a> {
    pub binary: &'a str,
    pub realtime_interval: &'a str,
    pub kline_type: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResMergedDepthParams<'a> {
    pub binary: &'a str,
    pub realtime_interval: &'a str,
    pub dump_scale: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response<'a, P, D> {
    pub symbol: &'a str,
    pub symbol_name: &'a str,
    pub topic: &'a str,
    pub params: P,
    pub data: Vec<D>,
    pub f: bool,
    pub send_time: u64,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PublicResponse<'a> {
    #[serde(borrow)]
    Trade(Response<'a, ResCommonParams<'a>, Trade<'a>>),
    Realtimes(Response<'a, ResCommonParams<'a>, Realtimes<'a>>),
    Kline(Response<'a, ResKlineParams<'a>, Kline<'a>>),
    Depth(Response<'a, ResCommonParams<'a>, Depth<'a>>),
    MergedDepth(Response<'a, ResMergedDepthParams<'a>, Depth<'a>>),
    DiffDepth(Response<'a, ResCommonParams<'a>, DiffDepth<'a>>),
    LT(Response<'a, ResCommonParams<'a>, LT<'a>>),
    Pong(Pong),
    Ping(Ping),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResCommonParamsV2<'a> {
    pub binary: &'a str,
    pub symbol: &'a str,
    pub symbol_name: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResKlineParamsV2<'a> {
    pub binary: &'a str,
    pub symbol: &'a str,
    pub symbol_name: &'a str,
    pub kline_type: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker<'a> {
    // Trading pair
    pub symbol: &'a str,
    // Best bid price
    pub bid_price: &'a str,
    // Bid quantity
    pub bid_qty: &'a str,
    // Best ask price
    pub ask_price: &'a str,
    // Ask quantity
    pub ask_qty: &'a str,
    // Timestamp (last update time of the order book)
    pub time: u64,
}

#[derive(Deserialize, Debug)]
pub struct ResponseV2<'a, P, D> {
    pub topic: &'a str,
    pub params: P,
    pub data: D,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PublicV2Response<'a> {
    #[serde(borrow)]
    Depth(ResponseV2<'a, ResCommonParamsV2<'a>, Depth<'a>>),
    Kline(ResponseV2<'a, ResKlineParamsV2<'a>, Kline<'a>>),
    Trade(ResponseV2<'a, ResCommonParamsV2<'a>, Trade<'a>>),
    BookTicker(ResponseV2<'a, ResCommonParamsV2<'a>, BookTicker<'a>>),
    Realtimes(ResponseV2<'a, ResCommonParamsV2<'a>, Realtimes<'a>>),
    Pong(Pong),
    Ping(Ping),
}

pub struct OnPublicV2Response;
impl<'a> On<'a> for OnPublicV2Response {
    type Arg = PublicV2Response<'a>;
}

#[derive(Deserialize, Debug)]
pub struct WalletBalanceChange<'a> {
    // coin name
    pub a: &'a str,
    // Available balance
    pub f: &'a str,
    // Reserved for orders
    pub l: &'a str,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct OutboundAccountInfo<'a> {
    // Event type
    pub e: &'a str,
    // Event time
    pub E: &'a str,
    // Allow trade
    pub T: bool,
    // Allow withdraw
    pub W: bool,
    //Allow deposit
    pub D: bool,
    // Wallet balance change
    pub B: Vec<WalletBalanceChange<'a>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ExecutionReport<'a> {
    // Event type
    pub e: &'a str,
    // Event time
    pub E: &'a str,
    // Trading pair
    pub s: &'a str,
    // User-generated order ID
    pub c: &'a str,
    // BUY indicates buy order, SELL indicates sell order
    pub S: &'a str,
    // Order type, LIMIT/MARKET_OF_QUOTE/MARKET_OF_BASE
    pub o: &'a str,
    // Time in force
    pub f: &'a str,
    // Quantity
    pub q: &'a str,
    // Price
    pub p: &'a str,
    // Order status
    pub X: &'a str,
    // Order ID
    pub i: &'a str,
    // Order ID of the opponent trader
    pub M: &'a str,
    // Last filled quantity
    pub l: &'a str,
    // Total filled quantity
    pub z: &'a str,
    // Last traded price
    pub L: &'a str,
    // Trading fee (for a single fill)
    pub n: &'a str,
    // Asset type in which fee is paid
    pub N: &'a str,
    // Is normal trade. False if self-trade.
    pub u: bool,
    // Is working
    pub w: bool,
    // Is LIMIT_MAKER
    pub m: bool,
    // Order creation time
    pub O: &'a str,
    // Total filled value
    pub Z: &'a str,
    // Account ID of the opponent trader
    pub A: &'a str,
    // Is close
    pub C: bool,
    // leverage
    pub v: &'a str,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TicketInfo<'a> {
    // Event type
    pub e: &'a str,
    // Event time
    pub E: &'a str,
    // Trading pair
    pub s: &'a str,
    // Quantity
    pub q: &'a str,
    // Timestamp
    pub t: &'a str,
    // Price
    pub p: &'a str,
    // Trade ID
    pub T: &'a str,
    // Order ID
    pub o: &'a str,
    // User-generated order ID
    pub c: &'a str,
    // Order ID of the opponent trader
    pub O: &'a str,
    // Account ID
    pub a: &'a str,
    // Account ID of the opponent trader
    pub A: &'a str,
    // Is MAKER. true=maker, false=taker
    pub m: bool,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PrivateResponse<'a> {
    #[serde(borrow)]
    OutboundAccountInfoSequence(Vec<OutboundAccountInfo<'a>>),
    ExecutionReportSequence(Vec<ExecutionReport<'a>>),
    TicketInfoSequence(Vec<TicketInfo<'a>>),
    Pong(Pong),
    Ping(Ping),
}

pub struct OnPrivateResponse;
impl<'a> On<'a> for OnPrivateResponse {
    type Arg = PrivateResponse<'a>;
}

#[derive(Serialize, Debug)]
struct CommonParams {
    binary: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MergedDepthParams {
    binary: bool,
    dump_scale: usize,
}

#[derive(Serialize, Debug)]
struct Subscription<P> {
    topic: String,
    event: String,
    symbol: String,
    params: P,
}

pub struct PublicWebSocketApiClient {
    uri: String,
    subscriptions: Vec<String>,
}

pub struct OnPublicResponse;
impl<'a> On<'a> for OnPublicResponse {
    type Arg = PublicResponse<'a>;
}

impl PublicWebSocketApiClient {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> PublicWebSocketApiClientBuilder {
        PublicWebSocketApiClientBuilder::default()
    }

    pub fn subscribe_trade<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = Subscription {
            topic: "trade".into(),
            event: "sub".into(),
            symbol: symbol.as_ref().to_owned(),
            params: CommonParams { binary },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_realtimes<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = Subscription {
            topic: "realtimes".into(),
            event: "sub".into(),
            symbol: symbol.as_ref().to_owned(),
            params: CommonParams { binary },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_kline<S: AsRef<str>>(&mut self, symbol: S, kline_type: &str, binary: bool) {
        let subscription = Subscription {
            topic: format!("kline_{}", kline_type),
            event: "sub".into(),
            symbol: symbol.as_ref().to_owned(),
            params: CommonParams { binary },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_depth<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = Subscription {
            topic: "depth".into(),
            event: "sub".into(),
            symbol: symbol.as_ref().to_owned(),
            params: CommonParams { binary },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_merged_depth<S: AsRef<str>>(
        &mut self,
        symbol: S,
        binary: bool,
        dump_scale: usize,
    ) {
        let subscription = Subscription {
            topic: "mergedDepth".into(),
            event: "sub".into(),
            symbol: symbol.as_ref().to_owned(),
            params: MergedDepthParams { binary, dump_scale },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_diff_depth<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = Subscription {
            topic: "diffDepth".into(),
            event: "sub".into(),
            symbol: symbol.as_ref().to_owned(),
            params: CommonParams { binary },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_lt<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = Subscription {
            topic: "lt".into(),
            event: "sub".into(),
            symbol: symbol.as_ref().to_owned(),
            params: CommonParams { binary },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn run<T: Callback<OnPublicResponse>>(&self, callback: T) -> Result<()> {
        run(&self.uri, None, &self.subscriptions, spot_ping, callback)
    }
}

pub struct PublicWebSocketApiClientBuilder {
    uri: String,
}

impl Default for PublicWebSocketApiClientBuilder {
    fn default() -> Self {
        Self {
            uri: MAINNET_PUBLIC.to_string(),
        }
    }
}

impl PublicWebSocketApiClientBuilder {
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET_PUBLIC.to_string();
        self
    }

    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
        self
    }

    pub fn build(self) -> PublicWebSocketApiClient {
        PublicWebSocketApiClient {
            uri: self.uri,
            subscriptions: Vec::new(),
        }
    }
}

#[derive(Serialize, Debug)]
struct CommonParamsV2 {
    binary: bool,
    symbol: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct KlineParamsV2 {
    binary: bool,
    symbol: String,
    kline_type: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepthParamsV2 {
    binary: bool,
    symbol: String,
    symbol_name: String,
}
#[derive(Serialize, Debug)]
struct SubscriptionV2<P> {
    topic: String,
    event: String,
    params: P,
}

pub struct PublicV2WebSocketApiClient {
    pub uri: String,
    subscriptions: Vec<String>,
}

impl PublicV2WebSocketApiClient {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> PublicV2WebSocketApiClientBuilder {
        PublicV2WebSocketApiClientBuilder::default()
    }

    pub fn subscribe_depth<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = SubscriptionV2 {
            topic: "depth".into(),
            event: "sub".into(),
            params: CommonParamsV2 {
                binary,
                symbol: symbol.as_ref().to_owned(),
            },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_kline<S: AsRef<str>>(&mut self, symbol: S, binary: bool, kline_type: S) {
        let subscription = SubscriptionV2 {
            topic: "kline".into(),
            event: "sub".into(),
            params: KlineParamsV2 {
                binary,
                symbol: symbol.as_ref().to_owned(),
                kline_type: kline_type.as_ref().to_owned(),
            },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_trade<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = SubscriptionV2 {
            topic: "trade".into(),
            event: "sub".into(),
            params: CommonParamsV2 {
                binary,
                symbol: symbol.as_ref().to_owned(),
            },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_book_ticker<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = SubscriptionV2 {
            topic: "bookTicker".into(),
            event: "sub".into(),
            params: CommonParamsV2 {
                binary,
                symbol: symbol.as_ref().to_owned(),
            },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn subscribe_realtimes<S: AsRef<str>>(&mut self, symbol: S, binary: bool) {
        let subscription = SubscriptionV2 {
            topic: "realtimes".into(),
            event: "sub".into(),
            params: CommonParamsV2 {
                binary,
                symbol: symbol.as_ref().to_owned(),
            },
        };
        self.subscriptions
            .push(serde_json::to_string(&subscription).unwrap());
    }

    pub fn run<T: Callback<OnPublicV2Response>>(&self, callback: T) -> Result<()> {
        run(&self.uri, None, &self.subscriptions, spot_ping, callback)
    }
}

pub struct PublicV2WebSocketApiClientBuilder {
    uri: String,
}

impl Default for PublicV2WebSocketApiClientBuilder {
    fn default() -> Self {
        Self {
            uri: MAINNET_PUBLIC_V2.to_string(),
        }
    }
}

impl PublicV2WebSocketApiClientBuilder {
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET_PUBLIC_V2.to_string();
        self
    }

    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
        self
    }

    pub fn build(self) -> PublicV2WebSocketApiClient {
        PublicV2WebSocketApiClient {
            uri: self.uri,
            subscriptions: Vec::new(),
        }
    }
}

#[derive(Serialize)]
struct AuthReq<'a> {
    op: &'a str,
    args: [&'a str; 3],
}

pub struct PrivateWebSocketApiClient {
    pub uri: String,
    pub api_key: String,
    pub secret: String,
}

impl PrivateWebSocketApiClient {
    pub fn new<S: AsRef<str>>(api_key: S, secret: S) -> Self {
        Self::builder()
            .build_with_credentials(api_key.as_ref().to_owned(), secret.as_ref().to_owned())
    }

    pub fn builder() -> PrivateWebSocketApiClientBuilder {
        PrivateWebSocketApiClientBuilder::default()
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
            spot_ping,
            callback,
        )
    }
}

pub struct PrivateWebSocketApiClientBuilder {
    uri: String,
}

impl Default for PrivateWebSocketApiClientBuilder {
    fn default() -> Self {
        Self {
            uri: MAINNET_PRIVATE.to_string(),
        }
    }
}

impl PrivateWebSocketApiClientBuilder {
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET_PRIVATE.to_string();
        self
    }

    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
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
        }
    }
}
