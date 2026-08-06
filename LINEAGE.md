# Beacon lineage

Tahto's first Hara/Hoplite control-plane shell is extracted from `greenways-ai/greenways-os/services/beacon` and immediately changes the product boundary from a fixed Space proxy to an application-neutral state fabric.

The source lineage is retained explicitly so reviewers can trace every inherited idea and determine which behavior was removed:

| Commit | Original change |
|---|---|
| `df16d192c80e2977e7e7447fe2734f1af08ef877` | Introduce Greenways Beacon on Hoplite |
| `7e7092862d515354e3f40091ef4303ed3b36614b` | Align Beacon with the Space discovery asset |
| `e8dc7682011281a479472ac103ad352fdeb8156b` | Read the published Space discovery record |
| `a2ee041b16f3434422fa5289d1fcdc59e46eb776` | Document the deployed discovery asset |
| `8d1a37ae76adc736be07ff32b580f28e50520bed` | Keep the implementation in the reserved `gw.*` namespace |
| `64febba8652e64abcd35e31f5f0fc7871f6ad201` | Distinguish Beacon product identity from its implementation namespace |

The final extracted Beacon blobs on Greenways OS `main` were:

```text
services/beacon/src/gw/beacon.hal       7f6816a7d25fb94cda79134a08ed534fc98c9acd
services/beacon/project.edn             14deeb8e774b0c51b1547b5ce1acc5a9521b2aa4
services/beacon/bin/greenways-beacon    a89c60e2b56d6955f0d82887775e2e7d8ea853c2
services/beacon/README.md               2ef5c39ba2f568ef74383df7f879d730289a6665
```

## Retained ideas

- Hara handlers hosted by Hoplite;
- a small inspectable discovery, health, and status surface;
- a fixed local operator port during migration;
- inert service descriptors rather than remote executable catalogues;
- explicit browser/OS authority rather than network reachability as authority; and
- a warning compatibility executable.

## Deliberately removed from core

- the immutable `/space/` proxy;
- `greenways.space` as the service or custody authority;
- Hestia and Ignatius presented as services composed behind Beacon;
- a request path whose primary purpose is remote Space discovery; and
- any assumption that a hosted service is required for local Greenways OS applications.

A future history-preserving subtree import may replace this explicit manifest when repository tooling supports cross-repository history transfer. The references above remain normative provenance either way.
