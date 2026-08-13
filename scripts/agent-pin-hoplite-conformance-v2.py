#!/usr/bin/env python3
from pathlib import Path

WORKFLOW = Path(".github/workflows/bootstrap-conformance.yml")
REVISION_FILE = Path("packaging/hoplite-revision")
REVISION = "d51c5954e427ea84439477135b970a3e1145c190"

text = WORKFLOW.read_text(encoding="utf-8")

job_anchor = "  hoplite-state:\n    runs-on: ubuntu-latest\n"
if "HOPLITE_REVISION:" not in text:
    if text.count(job_anchor) != 1:
        raise SystemExit("hoplite-state job anchor changed")
    text = text.replace(
        job_anchor,
        job_anchor + f"    env:\n      HOPLITE_REVISION: {REVISION}\n",
        1,
    )

checkout_old = """          repository: greenways-ai/hoplite
          ref: main
          path: .dependencies/technology/hoplite
"""
checkout_new = """          repository: greenways-ai/hoplite
          ref: ${{ env.HOPLITE_REVISION }}
          path: .dependencies/technology/hoplite
"""
if checkout_old in text:
    text = text.replace(checkout_old, checkout_new, 1)
elif checkout_new not in text:
    raise SystemExit("Hoplite checkout anchor changed")

legacy_path = ".dependencies/technology/hoplite/migration/value/src/hoplite/value.hal"
reviewed_path = ".dependencies/technology/hoplite/core/lib/src/hoplite/value.hal"
if legacy_path in text:
    text = text.replace(legacy_path, reviewed_path, 1)
elif reviewed_path not in text:
    raise SystemExit("Hoplite value source path changed")

verify_step = """      - name: Verify the reviewed Hoplite revision
        run: |
          test "$(git -C .dependencies/technology/hoplite rev-parse HEAD)" = "$HOPLITE_REVISION"
          test "$(tr -d '\\n' < packaging/hoplite-revision)" = "$HOPLITE_REVISION"
"""
if "Verify the reviewed Hoplite revision" not in text:
    anchors = [
        "      - name: Copy Hoplite sources into the Hara project\n",
        "      - name: Stage Hoplite sources\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, verify_step + anchor, 1)
            break
    else:
        raise SystemExit("Hoplite source staging step changed")

WORKFLOW.write_text(text, encoding="utf-8")
REVISION_FILE.parent.mkdir(parents=True, exist_ok=True)
REVISION_FILE.write_text(REVISION + "\n", encoding="utf-8")
