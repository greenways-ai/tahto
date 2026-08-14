#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

grep -F '(def host-service "hoplite.blob")' src/tahto/store/capability.hal
grep -F '(def host-service "hoplite.store")' src/tahto/store/provider.hal
grep -F '(ns tahto.store.upload' src/tahto/store/upload.hal
grep -F '(ns tahto.store.response-source' src/tahto/store/response_source.hal
grep -F '[hoplite.response-source :as portable]' src/tahto/store/response_source.hal
grep -F 'portable/response-source?' src/tahto/store/response_source.hal
grep -F '(ns tahto.store.memory-blob' src/tahto/store/memory_blob.hal
grep -F '(ns tahto.store.memory-store' src/tahto/store/memory_store.hal
grep -F '(ns tahto.semantic.model' src/tahto/semantic/model.hal
grep -F '(ns tahto.semantic.admission' src/tahto/semantic/admission.hal
grep -F '(ns tahto.semantic.index' src/tahto/semantic/index.hal
grep -F '(ns tahto.semantic.root' src/tahto/semantic/root.hal
grep -F '(ns tahto.semantic.history' src/tahto/semantic/history.hal
grep -F '(ns tahto.semantic.canonical-value' src/tahto/semantic/canonical_value.hal
grep -F '(ns tahto.semantic.read' src/tahto/semantic/read.hal
grep -F '(ns tahto.semantic.prepare' src/tahto/semantic/prepare.hal
grep -F '(ns tahto.semantic.submit' src/tahto/semantic/submit.hal
grep -F '(ns tahto.semantic.service' src/tahto/semantic/service.hal
grep -F '"tahto.semantic-service-request/0-alpha"' src/tahto/semantic/service.hal
grep -F '(def operation "semantic.read")' src/tahto/semantic/read.hal
grep -F '"semantic.prepare"' src/tahto/semantic/prepare.hal
grep -F '"semantic.submit"' src/tahto/semantic/submit.hal
grep -F '"semantic.read"' src/tahto/protocol/validate.hal
grep -F '"semantic.prepare"' src/tahto/protocol/validate.hal
grep -F '"semantic.submit"' src/tahto/protocol/validate.hal
grep -F '[hoplite.value :as portable]' src/tahto/semantic/canonical_value.hal
grep -F 'portable/verification-request' src/tahto/semantic/canonical_value.hal
grep -F '(def admission-fields' src/tahto/semantic/admission.hal
grep -F ':semantic-objects' src/tahto/semantic/admission.hal
grep -F ':semantic-indexes' src/tahto/semantic/index.hal
grep -F ':semantic-roots' src/tahto/semantic/root.hal
