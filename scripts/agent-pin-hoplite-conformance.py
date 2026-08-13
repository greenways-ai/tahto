#!/usr/bin/env python3
from pathlib import Path

WORKFLOW = Path(".github/workflows/bootstrap-conformance.yml")
REVISION_FILE = Path("packaging/hoplite-revision")
REVISION = "d51c5954e427ea84439477135b970a3e1145c190"

text = WORKFLOW.read_text(encoding="utf-8")
job = "\n  hoplite-state:\n"
if job not in text:
    raise SystemExit("hoplite-state job missing")
block_start = text.index(job) + 1
block_end = text.find("\n  ", block_start + len("  hoplite-state:\n"))
if block_end == -1:
    block_end = len(text)
block = text[block_start:block_end]
if "HOPLITE_REVISION:" not in block:
    anchor = "  hoplite-state:\n    runs-on: ubuntu-latest\n"
    if anchor not in text:
        raise SystemExit("hoplite-state runs-on anchor missing")
    text = text.replace(
        anchor,
        anchor + f"    env:\n      HOPLITE_REVISION: {REVISION}\n",
        1,
    )

old_checkout = """          repository: greenways-ai/hoplite
          ref: main
          path: .dependencies/technology/hoplite
"""
new_checkout = """          repository: greenways-ai/hoplite
          ref: ${{ env.HOPLITE_REVISION }}
          path: .dependencies/technology/hoplite
"""
if old_checkout not in text:
    raise SystemExit("Hoplite checkout anchor missing")
text = text.replace(old_checkout, new_checkout, 1)

legacy_path = ".dependencies/technology/hoplite/migration/value/src/hoplite/value.hal"
reviewed_path = ".dependencies/technology/hoplite/core/lib/src/hoplite/value.hal"
if legacy_path not in text:
    raise SystemExit("temporary Hoplite value path missing")
text = text.replace(legacy_path, reviewed_path, 1)

verify = """      - name: Verify the reviewed Hoplite revision
        run: |
          test "$(git -C .dependencies/technology/hoplite rev-parse HEAD)" = "$HOPLITE_REVISION"
          test "$(tr -d '\\n' < packaging/hoplite-revision)" = "$HOPLITE_REVISION"
"""
if "Verify the reviewed Hoplite revision" not in text:
    marker = "      - name: Copy Hoplite sources into the Hara project\n"
    if marker not in text:
        marker = "      - name: Stage Hoplite sources\n"
    if marker not in text:
        raise SystemExit("Hoplite copy step anchor missing")
    text = text.replace(marker, verify + marker, 1)

WORKFLOW.write_text(text, encoding="utf-8")
REVISION_FILE.parent.mkdir(parents=True, exist_ok=True)
REVISION_FILE.write_text(REVISION + "\n", encoding="utf-8")
