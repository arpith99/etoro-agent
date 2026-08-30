#!/usr/bin/env python3
"""Compare eToro's candle history against Tiingo's, to establish what a bar means.

eToro's `candlesResponse` carries raw open/high/low/close with no adjustment
field, and the docs do not say whether the series is corrected for splits and
dividends. That matters more than depth: backtesting an unadjusted 4:1 split
presents a 75% single-bar drawdown that a strategy will happily trade.

This script answers the question empirically. For each symbol it pulls eToro's
daily candles and the same date range from Tiingo, then compares eToro's closes
against *both* Tiingo series -- raw `close` and `adjClose`. Whichever it tracks
is the convention eToro uses.

Pick symbols with a split inside the window; a split is a far stronger signal
than dividends, and without either corporate action the two Tiingo series are
identical and nothing can be concluded (reported as "inconclusive").

Usage:

    # .env supplies ETORO_API_KEY, ETORO_USER_KEY and TIINGO_API_KEY
    python3 scripts/compare_candle_sources.py NVDA AAPL

Network-bound and read-only. Requires a free Tiingo token (https://tiingo.com).
"""

import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request
import uuid
from collections import OrderedDict

ETORO = "https://public-api.etoro.com"
TIINGO = "https://api.tiingo.com"

#: eToro caps `candlesCount` at 1000 and offers no date range, so this is the
#: entire history reachable through the API -- about four years of daily bars.
CANDLE_COUNT = 1000

#: urllib announces itself as "Python-urllib/3.x", which eToro's edge rejects
#: with a 403 -- the same bot protection that 403s a plain curl. reqwest sends
#: no User-Agent at all, which is why the Rust client is unaffected. Override
#: with SPIKE_USER_AGENT if this value is not accepted either.
USER_AGENT = os.environ.get("SPIKE_USER_AGENT", "etoro-agent/0.1 (candle source comparison)")

#: Below this, the two Tiingo series are indistinguishable and the comparison
#: says nothing about which one eToro follows.
INCONCLUSIVE_EPSILON = 1e-9


# --------------------------------------------------------------------------
# Pure helpers (unit-tested in scripts/tests/)
# --------------------------------------------------------------------------


def median(values):
    values = sorted(values)
    if not values:
        return float("nan")
    return values[len(values) // 2]


def biggest_jump(series):
    """Largest single-day price ratio. An unadjusted split shows up here.

    Returns ``(ratio, from_date, to_date)``, or ``(0.0, None, None)`` when the
    series is too short to have a move.
    """
    items = list(series.items())
    worst = (0.0, None, None)
    for (from_date, before), (to_date, after) in zip(items, items[1:]):
        if before > 0 and after > 0:
            ratio = max(after / before, before / after)
            if ratio > worst[0]:
                worst = (ratio, from_date, to_date)
    return worst


def compare_series(etoro, tiingo):
    """Decide which Tiingo series eToro's closes track.

    ``etoro`` maps date -> close. ``tiingo`` maps date -> (close, adj_close,
    split_factor). Returns a summary dict; ``verdict`` is one of ``adjusted``,
    ``raw`` or ``inconclusive``.
    """
    shared = [date for date in etoro if date in tiingo]
    raw_diffs, adj_diffs = [], []
    for date in shared:
        mine = etoro[date]
        close, adj_close, _ = tiingo[date]
        if close:
            raw_diffs.append(abs(mine - close) / close)
        if adj_close:
            adj_diffs.append(abs(mine - adj_close) / adj_close)

    median_raw, median_adj = median(raw_diffs), median(adj_diffs)
    if not shared:
        verdict = "inconclusive"
    elif abs(median_raw - median_adj) < INCONCLUSIVE_EPSILON:
        # No corporate action in the window: close == adjClose throughout, so
        # matching both proves nothing.
        verdict = "inconclusive"
    elif median_adj < median_raw:
        verdict = "adjusted"
    else:
        verdict = "raw"

    return {
        "overlap": len(shared),
        "median_raw": median_raw,
        "median_adj": median_adj,
        "verdict": verdict,
        "splits": [
            (date, factor)
            for date, (_, _, factor) in tiingo.items()
            if abs(factor - 1.0) > INCONCLUSIVE_EPSILON
        ],
        "largest_jump": biggest_jump(etoro),
    }


# --------------------------------------------------------------------------
# I/O
# --------------------------------------------------------------------------


def load_env(path=".env"):
    """Populates os.environ from .env without overriding what is already set."""
    if not os.path.exists(path):
        return
    with open(path) as handle:
        for line in handle:
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                key, _, value = line.partition("=")
                os.environ.setdefault(key.strip(), value.strip().strip('"').strip("'"))


def get(url, headers):
    headers = dict(headers)
    headers.setdefault("User-Agent", USER_AGENT)
    request = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        body = " ".join(error.read()[:300].decode("utf-8", "replace").split())
        # The query string is stripped: it carries the Tiingo token.
        raise RuntimeError(f"HTTP {error.code} from {url.split('?')[0]} :: {body}") from None


def etoro_headers():
    return {
        "x-api-key": os.environ["ETORO_API_KEY"],
        "x-user-key": os.environ["ETORO_USER_KEY"],
        "x-request-id": str(uuid.uuid4()),
    }


def resolve(symbol):
    """Symbol -> eToro instrument ID, verified client-side.

    The search endpoint ignores filters it does not recognise rather than
    rejecting them, so the match is re-established here. Negative IDs are the
    system-aggregate pseudo-instrument and never resolve.
    """
    query = urllib.parse.urlencode(
        {"internalSymbolFull": symbol, "fields": "instrumentId,internalSymbolFull"}
    )
    body = get(f"{ETORO}/api/v1/market-data/search?{query}", etoro_headers())
    for item in body.get("items", []):
        found, instrument_id = item.get("internalSymbolFull"), item.get("instrumentId")
        if found and found.upper() == symbol.upper() and instrument_id and instrument_id > 0:
            return instrument_id
    return None


def etoro_candles(instrument_id):
    path = (
        f"/api/v1/market-data/instruments/{instrument_id}"
        f"/history/candles/asc/OneDay/{CANDLE_COUNT}"
    )
    body = get(ETORO + path, etoro_headers())
    # `candles` nests inside `candles`: outer items are per-instrument, inner
    # items per time period.
    return OrderedDict(
        (candle["fromDate"][:10], float(candle["close"]))
        for group in body.get("candles", [])
        for candle in group.get("candles", [])
        if candle.get("fromDate") and candle.get("close") is not None
    )


def tiingo_prices(symbol, start, end):
    query = urllib.parse.urlencode(
        {"startDate": start, "endDate": end, "token": os.environ["TIINGO_API_KEY"]}
    )
    body = get(f"{TIINGO}/tiingo/daily/{symbol}/prices?{query}", {})
    return OrderedDict(
        (
            row["date"][:10],
            (float(row["close"]), float(row["adjClose"]), float(row.get("splitFactor", 1.0))),
        )
        for row in body
    )


def report(symbol):
    print(f"\n{'=' * 64}\n{symbol}\n{'=' * 64}")

    instrument_id = resolve(symbol)
    if not instrument_id:
        print("  eToro: symbol did not resolve")
        return
    etoro = etoro_candles(instrument_id)
    if not etoro:
        print(f"  eToro: instrument {instrument_id}, no candles returned")
        return

    dates = list(etoro)
    print(f"  eToro:  instrument {instrument_id}, {len(etoro)} bars {dates[0]} .. {dates[-1]}")
    tiingo = tiingo_prices(symbol, dates[0], dates[-1])
    print(f"  Tiingo: {len(tiingo)} bars")

    result = compare_series(etoro, tiingo)
    print(f"  overlap: {result['overlap']} dates")
    print(f"\n  median |diff| vs Tiingo close    (raw): {result['median_raw']:.6%}")
    print(f"  median |diff| vs Tiingo adjClose (adj): {result['median_adj']:.6%}")
    print(f"\n  splits in window (Tiingo): {result['splits'] or 'none'}")
    ratio, from_date, to_date = result["largest_jump"]
    print(f"  largest 1-day move in eToro series: {ratio:.2f}x  ({from_date} -> {to_date})")
    print(f"\n  --> eToro closes look {result['verdict'].upper()}")
    if result["verdict"] == "inconclusive":
        print("      (no corporate action in the window, or no overlapping dates)")
    elif result["splits"]:
        print("      (a split is in the window, so this is a strong signal)")
    else:
        print("      (dividends only -- weaker than a split, but real)")


def main(argv):
    load_env()
    missing = [
        name
        for name in ("ETORO_API_KEY", "ETORO_USER_KEY", "TIINGO_API_KEY")
        if not os.environ.get(name)
    ]
    if missing:
        sys.exit(f"missing environment variables: {', '.join(missing)}")

    for symbol in argv or ["NVDA", "AAPL"]:
        try:
            report(symbol)
        except Exception as error:  # a diagnostic: report and continue
            print(f"\n{symbol}: FAILED -- {error}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
