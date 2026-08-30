"""
backtest_starter.py — a deliberately small, honest backtest harness.

Strategy: long when fast SMA > slow SMA, flat otherwise. Long-only, one stock at a time.
This is not a good strategy. It is a *correct* harness for testing strategies, which is
the thing worth having. Swap out `signal()` and keep everything else.

Design choices that matter (most beginner backtests get these wrong):
  - Signals are shifted by 1 bar. You cannot trade on a close you haven't seen yet.
  - Transaction costs are charged on turnover, not per trade.
  - Buy-and-hold is always reported alongside. A strategy that loses to buy-and-hold
    on the same asset over the same window has not found anything.
  - A parameter sweep is printed. If only one cell in the grid looks good, it's noise.

Install:
    pip install pandas numpy
    # market_data.py must sit in the same folder
    # optional fallback:  pip install yfinance

Run:
    python backtest_starter.py
"""

from __future__ import annotations

import itertools
import numpy as np
import pandas as pd

# ----------------------------- configuration -----------------------------

TICKERS = ["AAPL", "MSFT", "KO"]
START = "2008-01-01"
END = None            # None = today

FAST, SLOW = 20, 100  # SMA lengths in trading days
COST_BPS = 10.0       # round-trip cost in basis points of traded notional (10 bps = 0.10%)
ANNUAL = 252

# Sweep grid — used to check whether FAST/SLOW above is a lucky cell or a plateau.
SWEEP_FAST = [5, 10, 20, 40, 60]
SWEEP_SLOW = [50, 100, 150, 200, 250]


# ----------------------------- data loading ------------------------------

def load_prices(ticker: str, start=START, end=END) -> pd.Series:
    """Return a daily close price series.

    Neither Stooq nor yfinance is survivorship-bias-free: delisted tickers are
    simply absent. Keep that in mind before backtesting a basket you picked by
    looking at today's index.
    """
    from market_data import load_close
    return load_close(ticker, start=start, end=end)


# ------------------------------- strategy --------------------------------

def signal(px: pd.Series, fast: int, slow: int) -> pd.Series:
    """Target position in [0, 1]. Replace this function to test your own idea.

    The `.shift(1)` is the whole ballgame. Without it you are trading on information
    you did not have, and your equity curve will look wonderful and be a lie.
    """
    raw = (px.rolling(fast).mean() > px.rolling(slow).mean()).astype(float)
    return raw.shift(1).fillna(0.0)


# ------------------------------- engine ----------------------------------

def run(px: pd.Series, fast: int, slow: int, cost_bps: float = COST_BPS) -> pd.DataFrame:
    """Return a frame with strategy and benchmark returns, net of costs."""
    rets = px.pct_change().fillna(0.0)
    pos = signal(px, fast, slow)

    turnover = pos.diff().abs().fillna(pos.abs())
    costs = turnover * (cost_bps / 10_000.0)

    gross = pos * rets
    net = gross - costs

    return pd.DataFrame({
        "price": px,
        "position": pos,
        "bench_ret": rets,
        "strat_ret": net,
        "turnover": turnover,
    })


# ------------------------------- metrics ---------------------------------

def metrics(rets: pd.Series) -> dict:
    rets = rets.dropna()
    if len(rets) < ANNUAL or rets.std() == 0:
        return {"CAGR": np.nan, "Vol": np.nan, "Sharpe": np.nan,
                "MaxDD": np.nan, "Years": len(rets) / ANNUAL}

    equity = (1.0 + rets).cumprod()
    years = len(rets) / ANNUAL
    cagr = equity.iloc[-1] ** (1.0 / years) - 1.0
    vol = rets.std() * np.sqrt(ANNUAL)
    sharpe = (rets.mean() * ANNUAL) / vol           # excess-of-zero, not excess-of-cash
    dd = (equity / equity.cummax() - 1.0).min()

    return {"CAGR": cagr, "Vol": vol, "Sharpe": sharpe, "MaxDD": dd, "Years": years}


def fmt(m: dict) -> str:
    return (f"CAGR {m['CAGR']:>7.2%}   Vol {m['Vol']:>6.2%}   "
            f"Sharpe {m['Sharpe']:>5.2f}   MaxDD {m['MaxDD']:>7.2%}")


# ------------------------------- reporting -------------------------------

def report(ticker: str, px: pd.Series) -> None:
    bt = run(px, FAST, SLOW)
    strat, bench = metrics(bt["strat_ret"]), metrics(bt["bench_ret"])

    trades = int((bt["position"].diff().abs() > 0).sum())
    exposure = bt["position"].mean()

    print(f"\n{'=' * 74}")
    print(f"{ticker}   {px.index[0].date()} to {px.index[-1].date()}   "
          f"({strat['Years']:.1f} years)")
    print("=" * 74)
    print(f"  SMA {FAST}/{SLOW}   {fmt(strat)}")
    print(f"  Buy & hold      {fmt(bench)}")
    print(f"  {trades} position changes, {exposure:.0%} time in market, "
          f"{COST_BPS:.0f} bps per turn")

    verdict = "beats" if strat["Sharpe"] > bench["Sharpe"] else "LOSES TO"
    print(f"  -> strategy {verdict} buy-and-hold on Sharpe")

    # Parameter sweep. You want a broad region of decent Sharpes, not one hot cell
    # surrounded by garbage. A lone hot cell is overfitting with extra steps.
    print(f"\n  Sharpe across parameters (rows = fast, cols = slow):")
    grid = pd.DataFrame(index=SWEEP_FAST, columns=SWEEP_SLOW, dtype=float)
    for f, s in itertools.product(SWEEP_FAST, SWEEP_SLOW):
        if f >= s:
            continue
        grid.loc[f, s] = metrics(run(px, f, s)["strat_ret"])["Sharpe"]
    print(grid.to_string(float_format=lambda v: f"{v:6.2f}", na_rep="     -"))

    good = grid.stack()
    if len(good):
        print(f"  median Sharpe across grid: {good.median():.2f}  "
              f"(vs {bench['Sharpe']:.2f} buy-and-hold)")


def main() -> None:
    print("Loading data...")
    for ticker in TICKERS:
        try:
            px = load_prices(ticker)
        except Exception as exc:  # noqa: BLE001
            print(f"skipping {ticker}: {exc}")
            continue
        report(ticker, px)

    print(f"\n{'=' * 74}")
    print("Reminders before you believe any of the above:")
    print("  1. You chose these tickers knowing how they turned out. That is a bias.")
    print("  2. Costs here are a guess. Use your broker's real ones.")
    print("  3. One good backtest out of twenty tried is not evidence. Count your tries.")
    print("  4. Reserve the last 2-3 years of data and don't look at it until the end.")


if __name__ == "__main__":
    main()
