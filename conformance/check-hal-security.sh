#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

grep -F '## Readiness matrix' README.md
grep -F '| Semantic value profiles | ready |' README.md
grep -F 'service: hoplite.store' protocol/metadata-store.md
grep -F '"metadataHost" "generic-installed-hara-store-no-request-selection"' src/tahto/protocol/descriptor.hal
grep -F '"objectResponseSource" "request-scoped-transport-ready"' src/tahto/protocol/descriptor.hal
grep -F '"portableTwoDeviceLaw" "ready"' src/tahto/protocol/descriptor.hal
grep -F '"semanticFabric" "kernel-ready:service-pending"' src/tahto/protocol/descriptor.hal
grep -F '"semanticRoutes" "not-exposed"' src/tahto/protocol/descriptor.hal
grep -F '"canonicalValueProvider" "not-installed"' src/tahto/protocol/descriptor.hal

grep -F '[tahto.node.console :as console]' src/tahto/node/app.hal
grep -F ":console #'console/dispatch" src/tahto/node/app.hal
grep -F '(defn request-envelope?' src/tahto/console/contract.hal
grep -F '(not (contract/request-envelope? request))' src/tahto/node/console.hal
! grep -F '[grant command input]' src/tahto/node/console.hal

! grep -R -F '(def native-abi' src/tahto
! grep -R -F ':native-abi' src/tahto
! grep -R -F '(def host-service "tahto.' src/tahto
! grep -R -F '"tahto-metadata-store/0-alpha"' src/tahto
! grep -R -F ':source-handle' src/tahto/semantic
! grep -R -F ':provider' src/tahto/semantic
! grep -R -F ':path' src/tahto/semantic
! grep -R -F ':url' src/tahto/semantic
! grep -F 'object-transfer execution and native response-source streaming remain' README.md
grep -F '(ns tahto.node.request-auth' src/tahto/node/request_auth.hal
grep -F '"verify-signature"' src/tahto/node/request_auth.hal
grep -F '(fresh? request now-seconds)' src/tahto/node/request_auth.hal
