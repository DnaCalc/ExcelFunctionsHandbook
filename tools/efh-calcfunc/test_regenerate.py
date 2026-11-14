"""Checks the attribution and presentation of independently recorded comparisons."""
import json
import tempfile
import unittest
from pathlib import Path

import rustindex

from regenerate import corpus_for, require_corpus_revision, shared_entry_item, state_of


class EvidenceTests(unittest.TestCase):
    def test_corpus_revision_must_match_pin(self):
        with tempfile.TemporaryDirectory() as tmp:
            (Path(tmp) / "summary.json").write_text(json.dumps({"run": {"repository": "old"}}))
            require_corpus_revision(tmp, "old")
            with self.assertRaises(SystemExit):
                require_corpus_revision(tmp, "new")

    def test_prefixed_and_plain_corpus_counts_are_combined(self):
        rows = {
            "FORECAST.ETS.STAT": {"cells": 3, "matched": 3, "worst_ulp": 0},
            "XLFN.FORECAST.ETS.STAT": {"cells": 105, "last_bit": 9, "refused": 96, "worst_ulp": 2},
        }
        corpus = corpus_for("FORECAST.ETS.STAT", rows)
        self.assertEqual(corpus["cells"], 108)
        self.assertEqual(corpus["matched"], 3)
        self.assertEqual(corpus["worst_ulp"], 2)
        label, _, detail = state_of("FORECAST.ETS.STAT", {"kind": "implemented"}, None, corpus, 0)
        self.assertEqual(label, "differs")
        self.assertIn("9 differ by at most 16 ULP", detail)
        self.assertIn("96 are refused", detail)

    def test_unread_and_excluded_bank_rows_remain_failures(self):
        bank = {"rows": 10, "matched": 7, "unread": 1, "excluded": 2}
        label, summary, detail = state_of("EXAMPLE", {"kind": "implemented"}, bank, None, 0)
        self.assertEqual(label, "differs")
        self.assertIn("3 of 10", summary)
        self.assertIn("1 were unread", detail)
        self.assertIn("2 were excluded", detail)

    def test_pending_workbook_cells_remain_failures(self):
        corpus = {"cells": 4, "matched": 3, "pending": 1}
        label, summary, detail = state_of("EXAMPLE", {"kind": "implemented"}, None, corpus, 0)
        self.assertEqual(label, "differs")
        self.assertIn("1 of 4", summary)
        self.assertIn("1 await a comparable workbook result", detail)

    def test_missing_workbook_result_is_not_called_a_refusal(self):
        corpus = {"cells": 4, "matched": 3, "failed": 1}
        label, summary, detail = state_of("EXAMPLE", {"kind": "implemented"}, None, corpus, 0)
        self.assertEqual(label, "differs")
        self.assertIn("1 of 4", summary)
        self.assertIn("1 failed or were missing", detail)
        self.assertNotIn("refused", detail)

    def test_shared_family_id_selects_the_attached_kernel(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            src = root / "crates/calc-func/src/catalogue"
            src.mkdir(parents=True)
            (src / "demo.rs").write_text("""use calc_catalogue::ported::demo::{CODE, ENTRIES as DECLARATIONS};
ported_shared_family! {
  1 first |s, a, sv| { first_kernel(s, a) }
  2 second |s, a, sv| { second_kernel(s, a) }
}
""")
            declarations = root / "crates/calc-catalogue/src"
            declarations.mkdir(parents=True)
            (declarations / "ported.rs").write_text("pub mod demo { pub const CODE: u16 = 123; }")
            index = rustindex.Index(src.parent)
            item, _ = shared_entry_item(index, root, {"id": (123 << 16) | 2, "spelling": "SECOND"})
            self.assertIn("second_kernel", item.text)
            self.assertNotIn("first_kernel", item.text)

    def test_missing_attribution_does_not_create_measurements(self):
        self.assertIsNone(corpus_for("ABS", {"XLFN.ABSENT": {"cells": 1}}))


if __name__ == "__main__":
    unittest.main()
