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
grep -F '(ns tahto.semantic.value-source' src/tahto/semantic/value_source.hal
grep -F '"tahto.semantic-value-source-request/0-alpha"' src/tahto/semantic/value_source.hal
grep -F '(ns tahto.client' src/tahto/client.hal
grep -F '[tahto.semantic.service :as semantic-service]' src/tahto/client.hal
grep -F '[tahto.semantic.value-source :as value-source]' src/tahto/client.hal
grep -F '(semantic-service/dispatch-verified' src/tahto/client.hal
grep -F '"tahto.semantic/value-source"' src/tahto/client.hal
! grep -F 'submit-payload-fields' src/tahto/client.hal
! grep -F '[tahto.semantic.read :as semantic-read]' src/tahto/client.hal
! grep -F '[tahto.semantic.prepare :as semantic-prepare]' src/tahto/client.hal
! grep -F '[tahto.semantic.submit :as semantic-submit]' src/tahto/client.hal
grep -F '(ns tahto.change-feed.facade' src/tahto/change_feed/facade.hal
grep -F '"tahto.semantic-change-checkpoint/0-alpha"' src/tahto/change_feed/facade.hal
grep -F '(core/subscribe' src/tahto/change_feed/facade.hal
grep -F '(core/current-head' src/tahto/change_feed/facade.hal
grep -F '(provider/state-revision' src/tahto/change_feed/facade.hal
grep -F ':tahto/change-feed {:export/namespace tahto.change-feed.facade}' project.edn
grep -F '(ns tahto.profile.chats.model' src/tahto/profile/chats/model.hal
grep -F '"greenways.chat/0-alpha"' src/tahto/profile/chats/model.hal
grep -F '"greenways.chat-message/0-alpha"' src/tahto/profile/chats/model.hal
grep -F '"greenways:tahto/collection/chats"' src/tahto/profile/chats/model.hal
grep -F '"hara:greenways/chats-profile"' src/tahto/profile/chats/model.hal
grep -F '(semantic/semantic-link-vector?' src/tahto/profile/chats/model.hal
grep -F '(ns tahto.profile.chats.collection' src/tahto/profile/chats/collection.hal
grep -F '"greenways.chat-collection/0-alpha"' src/tahto/profile/chats/collection.hal
grep -F '"collection/chats"' src/tahto/profile/chats/collection.hal
grep -F 'max-chat-count 1000' src/tahto/profile/chats/collection.hal
grep -F '(semantic/semantic-link-vector?' src/tahto/profile/chats/collection.hal
grep -F '(ns tahto.profile.chats.query' src/tahto/profile/chats/query.hal
grep -F '#{"recent" "chat" "messages"}' src/tahto/profile/chats/query.hal
grep -F 'greenways.chats/cursor-out-of-range' src/tahto/profile/chats/query.hal
grep -F '(ns tahto.profile.chats.read' src/tahto/profile/chats/read.hal
grep -F '(semantic-read/read' src/tahto/profile/chats/read.hal
grep -F '(canonical/read-with' src/tahto/profile/chats/read.hal
grep -F '(chat-collection/projection?' src/tahto/profile/chats/read.hal
grep -F 'greenways.chats/semantic-read-mismatch' src/tahto/profile/chats/read.hal
grep -F ':branches branches' src/tahto/profile/chats/read.hal
! grep -R -E 'credential|private-path|source-handle|provider-path|route' src/tahto/profile/chats

grep -F '(ns tahto.client-test' test/tahto/client_test.hal
grep -F '(ns tahto.change-feed.facade-test' test/tahto/change_feed/facade_test.hal
grep -F '(ns tahto.profile.chats.model-test' test/tahto/profile/chats/model_test.hal
grep -F '(ns tahto.profile.chats.collection-test' test/tahto/profile/chats/collection_test.hal
grep -F '(ns tahto.profile.chats.query-test' test/tahto/profile/chats/query_test.hal
grep -F '(ns tahto.profile.chats.read-test' test/tahto/profile/chats/read_test.hal
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
grep -F '(ns tahto.semantic.value-source-test' test/tahto/semantic/value_source_test.hal
grep -F '(ns tahto.sync.two-device-object-test' test/tahto/sync/two_device_object_test.hal
grep -F '(vault/accept-install candidate translated)' src/tahto/store/upload.hal
