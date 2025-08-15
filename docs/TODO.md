# AI Trading Agent – TODOs and Milestones

Legend: [ ] not started, [~] in progress, [x] done, (P0/P1/P2) priority

## Milestone M0 – Project Skeleton (P0)
- [x] Create Rust workspace `ai-trader` with primary crate `trader` and optional `sim` crate
- [x] Add base dependencies: `tokio`, `reqwest`, `serde`, `serde_json`, `tracing`, `thiserror`, `anyhow`, `dotenvy`, `clap`
- [x] Choose DB layer (Diesel vs SQLx). Start with SQLite for simplicity
- [x] Optional: Add Postgres support and docker-compose for local PG with pgvector
- [x] Configure `config` + `.env` for secrets and runtime flags (paper/live, exchange keys)
- [x] CI: format/lint (`rustfmt`, `clippy`) and a basic test job

Acceptance: `cargo test` passes; binary starts with `--help` (met)

## Milestone M1 – Architecture Scaffolding (P0)
- [ ] Define core traits (hexagonal ports):
  - [ ] `MarketDataSource` (subscribe klines/ticks, fetch historical)
  - [ ] `NewsSource` (poll/stream headlines)
  - [ ] `SentimentAnalyzer` (score -1..1)
  - [ ] `TechnicalAnalyzer` (update indicators, expose signals)
  - [ ] `DecisionEngine` (compute desired position/action)
  - [ ] `Executor` (paper/live execute, fees)
  - [ ] `RiskManager` (validate/scale/override orders)
  - [ ] `PortfolioStore` (balances, positions, trades)
- [ ] Define common domain types: `Candle`, `Tick`, `Order`, `Trade`, `Position`, `Side`, `Symbol`, `Decision`

Acceptance: Compiles with trait stubs and domain models

## Milestone M2 – Data Ingestion (P0)
- [ ] Exchange selection: Binance (spot) initial target
- [ ] REST historical klines (1m) adapter
- [ ] WebSocket live klines/ticker adapter
- [ ] In-memory ring buffer for recent candles (configurable length)
- [ ] Persist raw candles to SQLite (for backtests)

Acceptance: CLI can fetch N historical klines and stream live updates printing latest OHLC

## Milestone M3 – News Feed (P1)
- [ ] Implement RSS poller (e.g., Google News crypto RSS) with deduplication
- [ ] Optional: Twitter/X integration (feature-flag)
- [ ] Normalize `NewsItem { ts, source, title, url }`

Acceptance: CLI subcommand prints last K headlines every minute

## Milestone M4 – Technical Analysis (P0)
- [ ] Implement indicators:
  - [ ] RSI(14)
  - [ ] EMA(50), EMA(200)
  - [ ] Bollinger Bands(20, 2σ)
- [ ] Emit technical signals (e.g., `Oversold`, `Overbought`, `TrendUp`, `BandBreak`)
- [ ] Validate against TradingView/Binance charts (log comparison)

Acceptance: Unit tests on indicator math and rolling window updates

## Milestone M5 – Sentiment Analysis (P1)
- [ ] Integrate llama.cpp bindings crate
- [ ] Minimal prompt to score single headline -1..1
- [ ] Rate-limit/batch analysis to control latency
- [ ] Fallback analyzer (simple lexicon) behind feature flag
- [ ] Expose rolling sentiment average over last K headlines

Acceptance: Given sample headlines, scores match intuition (positive>0, negative<0)

## Milestone M6 – Decision Engine v1 (P0)
- [ ] Rule-based strategy:
  - [ ] Enter long when `RSI < 30` AND `sentiment > 0`
  - [ ] Exit/flat when `RSI > 70` OR `sentiment < 0`
- [ ] State-awareness (in-position vs flat)
- [ ] Parameterize thresholds in config

Acceptance: Dry-run prints decisions with inputs (RSI, sentiment, price)

## Milestone M7 – Paper Execution & Portfolio Sim (P0)
- [ ] Portfolio model (cash, holdings, equity, fees)
- [ ] Market order execution at latest price + fee (e.g., 10 bps)
- [ ] Trade log persistence (SQLite)
- [ ] Backtester: replay historical candles into strategy
- [ ] CLI: `paper run` (live), `paper backtest --from --to`

Acceptance: Backtest produces PnL curve and trade list CSV

## Milestone M8 – Risk Management (P0)
- [ ] Per-trade stop-loss and take-profit
- [ ] Position sizing: fixed fraction of equity; config min/max
- [ ] Daily loss limit and kill-switch
- [ ] Exposure guard across symbols (when multi-asset)

Acceptance: Sim shows stops firing correctly; daily loss limit halts trading

## Milestone M9 – Monitoring & Observability (P1)
- [ ] Structured logging with `tracing`
- [ ] Metrics (PnL, drawdown, win rate) computed from trades
- [ ] Optional Prometheus exporter or lightweight web dashboard
- [ ] Alerting hooks (email/webhook) for failures and thresholds

Acceptance: Metrics endpoint/web report shows latest performance

## Milestone M10 – Exchange Execution (Testnet) (P1)
- [ ] Define `Exchange` trait and implement `BinanceExchange`
- [ ] Auth via API keys (env/config secrets)
- [ ] Market/limit orders; handle partial fills and retries
- [ ] Testnet toggle and sandbox trading

Acceptance: Place/cancel small test orders on testnet via CLI

## Milestone M11 – Live Trading (Small) (P2)
- [ ] Runbook and safeguards reviewed
- [ ] Deploy to small VM; process supervision
- [ ] Alerting verified in live conditions

Acceptance: Live trades executed with tiny capital and monitored

## Milestone M12 – Iteration & Scaling (P2)
- [ ] Hyperparameter tuning (e.g., thresholds) on backtests
- [ ] Add more indicators/data sources if beneficial
- [ ] Multi-asset, multi-strategy support (independent agents)
- [ ] Research track: supervised model and RL environment

Acceptance: Documented improvements with ablation results

---

## Backlog
- [ ] On-chain signals (whale alerts, liquidations)
- [ ] Latency optimization (HTTP pooling, websockets tuning)
- [ ] GPU/quantized model variants for sentiment
- [ ] Web UI (Actix/Axum + SQLite) for monitoring
- [ ] Broker adapters for equities (Alpaca/IBKR)

## Definitions
- **Paper mode**: All orders executed in simulator with fees and no slippage initially
- **PnL curve**: Time series of portfolio equity; include drawdown stats
- **Kill-switch**: Immediate halt when daily loss > threshold or critical errors


