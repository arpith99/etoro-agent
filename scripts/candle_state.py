"""
candle_state.py — implements and stress-tests the red/green state machine.

Rules as specified:
    red   -> green : close short   (flat)
    green -> green : buy           (long)
    green -> red   : open short    (short)
    red   -> red   : sell          (short)

This is a 1-2 day continuation strategy. Its entire edge, if any, comes from
positive autocorrelation in daily returns. This file measures that dependence
directly by sweeping synthetic AR(1) series, then lets you run it on real data.

Run:
    python candle_state.py            # synthetic sensitivity analysis
    python candle_state.py AAPL SAP.DE   # real data, if pandas-datareader installed
"""

from __future__ import annotations

import sys
import numpy as np
import pandas as pd

ANNUAL = 252


# ---------------------------- the strategy -------------------------------

def candle_position(close: pd.Series, open_: pd.Series | None = None,
                    body: bool = False) -> pd.Series:
    """Map the red/green state machine to a position in {-1, 0, +1}.

    body=True  -> green means close > open  (candle body)
    body=False -> green means close > previous close
    """
    if body:
        if open_ is None:
            raise ValueError("body=True needs an open series")
        green = (close > open_)
    else:
        green = (close > close.shift(1))

    prev = green.shift(1)

    pos = pd.Series(np.nan, index=close.index, dtype=float)
    pos[(~prev.fillna(False)) & green] = 0.0    # red -> green : flat
    pos[prev.fillna(False) & green] = 1.0       # green -> green : long
    pos[prev.fillna(False) & ~green] = -1.0     # green -> red : short
    pos[(~prev.fillna(False)) & (~green)] = -1.0  # red -> red : short

    return pos.fillna(0.0)


def run(close: pd.Series, open_=None, body=False, cost_bps=10.0) -> pd.Series:
    """Net daily returns. Position is shifted: you act after seeing the close."""
    rets = close.pct_change().fillna(0.0)
    pos = candle_position(close, open_, body).shift(1).fillna(0.0)
    turnover = pos.diff().abs().fillna(0.0)
    return pos * rets - turnover * (cost_bps / 10_000.0)


def stats(r: pd.Series) -> dict:
    r = r.dropna()
    if len(r) < 60 or r.std() == 0:
        return {"Sharpe": np.nan, "CAGR": np.nan, "MaxDD": np.nan}
    eq = (1 + r).cumprod()
    return {
        "Sharpe": (r.mean() * ANNUAL) / (r.std() * np.sqrt(ANNUAL)),
        "CAGR": eq.iloc[-1] ** (ANNUAL / len(r)) - 1,
        "MaxDD": (eq / eq.cummax() - 1).min(),
    }


# --------------------- synthetic sensitivity analysis ---------------------

def ar1_series(n: int, phi: float, sigma: float = 0.015,
               seed: int = 0) -> pd.Series:
    """Price series whose daily returns have autocorrelation phi."""
    rng = np.random.default_rng(seed)
    eps = rng.normal(0, sigma, n)
    r = np.zeros(n)
    for t in range(1, n):
        r[t] = phi * r[t - 1] + eps[t]
    idx = pd.bdate_range("2010-01-04", periods=n)
    return pd.Series(100 * np.exp(np.cumsum(r)), index=idx)


def sweep() -> pd.DataFrame:
    """Sharpe of the strategy across a range of daily autocorrelations."""
    phis = np.round(np.arange(-0.15, 0.16, 0.025), 3)
    rows = []
    for phi in phis:
        gross, net = [], []
        for seed in range(40):                     # average out sampling noise
            px = ar1_series(3000, phi, seed=seed)
            gross.append(stats(run(px, cost_bps=0.0))["Sharpe"])
            net.append(stats(run(px, cost_bps=10.0))["Sharpe"])
        rows.append({"phi": phi,
                     "sharpe_gross": np.mean(gross),
                     "sharpe_net_10bps": np.mean(net)})
    return pd.DataFrame(rows)


def measure_autocorr(close: pd.Series) -> float:
    r = close.pct_change().dropna()
    return r.autocorr(lag=1)


# --------------------------------- main ----------------------------------

def main() -> None:
    tickers = sys.argv[1:]

    if not tickers:
        print("Synthetic sweep: strategy Sharpe vs daily return autocorrelation")
        print("(40 seeds x 3000 days per row)\n")
        df = sweep()
        print(df.to_string(index=False, float_format=lambda v: f"{v:8.3f}"))
        print("\nThe strategy is profitable only where phi > 0.")
        print("Typical daily autocorrelation for real equities is NEGATIVE,")
        print("roughly -0.02 to -0.05. Check yours with the ticker argument.")
        return

    for t in tickers:
        try:
            from market_data import load_ohlc
            df = load_ohlc(t, start="2010-01-01")
        except Exception as exc:  # noqa: BLE001
            print(f"{t}: could not load ({exc})")
            continue
        if df.empty:
            print(f"{t}: no data")
            continue

        close, open_ = df["Close"], df["Open"]
        phi = measure_autocorr(close)

        print(f"\n{t}  ({df.index[0].date()} to {df.index[-1].date()})")
        print(f"  daily return autocorrelation: {phi:+.4f}"
              f"   -> {'continuation' if phi > 0 else 'REVERSAL'}")

        for label, kwargs in [("close-vs-prev-close", {"body": False}),
                              ("candle body", {"body": True, "open_": open_})]:
            for bps in (0.0, 5.0, 10.0):
                s = stats(run(close, cost_bps=bps, **kwargs))
                print(f"  {label:<20} {bps:>4.0f}bps  "
                      f"Sharpe {s['Sharpe']:>6.2f}  CAGR {s['CAGR']:>7.2%}  "
                      f"MaxDD {s['MaxDD']:>7.1%}")

        bh = stats(close.pct_change())
        print(f"  {'buy & hold':<20} {'':>8}  Sharpe {bh['Sharpe']:>6.2f}  "
              f"CAGR {bh['CAGR']:>7.2%}  MaxDD {bh['MaxDD']:>7.1%}")


if __name__ == "__main__":
    main()
