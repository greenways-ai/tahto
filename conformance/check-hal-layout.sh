#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

test ! -e src-python
test ! -e test-python
test ! -e .github/workflows/materialize-native-metadata-lock.yml
test ! -e .github/workflows/promote-native-metadata-sqlite.yml
test -z "$(find src test -type f ! -name '*.hal' ! -name 'README.md' -print)"
! grep -F ':hoplite/authentication' project.edn
! grep -R -E 'extern crate|use [A-Za-z0-9_]+::|#include' src test

test -f src/tahto/store/model.hal
test -f src/tahto/store/host.hal
test -f src/tahto/store/capability.hal
test -f src/tahto/store/upload.hal
test -f src/tahto/store/response_source.hal
test -f src/tahto/store/memory_blob.hal
test -f src/tahto/store/memory_store.hal
test -f src/tahto/store/vault.hal
test -f src/tahto/store/graph.hal
test -f src/tahto/store/history.hal
test -f src/tahto/store/transaction.hal
test -f src/tahto/store/provider.hal
test -f src/tahto/semantic/model.hal
test -f src/tahto/semantic/admission.hal
test -f src/tahto/semantic/index.hal
test -f src/tahto/semantic/root.hal
test -f src/tahto/semantic/history.hal
test -f src/tahto/semantic/canonical_value.hal
test -f src/tahto/semantic/read.hal
test -f src/tahto/semantic/prepare.hal
test -f src/tahto/semantic/submit.hal
test -f src/tahto/semantic/service.hal
test -f src/tahto/semantic/value_source.hal
test -f src/tahto/sync/device.hal
test -f src/tahto/sync/session.hal
test -f src/tahto/service/state.hal
test -f src/tahto/protocol/validate.hal

test -f test/tahto/store/vault_test.hal
test -f test/tahto/store/graph_test.hal
test -f test/tahto/store/history_test.hal
test -f test/tahto/store/vault_hardening_test.hal
test -f test/tahto/store/transaction_test.hal
test -f test/tahto/store/provider_test.hal
test -f test/tahto/store/capability_test.hal
test -f test/tahto/store/upload_test.hal
test -f test/tahto/store/response_source_test.hal
test -f test/tahto/store/memory_blob_test.hal
test -f test/tahto/store/memory_store_test.hal
test -f test/tahto/semantic/model_test.hal
test -f test/tahto/semantic/admission_test.hal
test -f test/tahto/semantic/index_root_test.hal
test -f test/tahto/semantic/history_test.hal
test -f test/tahto/semantic/canonical_value_test.hal
test -f test/tahto/semantic/read_test.hal
test -f test/tahto/semantic/prepare_test.hal
test -f test/tahto/semantic/submit_test.hal
test -f test/tahto/semantic/submit_context_test.hal
test -f test/tahto/semantic/service_test.hal
test -f test/tahto/semantic/value_source_test.hal
test -f test/tahto/sync/device_test.hal
test -f test/tahto/sync/session_test.hal
test -f test/tahto/sync/two_device_object_test.hal
test -f test/tahto/service/state_test.hal

test -f protocol/services.md
test -f protocol/transactions.md
test -f protocol/metadata-store.md
test -f protocol/metadata-host.md
test -f protocol/host-capabilities.md
test -f protocol/upload-integration.md
test -f protocol/response-sources.md
test -f protocol/two-device-object-transfer.md
test -f protocol/semantic-values.md
test -f protocol/semantic-admission.md
test -f protocol/semantic-index-roots.md
test -f protocol/semantic-history.md
test -f protocol/canonical-values.md
test -f protocol/semantic-read.md
test -f protocol/semantic-prepare.md
test -f protocol/semantic-submit.md
test -f protocol/semantic-value-source.md
