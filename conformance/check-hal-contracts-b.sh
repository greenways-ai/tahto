#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

grep -F '"tahto.schema-ref/0-alpha"' src/tahto/semantic/model.hal
grep -F '"tahto.canonical-value-verification/0-alpha"' src/tahto/semantic/model.hal
grep -F '"tahto.semantic-link/0-alpha"' src/tahto/semantic/model.hal
grep -F '"tahto.semantic-object/0-alpha"' src/tahto/semantic/model.hal
grep -F '"tahto.semantic-index/0-alpha"' src/tahto/semantic/model.hal
grep -F '"tahto.semantic-root/0-alpha"' src/tahto/semantic/model.hal
grep -F '"tahto.semantic-commit-intent/0-alpha"' src/tahto/semantic/prepare.hal
grep -F '"tahto.semantic-head-intent/0-alpha"' src/tahto/semantic/prepare.hal
grep -F '"tahto.semantic-prepare-result/0-alpha"' src/tahto/semantic/prepare.hal
grep -F '"tahto.semantic-submit-result/0-alpha"' src/tahto/semantic/submit.hal
grep -F '(transaction/execute' src/tahto/semantic/submit.hal
grep -F '(provider/prepare-compare-and-swap' src/tahto/semantic/submit.hal
grep -F '(semantic-submit/submit-transition' src/tahto/semantic/route.hal
grep -F '(ns tahto.semantic.model-test' test/tahto/semantic/model_test.hal
grep -F '(ns tahto.semantic.admission-test' test/tahto/semantic/admission_test.hal
grep -F '(ns tahto.semantic.index-root-test' test/tahto/semantic/index_root_test.hal
grep -F '(ns tahto.semantic.history-test' test/tahto/semantic/history_test.hal
grep -F '(ns tahto.semantic.canonical-value-test' test/tahto/semantic/canonical_value_test.hal
grep -F '(ns tahto.semantic.read-test' test/tahto/semantic/read_test.hal
grep -F '(ns tahto.semantic.prepare-test' test/tahto/semantic/prepare_test.hal
grep -F '(ns tahto.semantic.submit-test' test/tahto/semantic/submit_test.hal
grep -F '(ns tahto.semantic.submit-context-test' test/tahto/semantic/submit_context_test.hal
grep -F '(ns tahto.semantic.service-test' test/tahto/semantic/service_test.hal
grep -F '(ns tahto.sync.two-device-object-test' test/tahto/sync/two_device_object_test.hal
grep -F '(vault/accept-install candidate translated)' src/tahto/store/upload.hal
