use super::callback::Callback;
use super::response::OptionPublicResponseArg;
use super::run;
use super::Subscriber;
use crate::error::Result;

const MAINNET_OPTION: &str = "wss://stream.bybit.com/v5/public/option";
const TESTNET_OPTION: &str = "wss://stream-testnet.bybit.com/v5/public/option";

pub enum OrderbookDepth {
    Level25,
    Level100,
}

impl From<OrderbookDepth> for u16 {
    fn from(value: OrderbookDepth) -> Self {
        use OrderbookDepth::*;
        match value {
            Level25 => 25,
            Level100 => 100,
        }
    }
}

pub struct OptionWebsocketApiClient {
    uri: String,
    subscriber: Subscriber,
}

impl OptionWebsocketApiClient {
    pub fn subscribe_orderbook<S: AsRef<str>>(&mut self, symbol: S, depth: OrderbookDepth) {
        self.subscriber.sub_orderbook(symbol.as_ref(), depth.into());
    }

    /// Subscribe to recent trades.
    ///
    /// Note that option uses the base coin, e.g., BTC.
    pub fn subscribe_trade<S: AsRef<str>>(&mut self, base_coin: S) {
        self.subscriber.sub_trade(base_coin.as_ref());
    }

    pub fn subscribe_ticker<S: AsRef<str>>(&mut self, symbol: S) {
        self.subscriber.sub_ticker(symbol.as_ref());
    }

    pub fn run<C: Callback<OptionPublicResponseArg>>(&self, callback: C) -> Result<()> {
        run(&self.uri, self.subscriber.topics(), None, callback)
    }
}

pub struct OptionWebSocketApiClientBuilder {
    uri: String,
}

impl OptionWebSocketApiClientBuilder {
    /// Create a new `OptionWebSocketApiClientBuilder`. Channel URI is set to the mainnet.
    pub fn new() -> Self {
        Self {
            uri: MAINNET_OPTION.to_string(),
        }
    }

    /// Change channel URI to the testnet.
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET_OPTION.to_string();
        self
    }

    /// Set channel URI to the URI specified.
    ///
    /// Note URI should **match** with api client kind.
    /// Do not set a spot channel URI to a option api client.
    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
        self
    }

    /// Build a option websocket api client.
    pub fn build(self) -> OptionWebsocketApiClient {
        OptionWebsocketApiClient {
            uri: self.uri,
            subscriber: Subscriber::new(),
        }
    }
}
