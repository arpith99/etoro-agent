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
    split_findings,
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

    def test_detects_a_dividend_unadjusted_series(self):
        etoro = OrderedDict([("d1", 100.0), ("d2", 101.0)])
        tiingo = self.tiingo([("d1", (100.0, 90.0, 1.0)), ("d2", (101.0, 91.0, 1.0))])
        self.assertEqual(compare_series(etoro, tiingo)["dividend_verdict"], "raw")

    def test_detects_a_dividend_adjusted_series(self):
        etoro = OrderedDict([("d1", 90.0), ("d2", 91.0)])
        tiingo = self.tiingo([("d1", (100.0, 90.0, 1.0)), ("d2", (101.0, 91.0, 1.0))])
        self.assertEqual(compare_series(etoro, tiingo)["dividend_verdict"], "adjusted")

    def test_identical_tiingo_series_cannot_decide(self):
        # No corporate action: close == adjClose, so matching both proves
        # nothing and claiming "adjusted" would be a false positive.
        etoro = OrderedDict([("d1", 100.0), ("d2", 101.0)])
        tiingo = self.tiingo([("d1", (100.0, 100.0, 1.0)), ("d2", (101.0, 101.0, 1.0))])
        self.assertEqual(compare_series(etoro, tiingo)["dividend_verdict"], "inconclusive")

    def test_a_near_tie_is_not_a_verdict(self):
        # The regression this exists for: TSLA pays no dividend, the two
        # medians landed 0.001 percentage points apart, and the smaller one
        # won. A gap that small is noise, not evidence.
        etoro = OrderedDict([("d1", 100.0), ("d2", 100.0)])
        tiingo = self.tiingo([("d1", (100.161, 100.160, 1.0)), ("d2", (100.161, 100.160, 1.0))])
        self.assertEqual(compare_series(etoro, tiingo)["dividend_verdict"], "inconclusive")

    def test_signed_error_separates_a_markup_from_scatter(self):
        # eToro consistently 1% above: a correctable bias.
        etoro = OrderedDict([("d1", 101.0), ("d2", 202.0)])
        tiingo = self.tiingo([("d1", (100.0, 100.0, 1.0)), ("d2", (200.0, 200.0, 1.0))])
        self.assertAlmostEqual(compare_series(etoro, tiingo)["median_signed"], 0.01)

        # Same absolute error, scattered either side: nothing to correct. An
        # odd number of points, because a two-element median is just one of
        # the two values and cannot show a distribution centred on zero.
        etoro = OrderedDict([("d1", 99.0), ("d2", 100.0), ("d3", 101.0)])
        tiingo = self.tiingo(
            [
                ("d1", (100.0, 100.0, 1.0)),
                ("d2", (100.0, 100.0, 1.0)),
                ("d3", (100.0, 100.0, 1.0)),
            ]
        )
        result = compare_series(etoro, tiingo)
        self.assertAlmostEqual(result["median_raw"], 0.01)
        self.assertAlmostEqual(result["median_signed"], 0.0)

    def test_counts_only_dates_present_in_both(self):
        etoro = OrderedDict([("d1", 100.0), ("d2", 101.0), ("d3", 102.0)])
        tiingo = self.tiingo([("d2", (101.0, 100.0, 1.0)), ("d9", (1.0, 1.0, 1.0))])
        self.assertEqual(compare_series(etoro, tiingo)["overlap"], 1)

    def test_no_overlapping_dates_cannot_decide(self):
        etoro = OrderedDict([("d1", 100.0)])
        tiingo = self.tiingo([("d9", (100.0, 90.0, 1.0))])
        result = compare_series(etoro, tiingo)
        self.assertEqual(result["dividend_verdict"], "inconclusive")
        self.assertEqual(result["overlap"], 0)


class TestSplitFindings(unittest.TestCase):
    @staticmethod
    def tiingo_with_split(date, factor):
        return OrderedDict(
            [
                ("d1", (1.0, 1.0, 1.0)),
                (date, (1.0, 1.0, factor)),
                ("d3", (1.0, 1.0, 1.0)),
            ]
        )

    def test_an_unadjusted_series_shows_the_split_factor(self):
        etoro = OrderedDict([("d1", 1200.0), ("d2", 120.0), ("d3", 121.0)])
        (date, factor, observed, verdict), = split_findings(
            etoro, self.tiingo_with_split("d2", 10.0)
        )
        self.assertEqual((date, factor), ("d2", 10.0))
        self.assertAlmostEqual(observed, 10.0)
        self.assertEqual(verdict, "unadjusted")

    def test_an_adjusted_series_shows_an_ordinary_move(self):
        # The real TSLA case: a 3:1 split with no discontinuity in the series.
        etoro = OrderedDict([("d1", 120.0), ("d2", 119.0), ("d3", 121.0)])
        (_, _, observed, verdict), = split_findings(etoro, self.tiingo_with_split("d2", 3.0))
        self.assertAlmostEqual(observed, 120.0 / 119.0)
        self.assertEqual(verdict, "adjusted")

    def test_a_split_outside_the_series_is_reported_not_guessed(self):
        etoro = OrderedDict([("d1", 100.0), ("d3", 101.0)])
        (_, _, observed, verdict), = split_findings(etoro, self.tiingo_with_split("d2", 3.0))
        self.assertIsNone(observed)
        self.assertIn("not covered", verdict)

    def test_a_split_on_the_first_bar_has_no_prior_day_to_compare(self):
        etoro = OrderedDict([("d2", 120.0), ("d3", 121.0)])
        (_, _, observed, _), = split_findings(etoro, self.tiingo_with_split("d2", 3.0))
        self.assertIsNone(observed)

    def test_no_splits_means_no_findings(self):
        etoro = OrderedDict([("d1", 100.0), ("d2", 101.0)])
        self.assertEqual(split_findings(etoro, self.tiingo_with_split("d2", 1.0)), [])



if __name__ == "__main__":
    unittest.main()
