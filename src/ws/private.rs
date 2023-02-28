use super::callback::Callback;
use super::response::PrivateResponseArg;
use super::Subscriber;
use super::{run, Credentials};
use crate::error::Result;

const MAINNET_PRIVATE: &str = "wss://stream.bybit.com/v5/private";
const TESTNET_PRIVATE: &str = "wss://stream-testnet.bybit.com/v5/private";

pub struct PrivateWebsocketApiClient {
    uri: String,
    subscriber: Subscriber,
    credentials: Credentials,
}

impl PrivateWebsocketApiClient {
    pub fn subscribe_position(&mut self) {
        self.subscriber.sub_position();
    }

    pub fn subscribe_order(&mut self) {
        self.subscriber.sub_order();
    }

    pub fn subscribe_wallet(&mut self) {
        self.subscriber.sub_wallet();
    }

    pub fn subscribe_execution(&mut self) {
        self.subscriber.sub_execution();
    }

    pub fn subscribe_greek(&mut self) {
        self.subscriber.sub_greek();
    }

    pub fn run<C: Callback<PrivateResponseArg>>(&self, callback: C) -> Result<()> {
        run(
            &self.uri,
            self.subscriber.topics(),
            Some(&self.credentials),
            callback,
        )
    }
}

pub struct PrivateWebSocketApiClientBuilder {
    uri: String,
}

impl PrivateWebSocketApiClientBuilder {
    /// Create a new `PrivateWebSocketApiClientBuilder`. Channel URI is set to the mainnet.
    pub fn new() -> Self {
        Self {
            uri: MAINNET_PRIVATE.to_string(),
        }
    }

    /// Change channel URI to the testnet.
    pub fn testnet(mut self) -> Self {
        self.uri = TESTNET_PRIVATE.to_string();
        self
    }

    /// Set channel URI to the URI specified.
    ///
    /// Note URI should **match** with api client kind.
    /// Do not set a spot channel URI to a private api client.
    pub fn uri<S: AsRef<str>>(mut self, uri: S) -> Self {
        self.uri = uri.as_ref().to_owned();
        self
    }

    /// Build a private websocket api client with api key and secret key.
    pub fn build_with_credentials<S: AsRef<str>>(
        self,
        api_key: S,
        secret: S,
    ) -> PrivateWebsocketApiClient {
        PrivateWebsocketApiClient {
            uri: self.uri,
            subscriber: Subscriber::new(),
            credentials: Credentials {
                api_key: api_key.as_ref().to_owned(),
                secret: secret.as_ref().to_owned(),
            },
        }
    }
}
