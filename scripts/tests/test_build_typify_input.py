"""Unit tests for the extraction/transform stage.

Run with:  python3 -m unittest discover -s scripts/tests -v
"""

import json
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from build_typify_input import (  # noqa: E402
    ALL_TAGS,
    apply_overrides,
    build_domains,
    load_overrides,
    normalize_enums,
    normalize_exclusive_bounds,
    normalize_nullable,
    rust_type_name,
    slugify,
    transitive_deps,
)


class TestSlugify(unittest.TestCase):
    def test_collapses_separators(self):
        self.assertEqual(slugify("Trading - Real"), "trading_real")
        self.assertEqual(slugify("Sub-Accounts - eToro Trading"), "sub_accounts_etoro_trading")
        self.assertEqual(slugify("Market Data"), "market_data")

    def test_explicit_overrides_win(self):
        self.assertEqual(slugify("PortfolioSearch"), "portfolio_search")
        self.assertEqual(slugify("PI Data"), "pi_data")
        self.assertEqual(slugify("SSO - Applications"), "sso_applications")

    def test_every_known_tag_is_a_valid_rust_ident(self):
        for tag in ALL_TAGS:
            self.assertRegex(slugify(tag), r"^[a-z][a-z0-9_]*$", f"bad slug for {tag!r}")

    def test_slugs_are_unique_across_tags(self):
        slugs = [slugify(tag) for tag in ALL_TAGS]
        duplicates = {s for s in slugs if slugs.count(s) > 1}
        self.assertEqual(duplicates, set(), f"tag slugs collide: {duplicates}")

    def test_slug_is_not_a_rust_keyword(self):
        # A tag named e.g. "Type" or "Match" would produce an unusable module.
        keywords = {"type", "match", "move", "ref", "self", "super", "crate", "mod", "use", "box"}
        for tag in ALL_TAGS:
            self.assertNotIn(slugify(tag), keywords)


class TestRustTypeName(unittest.TestCase):
    """typify renames schemas to UpperCamelCase; the facades must follow."""

    def test_lower_camel_schema_names(self):
        self.assertEqual(rust_type_name("meResponse"), "MeResponse")
        self.assertEqual(rust_type_name("candlesResponse"), "CandlesResponse")
        self.assertEqual(rust_type_name("gainEntry"), "GainEntry")
        self.assertEqual(rust_type_name("getUserDailyGainResponse"), "GetUserDailyGainResponse")

    def test_underscores_are_dropped(self):
        self.assertEqual(rust_type_name("AgentPortfolioApi_ErrorResponse"), "AgentPortfolioApiErrorResponse")
        self.assertEqual(rust_type_name("BalanceAggregatorApi_AccountType"), "BalanceAggregatorApiAccountType")

    def test_already_pascal_names_are_unchanged(self):
        for name in ("PublicAggregatedInfoAccountStatus", "WatchlistsResponse", "Market", "SvgAvatar"):
            self.assertEqual(rust_type_name(name), name)

    def test_acronym_runs_are_camelised_like_heck(self):
        self.assertEqual(rust_type_name("NOC_NOF_RFI"), "NocNofRfi")


class TestOverrideMerge(unittest.TestCase):
    def test_deep_merges_into_existing_schema(self):
        base = {"A": {"type": "integer", "description": "d"}}
        merged = apply_overrides(base, {"A": {"x-rust-type": {"path": "p"}}})
        self.assertEqual(merged["A"]["description"], "d")
        self.assertEqual(merged["A"]["x-rust-type"]["path"], "p")

    def test_unknown_schema_name_is_an_error(self):
        with self.assertRaisesRegex(ValueError, "override targets unknown schema: Ghost"):
            apply_overrides({}, {"Ghost": {}})

    def test_does_not_mutate_the_input(self):
        base = {"A": {"type": "integer"}}
        apply_overrides(base, {"A": {"x-rust-type": {"path": "p"}}})
        self.assertNotIn("x-rust-type", base["A"])

    def test_same_schema_stamped_in_two_files_is_an_error(self):
        with self.assertRaisesRegex(ValueError, r"A .*stamped in both"):
            load_overrides([
                ("a.json", {"schemas": {"A": {"x-rust-type": {}}}}),
                ("b.json", {"schemas": {"A": {"x-rust-type": {}}}}),
            ])

    def test_notes_are_not_carried_into_schemas(self):
        stamps = load_overrides([("a.json", {"_notes": {"n": ["x"]}, "schemas": {"A": {}}})])
        self.assertEqual(stamps, {"A": {}})


class TestTransitiveDeps(unittest.TestCase):
    POOL = {
        "Root": {"properties": {"child": {"$ref": "#/components/schemas/Child"}}},
        "Child": {"items": {"$ref": "#/components/schemas/GrandChild"}},
        "GrandChild": {"type": "string"},
        "Unrelated": {"type": "string"},
    }

    def test_walks_the_whole_chain(self):
        self.assertEqual(transitive_deps({"Root"}, self.POOL), {"Root", "Child", "GrandChild"})

    def test_survives_a_reference_cycle(self):
        pool = {
            "A": {"$ref": "#/components/schemas/B"},
            "B": {"$ref": "#/components/schemas/A"},
        }
        self.assertEqual(transitive_deps({"A"}, pool), {"A", "B"})


class TestBuildDomains(unittest.TestCase):
    OPERATIONS = [
        {
            "path": "/api/v1/me",
            "method": "get",
            "operationId": "getMe",
            "tags": ["Identity"],
            "responseSchemas": ["#/components/schemas/meResponse"],
            "requestSchemas": [],
        }
    ]
    SCHEMAS = {
        "meResponse": {"properties": {"a": {"$ref": "#/components/schemas/Nested"}}},
        "Nested": {"type": "string"},
        "Orphan": {"type": "string"},
    }

    def test_seeds_from_operation_refs_and_closes_over_them(self):
        domains = build_domains(self.OPERATIONS, self.SCHEMAS)
        self.assertEqual(domains["Identity"]["slug"], "identity")
        self.assertEqual(sorted(domains["Identity"]["schemas"]), ["Nested", "meResponse"])

    def test_reports_schemas_no_operation_reaches(self):
        domains = build_domains(self.OPERATIONS, self.SCHEMAS)
        self.assertIn("Orphan", domains["_orphans"])


class TestTransformOrder(unittest.TestCase):
    """The enum/nullable transforms carry behaviour the old pipeline documented."""

    def test_integer_enum_with_string_values_becomes_a_string_enum(self):
        node = {"type": "integer", "enum": ["Open", "Close"]}
        normalize_enums(node)
        self.assertEqual(node["type"], "string")

    def test_x_rust_type_protects_an_integer_enum_from_rewriting(self):
        node = {"type": "integer", "enum": ["Open", "Close"], "x-rust-type": {"path": "p"}}
        normalize_enums(node)
        self.assertEqual(node["type"], "integer")

    def test_x_enum_names_is_renamed_for_typify(self):
        node = {"type": "integer", "enum": [1, 2], "x-enumNames": ["One", "Two"]}
        normalize_enums(node)
        self.assertEqual(node["x-enum-varnames"], ["One", "Two"])
        self.assertNotIn("x-enumNames", node)

    def test_nullable_becomes_a_type_union(self):
        node = {"type": "string", "nullable": True}
        normalize_nullable(node)
        self.assertEqual(node["type"], ["string", "null"])

    def test_untyped_nullable_stays_untyped(self):
        # Forcing type:null here made typify emit `()`, which rejects every
        # real value. Discussion.reason hit exactly this.
        node = {"nullable": True, "description": "anything"}
        normalize_nullable(node)
        self.assertNotIn("type", node)

    def test_boolean_exclusive_minimum_becomes_the_2020_12_numeric_form(self):
        # OpenAPI 3.0 / draft-04 spell this as a boolean modifier on `minimum`;
        # JSON Schema 2020-12 wants the bound itself. typify parses 2020-12 and
        # rejects the boolean outright.
        node = {"type": "number", "minimum": 0, "exclusiveMinimum": True}
        normalize_exclusive_bounds(node)
        self.assertEqual(node, {"type": "number", "exclusiveMinimum": 0})

    def test_exclusive_minimum_false_just_leaves_an_inclusive_bound(self):
        node = {"type": "number", "minimum": 0, "exclusiveMinimum": False}
        normalize_exclusive_bounds(node)
        self.assertEqual(node, {"type": "number", "minimum": 0})

    def test_boolean_exclusive_maximum_is_handled_too(self):
        node = {"type": "number", "maximum": 10, "exclusiveMaximum": True}
        normalize_exclusive_bounds(node)
        self.assertEqual(node, {"type": "number", "exclusiveMaximum": 10})

    def test_numeric_exclusive_bound_is_left_alone(self):
        node = {"type": "number", "exclusiveMinimum": 0}
        normalize_exclusive_bounds(node)
        self.assertEqual(node, {"type": "number", "exclusiveMinimum": 0})

    def test_boolean_without_a_paired_bound_is_dropped(self):
        # Meaningless in either dialect; keeping it would fail typify's parse.
        node = {"type": "number", "exclusiveMinimum": True}
        normalize_exclusive_bounds(node)
        self.assertEqual(node, {"type": "number"})

    def test_nullable_ref_becomes_a_oneOf_with_null(self):
        node = {"$ref": "#/components/schemas/X", "nullable": True}
        normalize_nullable(node)
        self.assertNotIn("$ref", node)
        self.assertEqual(node["oneOf"], [{"$ref": "#/components/schemas/X"}, {"type": "null"}])


if __name__ == "__main__":
    unittest.main()
