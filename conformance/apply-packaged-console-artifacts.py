#!/usr/bin/env python3
from pathlib import Path

root = Path(__file__).resolve().parents[1]


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(
            f"{path}: expected one replacement, found {count}: {old[:100]!r}"
        )
    path.write_text(text.replace(old, new, 1))


app = root / "src/tahto/node/app.hal"
replace_once(
    app,
    '''  (:require [hoplite.core :as h]
            [tahto.node.console :as console]''',
    '''  (:require [hoplite.core :as h]
            [tahto.console.contract :as console-contract]
            [tahto.node.console :as console]''',
)
replace_once(
    app,
    ''':console #'console/dispatch''',
    ''':console
     {:handler #'console/dispatch
      :client "tahto.console"
      :descriptors console-contract/command-descriptors
      :grant
      {:protocol console-contract/grant-protocol
       :console "console.template"
       :commands console-contract/command-names
       :write false}}''',
)

security = root / "conformance/check-hal-security.sh"
replace_once(
    security,
    '''grep -F '[tahto.node.console :as console]' src/tahto/node/app.hal
grep -F ":console #'console/dispatch" src/tahto/node/app.hal''',
    '''grep -F '[tahto.console.contract :as console-contract]' src/tahto/node/app.hal
grep -F '[tahto.node.console :as console]' src/tahto/node/app.hal
grep -F ":handler #'console/dispatch" src/tahto/node/app.hal
grep -F ':client "tahto.console"' src/tahto/node/app.hal
grep -F ':descriptors console-contract/command-descriptors' src/tahto/node/app.hal
grep -F ':commands console-contract/command-names' src/tahto/node/app.hal
grep -F ':write false' src/tahto/node/app.hal''',
)
