use bybit::ws::response::{OrderbookItem, SpotPublicResponse};
use bybit::ws::spot;
use bybit::WebSocketApiClient;
use std::io::{self, Write};

struct OwnedOrderBookItem(String, String);

impl<'a> From<&OrderbookItem<'a>> for OwnedOrderBookItem {
    fn from(value: &OrderbookItem) -> Self {
        OwnedOrderBookItem(value.0.to_owned(), value.1.to_owned())
    }
}

fn main() {
    let mut client = WebSocketApiClient::spot().build();

    let symbol = "ETHUSDT";

    client.subscribe_trade(symbol);
    client.subscribe_orderbook(symbol, spot::OrderbookDepth::Level50);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let mut latest_price: String = String::new();
    let mut direction = "△";
    let mut asks: Vec<OwnedOrderBookItem> = Vec::new();
    let mut bids: Vec<OwnedOrderBookItem> = Vec::new();

    let callback = |res: SpotPublicResponse| {
        match res {
            SpotPublicResponse::Trade(res) => {
                let price = res.data[0].p.to_owned();
                if price < latest_price {
                    direction = "▽";
                } else if price > latest_price {
                    direction = "△";
                }
                latest_price = price
            }
            SpotPublicResponse::Orderbook(res) => {
                // > Once you have subscribed successfully, you will receive a snapshot.
                // > If you receive a new snapshot message, you will have to reset your local orderbook.
                if res.type_ == "snapshot" {
                    asks = res.data.a.iter().map(|item| item.into()).collect();
                    bids = res.data.b.iter().map(|item| item.into()).collect();
                    return;
                }

                // Receive a delta message, update the orderbook.
                // Note that asks and bids of a delta message **do not guarantee** to be ordered.

                // process asks
                let a = &res.data.a;
                let mut i: usize = 0;

                while i < a.len() {
                    let OrderbookItem(price, qty) = a[i];

                    let mut j: usize = 0;
                    while j < asks.len() {
                        let item = &mut asks[j];
                        let item_price: &str = &item.0;

                        if price < item_price {
                            asks.insert(j, OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                            break;
                        }

                        if price == item_price {
                            if qty != "0" {
                                item.1 = qty.to_owned();
                            } else {
                                asks.remove(j);
                            }
                            break;
                        }

                        j += 1;
                    }

                    if j == asks.len() {
                        asks.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()))
                    }

                    i += 1;
                }

                // process bids
                let b = &res.data.b;
                let mut i: usize = 0;

                while i < b.len() {
                    let OrderbookItem(price, qty) = b[i];

                    let mut j: usize = 0;
                    while j < bids.len() {
                        let item = &mut bids[j];
                        let item_price: &str = &item.0;
                        if price > item_price {
                            bids.insert(j, OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                            break;
                        }

                        if price == item_price {
                            if qty != "0" {
                                item.1 = qty.to_owned();
                            } else {
                                bids.remove(j);
                            }
                            break;
                        }

                        j += 1;
                    }

                    if j == bids.len() {
                        bids.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                    }

                    i += 1;
                }
            }
            _ => {}
        }

        write!(handle, "\x1B[2J\x1B[1;1H").unwrap();
        write!(handle, "ETHUSDT/USDT\n\n").unwrap();
        write!(handle, "{:<20} {:<20}\n", "Price(USDT)", "Quantity(ETH)").unwrap();
        let mut asks10 = asks.iter().take(10).collect::<Vec<_>>().clone();
        asks10.reverse();
        asks10.iter().for_each(|item| {
            write!(handle, "{:<20} {:<20}\n", item.0, item.1).unwrap();
        });
        write!(handle, "\n{} {}\n\n", direction, latest_price).unwrap();
        bids.iter().take(10).for_each(|item| {
            write!(handle, "{:<20} {:<20}\n", item.0, item.1).unwrap();
        });
        handle.flush().unwrap();
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
