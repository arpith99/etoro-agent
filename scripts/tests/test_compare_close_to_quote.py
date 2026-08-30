"""Unit tests for the close-versus-quote classification.

Covers the decision logic only; the HTTP calls are not exercised.

Run with:  python3 -m unittest discover -s scripts/tests -v
"""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from compare_close_to_quote import classify, spread_fraction  # noqa: E402


class TestClassify(unittest.TestCase):
    def test_a_close_on_the_bid_is_named(self):
        # The hypothesis under test: eToro reports the bid.
        label, offsets = classify(close=99.0, bid=99.0, ask=101.0)
        self.assertEqual(label, "bid")
        self.assertAlmostEqual(offsets["bid"], 0.0)

    def test_a_close_on_the_mid_is_named(self):
        label, _ = classify(close=100.0, bid=99.0, ask=101.0)
        self.assertEqual(label, "mid")

    def test_a_close_on_the_ask_is_named(self):
        label, _ = classify(close=101.0, bid=99.0, ask=101.0)
        self.assertEqual(label, "ask")

    def test_a_close_between_levels_is_described_not_forced(self):
        # Reporting the nearest level as a match would turn "somewhere in the
        # spread" into a false confirmation of whichever edge is closer.
        # 99.2 is unambiguously nearer the bid: 0.20% away against 0.80%
        # from the mid. (99.5 is not -- it sits fractionally nearer the mid,
        # 0.500% against 0.505%, which is how this test first went wrong.)
        label, _ = classify(close=99.2, bid=99.0, ask=101.0)
        self.assertIn("inside the spread", label)
        self.assertIn("bid", label)

    def test_a_close_outside_the_quote_is_reported_as_such(self):
        self.assertEqual(classify(close=98.0, bid=99.0, ask=101.0)[0], "below the bid")
        self.assertEqual(classify(close=102.0, bid=99.0, ask=101.0)[0], "above the ask")

    def test_a_tight_spread_still_distinguishes_bid_from_mid(self):
        # The real case: AAPL's spread is about 0.003%, well inside tolerance,
        # so bid, mid and ask all match and the test cannot separate them.
        label, offsets = classify(close=319.69, bid=319.69, ask=319.70)
        self.assertEqual(label, "bid")
        # Every level is within tolerance here, which is the point: a tight
        # quote cannot answer the question.
        self.assertTrue(all(abs(v) < 0.0002 for v in offsets.values()))


class TestSpreadFraction(unittest.TestCase):
    def test_spread_is_measured_against_the_mid(self):
        self.assertAlmostEqual(spread_fraction(99.0, 101.0), 2.0 / 100.0)

    def test_a_zero_quote_does_not_divide_by_zero(self):
        self.assertNotEqual(spread_fraction(0.0, 0.0), spread_fraction(0.0, 0.0))


if __name__ == "__main__":
    unittest.main()
