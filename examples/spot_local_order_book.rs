use bybit::spot::ws::{OrderBookItem, PublicResponse, PublicWebSocketApiClient};
use std::io::{self, Write};

struct OwnedOrderBookItem(String, String);

fn main() {
    let mut client = PublicWebSocketApiClient::new();

    client.subscribe_trade("BTCUSDT", false);
    client.subscribe_diff_depth("BTCUSDT", false);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let mut latest_price: String = String::new();
    let mut direction = "△";
    let mut asks: Vec<OwnedOrderBookItem> = Vec::new();
    let mut bids: Vec<OwnedOrderBookItem> = Vec::new();

    let callback = |res: PublicResponse| {
        match res {
            PublicResponse::Trade(res) => {
                let price = res.data[0].p.to_owned();
                if price < latest_price {
                    direction = "▽";
                } else if price > latest_price {
                    direction = "△";
                }
                latest_price = price
            }
            PublicResponse::Depth(res) => {
                res.data[0].a.iter().for_each(|&OrderBookItem(price, qty)| {
                    asks.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                });
                res.data[0].b.iter().for_each(|&OrderBookItem(price, qty)| {
                    bids.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                });
            }
            PublicResponse::DiffDepth(res) => {
                // process asks
                let a = &res.data[0].a;
                let mut i: usize = 0;
                let mut j: usize = 0;

                while i < a.len() {
                    let OrderBookItem(price, qty) = a[i];

                    while j < asks.len() {
                        let item = &mut asks[j];
                        let item_price: &str = &item.0;
                        if price < item_price {
                            asks.insert(j, OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                            i += 1;
                            j += 1;
                            break;
                        }

                        if price == item_price {
                            if qty != "0" {
                                item.1 = qty.to_owned();
                                i += 1;
                                j += 1;
                            } else {
                                asks.remove(j);
                                i += 1;
                            }
                            break;
                        }

                        j += 1;
                    }

                    if j == asks.len() {
                        a.iter().skip(i).for_each(|&OrderBookItem(price, qty)| {
                            asks.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                        });
                        break;
                    }
                }

                // process bids
                let b = &res.data[0].b;
                let mut i: usize = 0;
                let mut j: usize = 0;

                while i < b.len() {
                    let OrderBookItem(price, qty) = b[i];

                    while j < bids.len() {
                        let item = &mut bids[j];
                        let item_price: &str = &item.0;
                        if price > item_price {
                            bids.insert(j, OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                            i += 1;
                            j += 1;
                            break;
                        }

                        if price == item_price {
                            if qty != "0" {
                                item.1 = qty.to_owned();
                                i += 1;
                                j += 1;
                            } else {
                                bids.remove(j);
                                i += 1;
                            }
                            break;
                        }

                        j += 1;
                    }

                    if j == bids.len() {
                        b.iter().skip(i).for_each(|&OrderBookItem(price, qty)| {
                            bids.push(OwnedOrderBookItem(price.to_owned(), qty.to_owned()));
                        });
                        break;
                    }
                }
            }
            _ => {}
        }

        write!(handle, "\x1B[2J\x1B[1;1H").unwrap();
        write!(handle, "BTC/USDT OrderBook\n\n").unwrap();
        write!(handle, "{:<20} {:<20}\n", "Price(USDT)", "Quantity(BTC)").unwrap();
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
        Err(e) => println!("{}", e),
    }
}
