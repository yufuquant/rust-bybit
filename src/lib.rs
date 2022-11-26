pub mod error;
pub mod inverse;
pub mod linear;
pub mod spot;
pub mod util;

mod callback;

use callback::Callback;
use callback::OnAny;
use log::*;
use reqwest::Url;
use serde::Serialize;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::{sync::mpsc, thread, time::Duration};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message};

use crate::error::Result;
use crate::util::millseconds;
use crate::util::sign;

#[derive(Serialize)]
struct AuthReq<'a> {
    op: &'a str,
    args: [&'a str; 3],
}

struct Credentials {
    pub api_key: String,
    pub secret: String,
}

fn run<R, C>(
    uri: &str,
    credentials: Option<&Credentials>,
    subscriptions: &Vec<String>,
    ping_fn: fn() -> Receiver<String>,
    mut callback: C,
) -> Result<()>
where
    R: OnAny,
    C: Callback<R>,
{
    let req = Url::parse(uri)?;
    let (mut ws, _) = connect(req)?;
    match ws.get_mut() {
        MaybeTlsStream::NativeTls(t) => {
            t.get_mut()
                .set_read_timeout(Some(Duration::from_secs(15)))
                .expect("Error: cannot set read-timeout to underlying stream");
        }
        MaybeTlsStream::Plain(s) => {
            s.set_read_timeout(Some(Duration::from_secs(15)))
                .expect("Error: cannot set read-timeout to underlying stream");
        }
        _ => panic!("Error: it is not TlsStream"),
    }

    let rx = ping_fn();

    if let Some(credentials) = credentials {
        let expires = millseconds()? + 10000;
        let val = format!("GET/realtime{}", expires);
        let signature = sign(&credentials.secret, &val);
        let auth_req = AuthReq {
            op: "auth",
            args: [&credentials.api_key, &expires.to_string(), &signature],
        };
        ws.write_message(Message::Text(serde_json::to_string(&auth_req)?))?;
    }

    for subscription in subscriptions {
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
                    match serde_json::from_str(&content) {
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

fn spot_ping() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let s30 = Duration::from_secs(30);
        loop {
            if let Ok(ts) = millseconds() {
                tx.send(format!("{{\"ping\":{}}}", ts)).unwrap();
            }
            thread::sleep(s30);
        }
    });
    rx
}

fn future_ping() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let s30 = Duration::from_secs(30);
        loop {
            tx.send("{\"op\":\"ping\"}".into()).unwrap();
            thread::sleep(s30);
        }
    });
    rx
}
