"""
strategies.py — a panel backtester for comparing several classic strategies honestly.

Everything here operates on a price panel (dates x tickers) and returns a weight
panel of the same shape. Weights are shifted inside the engine, so strategy
functions may use same-day data without introducing look-ahead.

The strategies are ordered roughly by how much published evidence supports them.
That ordering is the most useful thing in this file. Read STRATEGY_NOTES.

Install:
    pip install pandas numpy pandas-datareader
    # optional fallback:  pip install yfinance

Run:
    python strategies.py
"""

from __future__ import annotations

import numpy as np
import pandas as pd

# ----------------------------- configuration -----------------------------

# EUR-denominated UCITS ETFs on Xetra would be the live universe for a German
# account. Stooq covers them with a .DE suffix, though history is shorter.
# US tickers are used here because the history is long enough to mean something.
UNIVERSE = ["SPY", "QQQ", "IWM", "EFA", "EEM", "TLT", "IEF", "GLD", "DBC", "VNQ"]

START = "2005-01-01"
END = None

OOS_SPLIT = "2019-01-01"   # everything after this is untouched until the end
COST_BPS = 5.0             # per unit turnover, one way
VOL_TARGET = 0.10          # 10% annualised portfolio volatility
VOL_WINDOW = 60            # trailing days for the vol estimate
MAX_LEVERAGE = 1.5         # cap, so vol targeting can't run away
ANNUAL = 252


# ------------------------------- data ------------------------------------

def load_panel(tickers=UNIVERSE, start=START, end=END) -> pd.DataFrame:
    """Return a dates x tickers panel of close prices."""
    series = {}
    for t in tickers:
        px = None
        try:
            from pandas_datareader import data as pdr
            df = pdr.DataReader(t, "stooq", start, end)
            if not df.empty:
                px = df["Close"].sort_index()
        except Exception:  # noqa: BLE001
            pass

        if px is None:
            try:
                import yfinance as yf
                df = yf.download(t, start=start, end=end,
                                 auto_adjust=True, progress=False)
                if not df.empty:
                    px = df["Close"]
                    if isinstance(px, pd.DataFrame):
                        px = px.iloc[:, 0]
                    px = px.sort_index()
            except Exception:  # noqa: BLE001
                pass

        if px is None:
            print(f"  could not load {t}, skipping")
            continue
        series[t] = px

    panel = pd.DataFrame(series).sort_index()
    return panel.dropna(how="all").ffill()


# ----------------------------- strategies --------------------------------
# Each takes a price panel and returns a weight panel. Weights are fractions of
# capital; they need not sum to 1 (cash is the remainder).

def buy_and_hold(px: pd.DataFrame) -> pd.DataFrame:
    """Equal weight, always invested. The benchmark everything must beat."""
    return pd.DataFrame(1.0 / px.shape[1], index=px.index, columns=px.columns)


def tsmom(px: pd.DataFrame, lookback: int = 252) -> pd.DataFrame:
    """Time-series momentum: hold an asset if its own trailing return is positive.

    Moskowitz, Ooi & Pedersen (2012). Documented across 58 instruments and
    ~25 years. One of the few effects with evidence outside equities.
    """
    trailing = px.pct_change(lookback)
    longs = (trailing > 0).astype(float)
    n = longs.sum(axis=1).replace(0, np.nan)
    return longs.div(n, axis=0).fillna(0.0)


def xsmom(px: pd.DataFrame, lookback: int = 252, skip: int = 21,
          n_hold: int = 3) -> pd.DataFrame:
    """Cross-sectional momentum: hold the top N by past return, skipping last month.

    Jegadeesh & Titman (1993). The skip month matters: recent returns show
    short-term reversal, which eats into the momentum signal if you include them.
    """
    mom = px.shift(skip).pct_change(lookback - skip)
    ranks = mom.rank(axis=1, ascending=False)
    picks = (ranks <= n_hold).astype(float)
    n = picks.sum(axis=1).replace(0, np.nan)
    return picks.div(n, axis=0).fillna(0.0)


def dual_momentum(px: pd.DataFrame, lookback: int = 252,
                  n_hold: int = 3) -> pd.DataFrame:
    """Antonacci's combination: rank cross-sectionally, but only hold if
    absolute return is also positive. Otherwise sit in cash."""
    xs = xsmom(px, lookback=lookback, n_hold=n_hold)
    absolute = (px.pct_change(lookback) > 0).astype(float)
    combined = xs * absolute
    return combined


def short_reversal(px: pd.DataFrame, lookback: int = 5,
                   n_hold: int = 3) -> pd.DataFrame:
    """Short-horizon mean reversion: buy the worst recent performers.

    Works better on liquid ETFs than single stocks, and better in high-vol
    regimes. Turnover is brutal, so costs decide whether it survives.
    """
    trailing = px.pct_change(lookback)
    ranks = trailing.rank(axis=1, ascending=True)
    picks = (ranks <= n_hold).astype(float)
    n = picks.sum(axis=1).replace(0, np.nan)
    return picks.div(n, axis=0).fillna(0.0)


def sma_cross(px: pd.DataFrame, fast: int = 50, slow: int = 200) -> pd.DataFrame:
    """The strategy every tutorial starts with. Included so you can watch it
    underperform the ones above and stop wondering about it."""
    longs = (px.rolling(fast).mean() > px.rolling(slow).mean()).astype(float)
    n = longs.sum(axis=1).replace(0, np.nan)
    return longs.div(n, axis=0).fillna(0.0)


STRATEGIES = {
    "buy_and_hold": buy_and_hold,
    "tsmom_12m": tsmom,
    "xsmom_top3": xsmom,
    "dual_momentum": dual_momentum,
    "short_reversal": short_reversal,
    "sma_50_200": sma_cross,
}


# --------------------------- position sizing ------------------------------

def apply_vol_target(weights: pd.DataFrame, rets: pd.DataFrame,
                     target: float = VOL_TARGET,
                     window: int = VOL_WINDOW,
                     max_lev: float = MAX_LEVERAGE) -> pd.DataFrame:
    """Scale the whole book so realised portfolio vol tracks the target.

    This is the single highest-value addition to almost any strategy. It does
    not improve your signal at all; it stops position size from being an
    accident of which assets you happened to pick.
    """
    port_ret = (weights * rets).sum(axis=1)
    realised = port_ret.rolling(window).std() * np.sqrt(ANNUAL)
    scale = (target / realised).clip(upper=max_lev).fillna(0.0)
    return weights.mul(scale, axis=0)


# ------------------------------- engine ----------------------------------

def backtest(px: pd.DataFrame, weight_fn, vol_target: bool = True,
             cost_bps: float = COST_BPS, **kwargs) -> pd.Series:
    """Return a daily net return series for the strategy."""
    rets = px.pct_change().fillna(0.0)
    w = weight_fn(px, **kwargs)

    if vol_target:
        w = apply_vol_target(w, rets)

    w = w.shift(1).fillna(0.0)          # trade on yesterday's information

    turnover = w.diff().abs().sum(axis=1).fillna(0.0)
    costs = turnover * (cost_bps / 10_000.0)

    return (w * rets).sum(axis=1) - costs


# ------------------------------- metrics ---------------------------------

def metrics(rets: pd.Series) -> dict:
    rets = rets.dropna()
    if len(rets) < 60 or rets.std() == 0:
        return dict.fromkeys(["CAGR", "Vol", "Sharpe", "MaxDD", "Calmar"], np.nan)

    equity = (1.0 + rets).cumprod()
    years = len(rets) / ANNUAL
    cagr = equity.iloc[-1] ** (1.0 / years) - 1.0
    vol = rets.std() * np.sqrt(ANNUAL)
    sharpe = (rets.mean() * ANNUAL) / vol
    dd = (equity / equity.cummax() - 1.0).min()
    calmar = cagr / abs(dd) if dd < 0 else np.nan

    return {"CAGR": cagr, "Vol": vol, "Sharpe": sharpe,
            "MaxDD": dd, "Calmar": calmar}


def table(results: dict[str, pd.Series], label: str) -> None:
    print(f"\n{label}")
    print("-" * 72)
    print(f"{'strategy':<18}{'CAGR':>9}{'Vol':>9}{'Sharpe':>9}"
          f"{'MaxDD':>10}{'Calmar':>9}")
    for name, r in results.items():
        m = metrics(r)
        print(f"{name:<18}{m['CAGR']:>8.1%}{m['Vol']:>9.1%}"
              f"{m['Sharpe']:>9.2f}{m['MaxDD']:>10.1%}{m['Calmar']:>9.2f}")


# -------------------------------- main -----------------------------------

def main() -> None:
    print("Loading price panel...")
    px = load_panel()
    if px.shape[1] < 2:
        print("Not enough data loaded. Check your connection or ticker list.")
        return

    print(f"Loaded {px.shape[1]} instruments, "
          f"{px.index[0].date()} to {px.index[-1].date()}")

    is_px = px.loc[:OOS_SPLIT]
    oos_px = px.loc[OOS_SPLIT:]

    for period_name, panel in [("IN-SAMPLE (design here)", is_px),
                               ("OUT-OF-SAMPLE (judge here)", oos_px)]:
        results = {name: backtest(panel, fn) for name, fn in STRATEGIES.items()}
        table(results, f"{period_name}  {panel.index[0].date()} to "
                       f"{panel.index[-1].date()}")

    # Show what vol targeting actually does, using one strategy as the example.
    print("\n\nEffect of volatility targeting (tsmom_12m, full period)")
    print("-" * 72)
    comparison = {
        "without": backtest(px, tsmom, vol_target=False),
        "with": backtest(px, tsmom, vol_target=True),
    }
    table(comparison, "")

    print("\n" + "=" * 72)
    print("How to read this:")
    print("  - Compare every row to buy_and_hold in the SAME period.")
    print("  - A strategy that wins in-sample and loses out-of-sample is noise.")
    print("  - Sharpe rewards consistency; Calmar rewards surviving drawdowns.")
    print("  - Vol targeting usually lifts Sharpe without improving the signal.")


if __name__ == "__main__":
    main()
