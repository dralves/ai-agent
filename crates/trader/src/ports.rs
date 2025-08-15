use crate::domain::*;
use async_trait::async_trait;
use futures_core::Stream;
use std::pin::Pin;

pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;

#[async_trait]
pub trait MarketDataSource: Send + Sync {
    async fn fetch_historical_candles(
        &self,
        symbol: &str,
        interval_secs: u64,
        limit: usize,
    ) -> anyhow::Result<Vec<Candle>>;

    async fn stream_candles<'a>(
        &'a self,
        symbol: &str,
        interval_secs: u64,
    ) -> anyhow::Result<BoxStream<'a, Candle>>;

    async fn stream_ticks<'a>(&'a self, symbol: &str) -> anyhow::Result<BoxStream<'a, Tick>>;
}

#[async_trait]
pub trait NewsSource: Send + Sync {
    async fn poll_headlines<'a>(&'a self) -> anyhow::Result<BoxStream<'a, String>>;
}

#[async_trait]
pub trait SentimentAnalyzer: Send + Sync {
    async fn score(&self, text: &str) -> anyhow::Result<f64>; // -1..1
}

#[async_trait]
pub trait TechnicalAnalyzer: Send + Sync {
    async fn update_with_candle(&self, candle: &Candle) -> anyhow::Result<()>;
    async fn current_signals(&self, symbol: &str) -> anyhow::Result<TechnicalSignals>;
}

#[async_trait]
pub trait DecisionEngine: Send + Sync {
    async fn decide(
        &self,
        symbol: &str,
        price: f64,
        tech: &TechnicalSignals,
        sentiment: &SentimentSignals,
        portfolio: &Portfolio,
    ) -> anyhow::Result<Decision>;
}

#[async_trait]
pub trait RiskManager: Send + Sync {
    async fn review_order(&self, desired: &Decision, portfolio: &Portfolio) -> anyhow::Result<Decision>;
}

#[async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, decision: &Decision, price: f64) -> anyhow::Result<Option<Trade>>;
}

#[async_trait]
pub trait PortfolioStore: Send + Sync {
    async fn load(&self) -> anyhow::Result<Portfolio>;
    async fn save(&self, portfolio: &Portfolio) -> anyhow::Result<()>;
    async fn record_trade(&self, trade: &Trade) -> anyhow::Result<()>;
}


