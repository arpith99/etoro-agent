"""
trend.py — the trend-following family, implemented and compared honestly.

Four ways of measuring the same thing:
    tsmom     : is the trailing 12m return positive?
    donchian  : has price broken the N-day high?
    turtle    : Donchian entry + ATR-based sizing + pyramiding + 2N stop
    ewmac     : continuous forecast from EWMA crossover, vol-normalised (Carver)

They differ far less than their proponents claim. What separates them is not the
entry rule but the position sizing, which is why the Turtle sizing rules get a
full implementation here rather than a mention.

Trend following has a characteristic signature: a LOW hit rate (35-45%), positive
skew, and long drawdowns. The extra metrics in this file exist to show that.

Run:
    python trend.py                    # synthetic validation
    python trend.py SPY EFA TLT GLD    # real data
"""

from __future__ import annotations

import sys
import numpy as np
import pandas as pd

ANNUAL = 252
RISK_PER_UNIT = 0.01      # Turtle: 1% of equity risked per unit
MAX_UNITS = 4             # Turtle: max 4 units pyramided per market
STOP_N_MULT = 2.0         # Turtle: stop at 2N against entry
VOL_TARGET = 0.15
MAX_WEIGHT = 2.0


# ------------------------------ indicators -------------------------------

def atr(high: pd.Series, low: pd.Series, close: pd.Series,
        n: int = 20) -> pd.Series:
    """Average true range. The Turtles called this 'N'. It is a volatility
    estimate that accounts for overnight gaps, unlike a plain return stdev."""
    prev = close.shift(1)
    tr = pd.concat([high - low, (high - prev).abs(), (low - prev).abs()],
                   axis=1).max(axis=1)
    return tr.ewm(span=n, adjust=False).mean()


def realised_vol(close: pd.Series, n: int = 36) -> pd.Series:
    return close.pct_change().ewm(span=n, adjust=False).std() * np.sqrt(ANNUAL)


# ------------------------------ strategies -------------------------------

def tsmom(close, high=None, low=None, lookback=252, **kw) -> pd.Series:
    """Binary time-series momentum. The simplest member of the family."""
    return (close.pct_change(lookback) > 0).astype(float)


def donchian(close, high=None, low=None, entry=20, exit_=10, **kw) -> pd.Series:
    """Long when price breaks the N-day high; flat when it breaks the M-day low.

    Richard Donchian's rule, and the entry the Turtles used. Note entry and exit
    windows differ: you enter on strength but give the trend room before quitting.
    """
    high = close if high is None else high
    low = close if low is None else low

    upper = high.rolling(entry).max().shift(1)
    lower = low.rolling(exit_).min().shift(1)

    sig = pd.Series(np.nan, index=close.index)
    sig[close > upper] = 1.0
    sig[close < lower] = 0.0
    return sig.ffill().fillna(0.0)


def turtle(close, high=None, low=None, entry=20, exit_=10,
           equity_frac=True, **kw) -> pd.Series:
    """The full Turtle system, as taught by Dennis and Eckhardt in 1983.

    The famous part is the Donchian entry. The part that actually made it work
    is below: position size is inversely proportional to ATR, so every market
    contributes the same risk regardless of its volatility. Units are pyramided
    as the trend runs, and a 2N stop caps the loss on each.

    Returns a weight (fraction of equity), not a binary signal.
    """
    high = close if high is None else high
    low = close if low is None else low

    n = atr(high, low, close, 20)
    upper = high.rolling(entry).max().shift(1)
    lower = low.rolling(exit_).min().shift(1)

    # unit size: risking RISK_PER_UNIT of equity over a 1N move
    #   shares = (risk_frac * equity) / N   ->   weight = risk_frac * price / N
    unit_w = (RISK_PER_UNIT * close / n).replace([np.inf, -np.inf], np.nan)

    weights = np.zeros(len(close))
    units = 0
    entry_px = np.nan

    c = close.to_numpy()
    u = upper.to_numpy()
    l = lower.to_numpy()
    nn = n.to_numpy()
    uw = unit_w.to_numpy()

    for i in range(len(c)):
        if np.isnan(nn[i]) or np.isnan(uw[i]):
            continue

        if units > 0:
            # 2N stop, or Donchian exit
            if c[i] < entry_px - STOP_N_MULT * nn[i] or c[i] < l[i]:
                units, entry_px = 0, np.nan
            # pyramid: add a unit every 0.5N of favourable movement
            elif c[i] > entry_px + 0.5 * nn[i] and units < MAX_UNITS:
                units += 1
                entry_px = c[i]
        elif c[i] > u[i]:
            units, entry_px = 1, c[i]

        weights[i] = min(units * uw[i], MAX_WEIGHT)

    return pd.Series(weights, index=close.index)


def ewmac(close, high=None, low=None, fast=32, slow=128, **kw) -> pd.Series:
    """Carver's EWMAC: a continuous forecast rather than a binary signal.

    The crossover is divided by price volatility so the forecast means the same
    thing across assets and regimes, then scaled and capped. This gives partial
    positions in weak trends instead of all-or-nothing, which cuts turnover.
    """
    raw = close.ewm(span=fast).mean() - close.ewm(span=slow).mean()
    vol = close.diff().ewm(span=36).std()
    forecast = (raw / vol.replace(0, np.nan))

    # scale so average absolute forecast is ~10, cap at 20 (Carver's convention)
    scalar = 10.0 / forecast.abs().expanding(252).mean()
    scaled = (forecast * scalar).clip(-20, 20)

    return (scaled / 10.0).clip(lower=0).fillna(0.0)   # long-only version


STRATEGIES = {
    "tsmom_12m": tsmom,
    "donchian_20_10": donchian,
    "turtle_full": turtle,
    "ewmac_32_128": ewmac,
}


# -------------------------------- engine ---------------------------------

def backtest(close, high=None, low=None, fn=tsmom, vol_target=VOL_TARGET,
             cost_bps=10.0, **kw) -> pd.Series:
    rets = close.pct_change().fillna(0.0)
    w = fn(close, high, low, **kw)

    if vol_target:
        # scale to a constant risk target; turtle already sizes by ATR so this
        # mostly normalises the others onto the same footing for comparison
        vol = (w * rets).ewm(span=60).std() * np.sqrt(ANNUAL)
        w = (w * (vol_target / vol.replace(0, np.nan))).clip(0, MAX_WEIGHT)

    w = w.shift(1).fillna(0.0)
    turn = w.diff().abs().fillna(0.0)
    net = (w * rets - turn * cost_bps / 10_000.0).fillna(0.0)
    return net, w


# ------------------------------- metrics ---------------------------------

def dd_duration(equity: pd.Series) -> int:
    """Longest stretch, in days, spent below a previous peak."""
    peak = equity.cummax()
    under = equity < peak
    if not under.any():
        return 0
    grp = (~under).cumsum()
    return int(under.groupby(grp).sum().max())


def trade_stats(r: pd.Series, w: pd.Series) -> dict:
    """Per-TRADE statistics, not per-day.

    This distinction matters enormously. Daily hit rate for any long strategy
    sits near 51% and tells you nothing. Trend following's actual signature is
    a low hit rate PER TRADE with a large win/loss ratio: you are wrong most
    times you enter, and the rare large winner pays for all of it.
    """
    in_mkt = w.abs() > 1e-9
    if not in_mkt.any():
        return {"Trades": 0, "Hit": np.nan, "W/L": np.nan, "Best": np.nan}

    # each contiguous block of exposure is one trade
    blocks = (in_mkt != in_mkt.shift()).cumsum()[in_mkt]
    pnl = r[in_mkt].groupby(blocks).apply(lambda x: (1 + x).prod() - 1)

    wins, losses = pnl[pnl > 0], pnl[pnl < 0]
    return {
        "Trades": len(pnl),
        "Hit": len(wins) / len(pnl) if len(pnl) else np.nan,
        "W/L": wins.mean() / abs(losses.mean()) if len(losses) else np.nan,
        "Best": pnl.max() if len(pnl) else np.nan,
    }


def metrics(r: pd.Series, w: pd.Series | None = None) -> dict:
    r = r.dropna()
    r = r[r.index.notna()]
    base = dict.fromkeys(
        ["CAGR", "Vol", "Sharpe", "MaxDD", "DDdays", "Trades", "Hit",
         "W/L", "Skew"], np.nan)
    if len(r) < 100 or r.std() == 0:
        return base

    eq = (1 + r).cumprod()
    yrs = len(r) / ANNUAL

    ts = trade_stats(r, w.reindex(r.index).fillna(0.0)) if w is not None else {}

    return {
        "CAGR": eq.iloc[-1] ** (1 / yrs) - 1,
        "Vol": r.std() * np.sqrt(ANNUAL),
        "Sharpe": (r.mean() * ANNUAL) / (r.std() * np.sqrt(ANNUAL)),
        "MaxDD": (eq / eq.cummax() - 1).min(),
        "DDdays": dd_duration(eq),
        "Trades": ts.get("Trades", np.nan),
        "Hit": ts.get("Hit", np.nan),
        "W/L": ts.get("W/L", np.nan),
        "Skew": r.skew(),
    }


def table(results: dict, label: str = "") -> None:
    if label:
        print(f"\n{label}")
    print("-" * 94)
    print(f"{'strategy':<17}{'CAGR':>8}{'Vol':>8}{'Sharpe':>8}{'MaxDD':>9}"
          f"{'DDdays':>8}{'Trades':>8}{'Hit':>7}{'W/L':>7}{'Skew':>8}")
    for name, val in results.items():
        r, w = val if isinstance(val, tuple) else (val, None)
        m = metrics(r, w)
        tr = f"{m['Trades']:>8.0f}" if m['Trades'] == m['Trades'] else f"{'-':>8}"
        hit = f"{m['Hit']:>7.1%}" if m['Hit'] == m['Hit'] else f"{'-':>7}"
        wl = f"{m['W/L']:>7.2f}" if m['W/L'] == m['W/L'] else f"{'-':>7}"
        print(f"{name:<17}{m['CAGR']:>7.1%}{m['Vol']:>8.1%}{m['Sharpe']:>8.2f}"
              f"{m['MaxDD']:>9.1%}{m['DDdays']:>8.0f}{tr}{hit}{wl}"
              f"{m['Skew']:>8.2f}")


# ------------------------------ data / main -------------------------------

def load(ticker: str, start="2000-01-01"):
    from market_data import load_ohlc
    df = load_ohlc(ticker, start=start)
    return df["Close"], df["High"], df["Low"]


def synthetic(n=6000, regime=400, seed=5):
    """Trending series: the environment these strategies are built for."""
    rng = np.random.default_rng(seed)
    reg = np.repeat(rng.choice([-1, 1], size=n // regime + 1), regime)[:n]
    r = reg * 0.0008 + rng.normal(0, 0.012, n)
    idx = pd.bdate_range("2003-01-02", periods=n)
    close = pd.Series(100 * np.exp(np.cumsum(r)), index=idx)
    noise = np.abs(rng.normal(0, 0.004, n))
    return close, close * (1 + noise), close * (1 - noise)


def main() -> None:
    tickers = sys.argv[1:]

    if not tickers:
        print("No tickers given — running synthetic validation on TRENDING data.")
        print("(This is the environment trend following is designed for; real")
        print(" markets trend far less often, so treat these as upper bounds.)\n")
        close, high, low = synthetic()
        res = {n: backtest(close, high, low, fn)
               for n, fn in STRATEGIES.items()}
        res["buy_and_hold"] = close.pct_change().fillna(0.0)
        table(res, "Synthetic trending series")
        print("\nNote the signature: hit rates well under 50%, W/L above 1,")
        print("positive skew. Trend following loses often and wins big. If your")
        print("backtest shows a high hit rate, you have built something else.")
        return

    for t in tickers:
        try:
            close, high, low = load(t)
        except Exception as exc:  # noqa: BLE001
            print(f"{t}: {exc}")
            continue
        res = {n: backtest(close, high, low, fn)
               for n, fn in STRATEGIES.items()}
        res["buy_and_hold"] = close.pct_change().fillna(0.0)
        table(res, f"{t}   {close.index[0].date()} to {close.index[-1].date()}")


if __name__ == "__main__":
    main()
