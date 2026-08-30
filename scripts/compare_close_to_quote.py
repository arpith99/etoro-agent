#!/usr/bin/env python3
"""Is eToro's candle close the bid, the mid, or the last trade?

eToro's daily closes sit about 0.16% *below* Tiingo's, consistently across
unrelated tickers -- see docs/api-observations.md. That is 25-50x the intraday
spreads measured on the same instruments, and far too small to be a session
misalignment. The leading explanation is that eToro reports the bid rather than
a last trade, with spreads widening at the close.

This checks that against eToro's own live quote, which avoids the cross-vendor
question entirely: fetch the most recent daily candle and the live bid/ask for
the same instrument, and see which side of the quote the close sits on.

**Run this while the market is closed.** With the market open, the newest
candle is a previous session and the live quote has moved since, so the
comparison measures intraday drift rather than quote convention. The script
reports the quote's own timestamp so you can tell which situation you are in,
and says so when they disagree.

    python3 scripts/compare_close_to_quote.py AAPL TSLA MBLY
"""

import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request
import uuid

ETORO = "https://public-api.etoro.com"

#: urllib's default user agent is rejected by eToro's edge; see
#: scripts/compare_candle_sources.py for the same trap.
USER_AGENT = os.environ.get("SPIKE_USER_AGENT", "etoro-agent/0.1 (quote comparison)")

#: Within this fraction, a close is treated as matching a quote level.
MATCH_TOLERANCE = 0.0002


# --------------------------------------------------------------------------
# Pure helpers (unit-tested in scripts/tests/)
# --------------------------------------------------------------------------


def classify(close, bid, ask):
    """Which level the close sits on: 'bid', 'ask', 'mid', or a description.

    Returns ``(label, offsets)`` where offsets maps each level to the close's
    relative distance from it.
    """
    mid = (bid + ask) / 2.0
    offsets = {
        "bid": (close - bid) / bid if bid else float("nan"),
        "mid": (close - mid) / mid if mid else float("nan"),
        "ask": (close - ask) / ask if ask else float("nan"),
    }
    nearest = min(offsets, key=lambda level: abs(offsets[level]))
    if abs(offsets[nearest]) <= MATCH_TOLERANCE:
        return nearest, offsets
    if close < bid:
        return "below the bid", offsets
    if close > ask:
        return "above the ask", offsets
    return f"inside the spread, nearest {nearest}", offsets


def spread_fraction(bid, ask):
    """Quoted spread as a fraction of mid."""
    mid = (bid + ask) / 2.0
    return (ask - bid) / mid if mid else float("nan")


# --------------------------------------------------------------------------
# I/O
# --------------------------------------------------------------------------


def load_env(path=".env"):
    if not os.path.exists(path):
        return
    with open(path) as handle:
        for line in handle:
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                key, _, value = line.partition("=")
                os.environ.setdefault(key.strip(), value.strip().strip('"').strip("'"))


def get(url):
    request = urllib.request.Request(
        url,
        headers={
            "x-api-key": os.environ["ETORO_API_KEY"],
            "x-user-key": os.environ["ETORO_USER_KEY"],
            "x-request-id": str(uuid.uuid4()),
            "User-Agent": USER_AGENT,
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        body = " ".join(error.read()[:300].decode("utf-8", "replace").split())
        raise RuntimeError(f"HTTP {error.code} from {url.split('?')[0]} :: {body}") from None


def resolve(symbol):
    query = urllib.parse.urlencode(
        {"internalSymbolFull": symbol, "fields": "instrumentId,internalSymbolFull"}
    )
    body = get(f"{ETORO}/api/v1/market-data/search?{query}")
    for item in body.get("items", []):
        found, iid = item.get("internalSymbolFull"), item.get("instrumentId")
        if found and found.upper() == symbol.upper() and iid and iid > 0:
            return iid
    return None


def latest_candle(instrument_id):
    path = (
        f"/api/v1/market-data/instruments/{instrument_id}"
        f"/history/candles/desc/OneDay/2"
    )
    body = get(ETORO + path)
    for group in body.get("candles", []):
        for candle in group.get("candles", []):
            if candle.get("close") is not None:
                return candle
    return None


def live_rate(instrument_id):
    query = urllib.parse.urlencode({"instrumentIds": str(instrument_id)})
    body = get(f"{ETORO}/api/v1/market-data/instruments/rates?{query}")
    for rate in body.get("rates", []):
        if rate.get("instrumentID") == instrument_id:
            return rate
    return None


def report(symbol):
    print(f"\n{'=' * 60}\n{symbol}\n{'=' * 60}")
    instrument_id = resolve(symbol)
    if not instrument_id:
        print("  did not resolve")
        return

    candle, rate = latest_candle(instrument_id), live_rate(instrument_id)
    if not candle or not rate:
        print(f"  instrument {instrument_id}: no candle or no live rate")
        return

    close = float(candle["close"])
    bid, ask = float(rate.get("bid", 0)), float(rate.get("ask", 0))
    candle_date, quote_date = (candle.get("fromDate") or "")[:10], (rate.get("date") or "")
    print(f"  instrument {instrument_id}")
    print(f"  candle {candle_date}  close {close:.4f}")
    print(f"  quote  {quote_date[:19]}  bid {bid:.4f}  ask {ask:.4f}  "
          f"spread {spread_fraction(bid, ask):.4%}")

    if not bid or not ask:
        print("  no usable quote")
        return

    label, offsets = classify(close, bid, ask)
    print(f"\n  close vs bid {offsets['bid']:+.4%}   "
          f"mid {offsets['mid']:+.4%}   ask {offsets['ask']:+.4%}")
    print(f"  --> the close sits on the {label}")

    if quote_date[:10] != candle_date:
        print("\n  ! the quote and the candle are from different days, so this")
        print("    measures how far price moved since, not quote convention.")
        print("    Re-run while the market is closed for a clean reading.")


def main(argv):
    load_env()
    missing = [n for n in ("ETORO_API_KEY", "ETORO_USER_KEY") if not os.environ.get(n)]
    if missing:
        sys.exit(f"missing environment variables: {', '.join(missing)}")
    for symbol in argv or ["AAPL", "TSLA", "MBLY"]:
        try:
            report(symbol)
        except Exception as error:
            print(f"\n{symbol}: FAILED -- {error}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
