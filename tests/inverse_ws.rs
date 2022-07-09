use std::error::Error;

use bybit::{
    self,
    inverse::ws::{
        Execution, ExecutionMessage, Order, OrderMessage, OrderSide, Position, PositionMessage,
        PositionSide, StopOrder, StopOrderMessage, Wallet, WalletMessage,
    },
};

#[test]
fn deserialize_position_message() -> Result<(), Box<dyn Error>> {
    let message = r#"
    {
        "topic": "position",
        "data": [{
            "user_id": 12345,
            "symbol": "ETHUSD",
            "size": 0,
            "side": "None",
            "position_value": "0",
            "entry_price": "0",
            "liq_price": "0",
            "bust_price": "0",
            "leverage": "10",
            "order_margin": "0.00008362",
            "position_margin": "0",
            "available_balance": "0.11003042",
            "take_profit": "0",
            "stop_loss": "0",
            "realised_pnl": "0.00280582",
            "trailing_stop": "0",
            "trailing_active": "0",
            "wallet_balance": "0.11011404",
            "risk_id": 11,
            "occ_closing_fee": "0",
            "occ_funding_fee": "0",
            "auto_add_margin": 1,
            "cum_realised_pnl": "0.01511404",
            "position_status": "Normal",
            "position_seq": 0,
            "Isolated": false,
            "mode": 0,
            "position_idx": 0,
            "tp_sl_mode": "Full",
            "tp_order_num": 0,
            "sl_order_num": 0,
            "tp_free_size_x": 0,
            "sl_free_size_x": 0
        }]
    }
    "#;

    let expected: PositionMessage = PositionMessage {
        topic: "position",
        data: vec![Position {
            user_id: 12345,
            symbol: "ETHUSD",
            size: 0,
            side: PositionSide::None,
            position_value: "0",
            entry_price: "0",
            liq_price: "0",
            bust_price: "0",
            leverage: "10",
            order_margin: "0.00008362",
            position_margin: "0",
            available_balance: "0.11003042",
            take_profit: "0",
            stop_loss: "0",
            realised_pnl: "0.00280582",
            trailing_stop: "0",
            trailing_active: "0",
            wallet_balance: "0.11011404",
            risk_id: 11,
            occ_closing_fee: "0",
            occ_funding_fee: "0",
            auto_add_margin: 1,
            cum_realised_pnl: "0.01511404",
            position_status: "Normal",
            position_seq: 0,
            isolated: false,
            mode: 0,
            position_idx: 0,
            tp_sl_mode: "Full",
            tp_order_num: 0,
            sl_order_num: 0,
            tp_free_size_x: 0,
            sl_free_size_x: 0,
        }],
    };

    let actual: PositionMessage = serde_json::from_str(message)?;
    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn deserialize_execution_message() -> Result<(), Box<dyn Error>> {
    let message = r#"
    {
        "topic": "execution",
        "data": [{
            "symbol": "ETHUSD",
            "side": "Buy",
            "order_id": "51d018c4-cda0-410d-8e37-0c7ea39aef8c",
            "exec_id": "8b0e0dfa-dc2a-5cca-b11a-dc940d613954",
            "order_link_id": "",
            "price": "1219.8",
            "order_qty": 1,
            "exec_type": "Trade",
            "exec_qty": 1,
            "exec_fee": "0.0000005",
            "leaves_qty": 0,
            "is_maker": false,
            "trade_time": "2022-07-09T21:17:25.693Z"
        }]
    }
    "#;

    let expected: ExecutionMessage = ExecutionMessage {
        topic: "execution",
        data: vec![Execution {
            symbol: "ETHUSD",
            side: OrderSide::Buy,
            order_id: "51d018c4-cda0-410d-8e37-0c7ea39aef8c",
            exec_id: "8b0e0dfa-dc2a-5cca-b11a-dc940d613954",
            order_link_id: "",
            price: 1219.8,
            order_qty: 1,
            exec_type: "Trade",
            exec_qty: 1,
            exec_fee: "0.0000005",
            leaves_qty: 0,
            is_maker: false,
            trade_time: "2022-07-09T21:17:25.693Z",
        }],
    };

    let actual: ExecutionMessage = serde_json::from_str(message)?;
    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn deserialize_order_message_message() -> Result<(), Box<dyn Error>> {
    let message = r#"
    {
        "topic": "order",
        "data": [{
            "order_id": "310d02f7-4454-49b0-a467-eff609b00362",
            "order_link_id": "",
            "symbol": "ETHUSD",
            "side": "Buy",
            "order_type": "Limit",
            "price": "1211",
            "qty": 1,
            "time_in_force": "PostOnly",
            "create_type": "CreateByUser",
            "cancel_type": "",
            "order_status": "New",
            "leaves_qty": 1,
            "cum_exec_qty": 0,
            "cum_exec_value": "0",
            "cum_exec_fee": "0",
            "timestamp": "2022-07-09T15:39:18.973Z",
            "take_profit": "0",
            "stop_loss": "0",
            "trailing_stop": "0",
            "last_exec_price": "0",
            "reduce_only": false,
            "close_on_trigger": false
        }]
    }
    "#;

    let expected: OrderMessage = OrderMessage {
        topic: "order",
        data: vec![Order {
            order_id: "310d02f7-4454-49b0-a467-eff609b00362",
            order_link_id: "",
            symbol: "ETHUSD",
            side: OrderSide::Buy,
            order_type: "Limit",
            price: 1211.,
            qty: 1,
            time_in_force: "PostOnly",
            create_type: "CreateByUser",
            cancel_type: "",
            order_status: "New",
            leaves_qty: 1,
            cum_exec_qty: 0,
            cum_exec_value: "0",
            cum_exec_fee: "0",
            timestamp: "2022-07-09T15:39:18.973Z",
            take_profit: "0",
            tp_trigger_by: None,
            stop_loss: "0",
            sl_trigger_by: None,
            trailing_stop: "0",
            last_exec_price: "0",
            reduce_only: false,
            close_on_trigger: false,
        }],
    };

    let actual: OrderMessage = serde_json::from_str(message)?;
    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn deserialize_stop_order_message() -> Result<(), Box<dyn Error>> {
    let message = r#"
    {
        "topic": "stop_order",
        "data": [{
            "order_id": "1362c33a-7194-4463-a58c-b674f3bd7f25",
            "order_link_id": "",
            "user_id": 28587642,
            "symbol": "ETHUSD",
            "side": "Buy",
            "order_type": "Market",
            "price": "0",
            "qty": 1,
            "time_in_force": "ImmediateOrCancel",
            "create_type": "CreateByStopLoss",
            "cancel_type": "",
            "order_status": "Untriggered",
            "stop_order_type": "StopLoss",
            "trigger_by": "LastPrice",
            "trigger_price": "1225.75",
            "timestamp": "2022-07-09T21: 17: 15.353Z",
            "close_on_trigger": true
        }]
    }
    "#;

    let expected: StopOrderMessage = StopOrderMessage {
        topic: "stop_order",
        data: vec![StopOrder {
            order_id: "1362c33a-7194-4463-a58c-b674f3bd7f25",
            order_link_id: "",
            user_id: 28587642,
            symbol: "ETHUSD",
            side: OrderSide::Buy,
            order_type: "Market",
            price: "0",
            qty: 1,
            time_in_force: "ImmediateOrCancel",
            create_type: "CreateByStopLoss",
            cancel_type: "",
            order_status: "Untriggered",
            stop_order_type: "StopLoss",
            trigger_by: "LastPrice",
            trigger_price: "1225.75",
            timestamp: "2022-07-09T21: 17: 15.353Z",
            close_on_trigger: true,
        }],
    };

    let actual: StopOrderMessage = serde_json::from_str(message)?;
    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn deserialize_wallet_message() -> Result<(), Box<dyn Error>> {
    let message = r#"
    {
        "topic": "wallet",
        "data": [{
            "user_id": 12345,
            "coin": "ETH",
            "wallet_balance": "0.11011404",
            "available_balance": "0.11003042"
        }]
    }
    "#;

    let expected: WalletMessage = WalletMessage {
        topic: "wallet",
        data: vec![Wallet {
            user_id: 12345,
            coin: "ETH",
            wallet_balance: 0.11011404,
            available_balance: 0.11003042,
        }],
    };

    let actual: WalletMessage = serde_json::from_str(message)?;
    assert_eq!(actual, expected);

    Ok(())
}
