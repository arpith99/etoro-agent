"""
market_data.py — a small, dependency-light price loader.

pandas-datareader 0.11.1 removed its stooq module, so DataReader(..., "stooq")
now raises NotImplementedError. This talks to Stooq's CSV endpoint directly,
which needs nothing beyond pandas, and falls back to yfinance if installed.

Usage:
    from market_data import load_ohlc, load_close, load_panel

    df    = load_ohlc("AAPL")            # DataFrame: Open/High/Low/Close/Volume
    close = load_close("AAPL")           # Series
    panel = load_panel(["SPY", "TLT"])   # DataFrame of closes
"""

from __future__ import annotations

import io
import urllib.request
import pandas as pd

STOOQ_URL = "https://stooq.com/q/d/l/?s={sym}&i=d"
_UA = {"User-Agent": "Mozilla/5.0 (compatible; research script)"}


def _stooq_symbol(ticker: str) -> str:
    """Stooq wants a market suffix. US tickers need .US; European ones like
    SAP.DE already carry theirs."""
    t = ticker.lower()
    return t if "." in t else f"{t}.us"


def _from_stooq(ticker: str) -> pd.DataFrame | None:
    url = STOOQ_URL.format(sym=_stooq_symbol(ticker))
    try:
        req = urllib.request.Request(url, headers=_UA)
        with urllib.request.urlopen(req, timeout=30) as resp:
            raw = resp.read().decode("utf-8", errors="replace")
    except Exception as exc:  # noqa: BLE001
        print(f"  stooq request failed for {ticker}: {exc}")
        return None

    if "Date" not in raw[:200]:
        # Stooq returns the string "No data" for unknown symbols
        print(f"  stooq returned no data for {ticker} "
              f"(tried symbol '{_stooq_symbol(ticker)}')")
        return None

    df = pd.read_csv(io.StringIO(raw), parse_dates=["Date"], index_col="Date")
    return df.sort_index() if not df.empty else None


def _from_yfinance(ticker: str, start=None, end=None) -> pd.DataFrame | None:
    try:
        import yfinance as yf
    except ImportError:
        return None
    try:
        df = yf.download(ticker, start=start, end=end,
                         auto_adjust=True, progress=False)
        if df is None or df.empty:
            return None
        if isinstance(df.columns, pd.MultiIndex):     # single-ticker MultiIndex
            df.columns = df.columns.get_level_values(0)
        return df.sort_index()
    except Exception as exc:  # noqa: BLE001
        print(f"  yfinance failed for {ticker}: {exc}")
        return None


def load_ohlc(ticker: str, start=None, end=None) -> pd.DataFrame:
    """Return an OHLCV frame. Raises if no source works."""
    df = _from_stooq(ticker)
    if df is None:
        df = _from_yfinance(ticker, start, end)
    if df is None:
        raise RuntimeError(
            f"could not load {ticker}. Try 'pip install yfinance', "
            f"or check the symbol (Stooq uses e.g. 'ino.us', 'sap.de')."
        )

    keep = [c for c in ("Open", "High", "Low", "Close", "Volume")
            if c in df.columns]
    df = df[keep]

    if start is not None:
        df = df.loc[str(start):]
    if end is not None:
        df = df.loc[:str(end)]
    return df


def load_close(ticker: str, start=None, end=None) -> pd.Series:
    return load_ohlc(ticker, start, end)["Close"].rename(ticker)


def load_panel(tickers, start=None, end=None) -> pd.DataFrame:
    """Return a dates x tickers panel of closes, skipping failures."""
    out = {}
    for t in tickers:
        try:
            out[t] = load_close(t, start, end)
        except Exception as exc:  # noqa: BLE001
            print(f"  skipping {t}: {exc}")
    if not out:
        raise RuntimeError("no tickers loaded")
    return pd.DataFrame(out).sort_index().ffill()


if __name__ == "__main__":
    import sys
    for t in sys.argv[1:] or ["SPY"]:
        try:
            df = load_ohlc(t)
            print(f"{t}: {len(df)} rows, {df.index[0].date()} to "
                  f"{df.index[-1].date()}, last close {df['Close'].iloc[-1]:.2f}")
        except Exception as exc:  # noqa: BLE001
            print(f"{t}: {exc}")
