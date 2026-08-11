---
title: History and synchronization
description: Immutable commits, divergent heads, closure transfer, and recovery.
---
# History and synchronization

Tahto history uses verified immutable commits, strict per-device sequence slots, signed compare-and-swap heads, divergent-head preservation, backup closure pins, deterministic restore manifests, and exact receipt evidence.

Synchronization negotiates the missing closure rather than transferring ambient mutable state. Application-authored merge policy is required whenever multiple valid heads diverge.
