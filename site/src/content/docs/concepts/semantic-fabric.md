---
title: Semantic fabric
description: Values, objects, stable indexes, roots, and semantic operations.
---
# Semantic fabric

An exact installed specification package identifies the validator for a canonical application value. Tahto admits the value as a semantic object, records its stable identity and typed links, constructs complete indexes and roots, and commits those roots through ordinary Tahto history.

## Operations

- `semantic.read` selects a bounded canonical value.
- `semantic.prepare` validates and prepares a deterministic change without committing it.
- `semantic.submit` atomically admits the prepared change through the history boundary.

Kernel availability does not imply an installed HTTP service. Consult [status and roadmap](../project/status/) before depending on a route.
