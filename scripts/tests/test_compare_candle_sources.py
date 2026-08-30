"""Unit tests for the candle-source comparison.

Covers the pure decision logic only; the HTTP calls are not exercised.

Run with:  python3 -m unittest discover -s scripts/tests -v
"""

import sys
import unittest
from collections import OrderedDict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from compare_candle_sources import (  # noqa: E402
    biggest_jump,
    compare_series,
    median,
)


class TestMedian(unittest.TestCase):
    def test_empty_is_nan(self):
        self.assertNotEqual(median([]), median([]))  # NaN != NaN

    def test_picks_the_middle_of_an_odd_series(self):
        self.assertEqual(median([3.0, 1.0, 2.0]), 2.0)

    def test_is_insensitive_to_input_order(self):
        self.assertEqual(median([9.0, 1.0, 5.0]), median([1.0, 5.0, 9.0]))


class TestBiggestJump(unittest.TestCase):
    def test_finds_an_unadjusted_split(self):
        # A 10:1 split in an unadjusted series: the price falls by 10x in a day.
        series = OrderedDict(
            [("2024-06-07", 1200.0), ("2024-06-10", 120.0), ("2024-06-11", 121.0)]
        )
        ratio, from_date, to_date = biggest_jump(series)
        self.assertAlmostEqual(ratio, 10.0)
        self.assertEqual((from_date, to_date), ("2024-06-07", "2024-06-10"))

    def test_reports_nothing_for_a_series_too_short_to_move(self):
        self.assertEqual(biggest_jump(OrderedDict([("2024-06-07", 10.0)])), (0.0, None, None))

    def test_ignores_non_positive_prices(self):
        # -1 is eToro's "no data" sentinel elsewhere; it must not become a ratio.
        series = OrderedDict([("d1", -1.0), ("d2", 100.0), ("d3", 200.0)])
        ratio, _, _ = biggest_jump(series)
        self.assertAlmostEqual(ratio, 2.0)


class TestCompareSeries(unittest.TestCase):
    @staticmethod
    def tiingo(rows):
        return OrderedDict(rows)

    def test_detects_an_unadjusted_series(self):
        etoro = OrderedDict([("d1", 1200.0), ("d2", 120.0)])
        tiingo = self.tiingo([("d1", (1200.0, 120.0, 1.0)), ("d2", (120.0, 120.0, 10.0))])
        result = compare_series(etoro, tiingo)
        self.assertEqual(result["verdict"], "raw")
        self.assertEqual(result["splits"], [("d2", 10.0)])

    def test_detects_an_adjusted_series(self):
        etoro = OrderedDict([("d1", 120.0), ("d2", 120.0)])
        tiingo = self.tiingo([("d1", (1200.0, 120.0, 1.0)), ("d2", (120.0, 120.0, 10.0))])
        self.assertEqual(compare_series(etoro, tiingo)["verdict"], "adjusted")

    def test_identical_tiingo_series_cannot_decide(self):
        # No corporate action: close == adjClose, so matching both proves
        # nothing and claiming "adjusted" would be a false positive.
        etoro = OrderedDict([("d1", 100.0), ("d2", 101.0)])
        tiingo = self.tiingo([("d1", (100.0, 100.0, 1.0)), ("d2", (101.0, 101.0, 1.0))])
        self.assertEqual(compare_series(etoro, tiingo)["verdict"], "inconclusive")

    def test_no_overlapping_dates_cannot_decide(self):
        etoro = OrderedDict([("d1", 100.0)])
        tiingo = self.tiingo([("d9", (100.0, 90.0, 1.0))])
        result = compare_series(etoro, tiingo)
        self.assertEqual(result["verdict"], "inconclusive")
        self.assertEqual(result["overlap"], 0)

    def test_counts_only_dates_present_in_both(self):
        etoro = OrderedDict([("d1", 100.0), ("d2", 101.0), ("d3", 102.0)])
        tiingo = self.tiingo([("d2", (101.0, 100.0, 1.0)), ("d9", (1.0, 1.0, 1.0))])
        self.assertEqual(compare_series(etoro, tiingo)["overlap"], 1)


if __name__ == "__main__":
    unittest.main()
