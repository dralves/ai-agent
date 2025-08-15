use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub type Symbol = String;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub ts: SystemTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub symbol: Symbol,
    pub interval_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub ts: SystemTime,
    pub price: f64,
    pub volume: f64,
    pub symbol: Symbol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: Symbol,
    pub side: Side,
    pub quantity: f64,
    pub order_type: OrderType,
    pub price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub ts: SystemTime,
    pub symbol: Symbol,
    pub side: Side,
    pub price: f64,
    pub quantity: f64,
    pub fee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    pub symbol: Symbol,
    pub quantity: f64,
    pub avg_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Portfolio {
    pub cash: f64,
    pub positions: Vec<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalSignals {
    pub rsi: Option<f64>,
    pub ema_short: Option<f64>,
    pub ema_long: Option<f64>,
    pub bollinger_upper: Option<f64>,
    pub bollinger_lower: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentSignals {
    pub score: f64, // -1..1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Decision {
    Buy { symbol: Symbol, fraction_of_cash: f64 },
    SellAll { symbol: Symbol },
    Hold,
}


