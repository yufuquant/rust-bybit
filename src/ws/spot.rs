use super::callback::Callback;
use super::response::SpotPublicResponseArg;
use super::run;
use super::Subscriber;
use crate::error::Result;
use crate::KlineInterval;

const MAINNET_SPOT: &str = "wss://stream.bybit.com/v5/public/spot";
const TESTNET_SPOT: &str = "wss://stream-testnet.bybit.com/v5/public/spot";

pub enum OrderbookDepth {
    Level1,
    Level50,
}

impl From<OrderbookDepth> for u16 {
    fn from(value: OrderbookDepth) -> Self {
        use OrderbookDepth::*;
        match value {
            Level1 => 1,
            Level50 => 50,
        }
    }
}

pub struct SpotWebsocketApiClient {
    uri: String,
    subscriber: Subscriber,
}

impl SpotWebsocketApiClient {
    pub fn subscribe_orderbook<S: AsRef<str>>(&mut self, symbol: S, depth: OrderbookDepth) {
        self.subscriber.sub_orderbook(symbol.as_ref(), depth.into());
    }

    pub fn subscribe_trade<S: AsRef<str>>(&mut self, symbol: S) {
        self.subscriber.sub_trade(symbol.as_ref());
    }

    pub fn subscribe_ticker<S: AsRef<str>>(&mut self, symbol: S) {
        self.subscriber.sub_ticker(symbol.as_ref());
    }

    pub fn subscribe_kline<S: AsRef<str>>(&mut self, symbol: S, interval: KlineInterval) {
        self.subscriber.sub_kline(symbol.as_ref(), interval.into());
    }

    pub fn subscribe_lt_kline<S: AsRef<str>>(&mut self, symbol: S, interval: KlineInterval) {
        self.subscriber
            .sub_lt_kline(symbol.as_ref(), interval.into());
    }

    pub fn subscribe_lt_ticker<S: AsRef<str>>(&mut self, symbol: S) {
        self.subscriber.sub_lt_ticker(symbol.as_ref());
    }

    pub fn subscribe_lt_nav<S: AsRef<str>>(&mut self, symbol: S) {
        self.subscriber.sub_lt_nav(symbol.as_ref());
    }

    pub fn run<C: Callback<SpotPublicResponseArg>>(&self, callback: C) -> Result<()> {
        run(&self.uri, self.subscriber.topics(), None, callback)
    }
}

pub struct SpotWebSocketApiClientBuilder {
    uri: String,
}

impl SpotWebSocketApiClientBuilder {
    /// Create a new `SpotWebSocketApiClientBuilder`. Channel URI is set to the mainnet.
    pub fn new() -> Self {
        Self {
            uri: MAINNET_SPOT.to_string(),
        }
    }

    /// Change channel URI to the testnet.
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET_SPOT.to_string();
        self
    }

    /// Set channel URI to the URI specified.
    ///
    /// Note URI should **match** with api client kind.
    /// Do not set a future channel URI to a spot api client.
    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
        self
    }

    /// Build a spot websocket api client.
    pub fn build(self) -> SpotWebsocketApiClient {
        SpotWebsocketApiClient {
            uri: self.uri,
            subscriber: Subscriber::new(),
        }
    }
}
