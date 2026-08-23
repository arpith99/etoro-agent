"""Unit tests for the schema pipeline scripts.

Run with:  python3 -m unittest discover -s scripts/tests -v
"""

import json
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from fetch_spec import assemble_schemas, assemble_operations, validate_refs  # noqa: E402


class TestAssembleSchemas(unittest.TestCase):
    def test_merges_chunks_in_key_order(self):
        out = assemble_schemas([{"B": {"type": "string"}}, {"A": {"type": "integer"}}])
        self.assertEqual(list(out), ["A", "B"])

    def test_rejects_duplicate_keys_across_chunks(self):
        with self.assertRaisesRegex(ValueError, "duplicate schema: A"):
            assemble_schemas([{"A": {"type": "string"}}, {"A": {"type": "string"}}])

    def test_rejects_non_object_chunk(self):
        with self.assertRaisesRegex(ValueError, "chunk 1 is not a JSON object"):
            assemble_schemas([{"A": {}}, ["not", "an", "object"]])


class TestAssembleOperations(unittest.TestCase):
    def test_concatenates_and_sorts_by_path_then_method(self):
        chunks = [
            [{"path": "/b", "method": "get", "operationId": "b"}],
            [
                {"path": "/a", "method": "post", "operationId": "ap"},
                {"path": "/a", "method": "get", "operationId": "ag"},
            ],
        ]
        out = assemble_operations(chunks)
        self.assertEqual([o["operationId"] for o in out], ["ag", "ap", "b"])

    def test_rejects_duplicate_operation(self):
        chunks = [[{"path": "/a", "method": "get"}], [{"path": "/a", "method": "get"}]]
        with self.assertRaisesRegex(ValueError, "duplicate operation: get /a"):
            assemble_operations(chunks)


class TestValidateRefs(unittest.TestCase):
    def test_rejects_dangling_ref(self):
        with self.assertRaisesRegex(ValueError, r"dangling \$ref: Missing"):
            validate_refs({"A": {"$ref": "#/components/schemas/Missing"}})

    def test_accepts_resolvable_ref(self):
        validate_refs({"A": {"$ref": "#/components/schemas/B"}, "B": {"type": "string"}})

    def test_finds_refs_nested_in_arrays_and_objects(self):
        schemas = {
            "A": {"properties": {"xs": {"type": "array", "items": {"$ref": "#/components/schemas/Gone"}}}},
            "B": {"type": "string"},
        }
        with self.assertRaisesRegex(ValueError, r"dangling \$ref: Gone"):
            validate_refs(schemas)

    def test_reports_every_dangling_ref_not_just_the_first(self):
        schemas = {"A": {"$ref": "#/components/schemas/X"}, "B": {"$ref": "#/components/schemas/Y"}}
        with self.assertRaises(ValueError) as ctx:
            validate_refs(schemas)
        self.assertIn("X", str(ctx.exception))
        self.assertIn("Y", str(ctx.exception))


class TestCommittedSnapshot(unittest.TestCase):
    """Guards the real snapshot, so a bad refresh cannot be committed silently."""

    @classmethod
    def setUpClass(cls):
        cls.spec_dir = Path(__file__).resolve().parents[2] / "docs" / "spec"
        if not (cls.spec_dir / "schemas.json").exists():
            raise unittest.SkipTest("docs/spec/ not populated yet")

    def test_schema_count_matches_meta(self):
        schemas = json.loads((self.spec_dir / "schemas.json").read_text())
        meta = json.loads((self.spec_dir / "_meta.json").read_text())
        self.assertEqual(len(schemas), meta["counts"]["schemas"])

    def test_every_ref_resolves(self):
        validate_refs(json.loads((self.spec_dir / "schemas.json").read_text()))


if __name__ == "__main__":
    unittest.main()
