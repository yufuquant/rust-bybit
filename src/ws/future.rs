use super::callback::Callback;
use super::response::FuturePublicResponseArg;
use super::run;
use super::Subscriber;
use crate::error::Result;
use crate::{FutureRole, KlineInterval};

const MAINNET_LINEAR: &str = "wss://stream.bybit.com/v5/public/linear";
const MAINNET_INVERSE: &str = "wss://stream.bybit.com/v5/public/inverse";
const TESTNET_LINEAR: &str = "wss://stream-testnet.bybit.com/v5/public/linear";
const TESTNET_INVERSE: &str = "wss://stream-testnet.bybit.com/v5/public/inverse";

pub enum OrderbookDepth {
    Level1,
    Level50,
    Level200,
    Level500,
}

impl From<OrderbookDepth> for u16 {
    fn from(value: OrderbookDepth) -> Self {
        use OrderbookDepth::*;
        match value {
            Level1 => 1,
            Level50 => 50,
            Level200 => 200,
            Level500 => 500,
        }
    }
}

pub struct FutureWebsocketApiClient {
    uri: String,
    subscriber: Subscriber,
}

impl FutureWebsocketApiClient {
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

    pub fn subscribe_liquidation<S: AsRef<str>>(&mut self, symbol: S) {
        self.subscriber.sub_liquidation(symbol.as_ref());
    }

    pub fn run<C: Callback<FuturePublicResponseArg>>(&self, callback: C) -> Result<()> {
        run(&self.uri, self.subscriber.topics(), None, callback)
    }
}

pub struct FutureWebSocketApiClientBuilder {
    uri: String,
    role: FutureRole,
}

impl FutureWebSocketApiClientBuilder {
    /// Create a new `FutureWebSocketApiClientBuilder`. Channel URI is set to the mainnet.
    pub fn new(role: FutureRole) -> Self {
        let uri = match role {
            FutureRole::Linear => MAINNET_LINEAR.to_string(),
            FutureRole::Inverse => MAINNET_INVERSE.to_string(),
        };
        Self { uri, role }
    }

    /// Change channel URI to the testnet.
    pub fn testnet(mut self) -> Self {
        self.uri = match self.role {
            FutureRole::Linear => TESTNET_LINEAR.to_string(),
            FutureRole::Inverse => TESTNET_INVERSE.to_string(),
        };
        self
    }

    /// Set channel URI to the URI specified.
    ///
    /// Note URI should **match** with api client kind.
    /// Do not set a spot channel URI to a future api client.
    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
        self
    }

    /// Build a future websocket api client.
    pub fn build(self) -> FutureWebsocketApiClient {
        FutureWebsocketApiClient {
            uri: self.uri,
            subscriber: Subscriber::new(),
        }
    }
}
