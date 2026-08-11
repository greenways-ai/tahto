---
title: HTTP surface
description: Current discovery, status, diagnostics, and pairing routes.
---
# HTTP surface

| Method | Route | Purpose |
| --- | --- | --- |
| `GET` | `/.well-known/tahto` | Discover the node |
| `GET` | `/tahto/v1/health` | Read provider health |
| `GET` | `/tahto/v1/status` | Read installed component status |
| `POST` | `/tahto/v1/diagnostics` | Return signed aggregate diagnostics |
| `POST` | `/tahto/v1/pairing/prepare` | Prepare an enrolment intent |
| `POST` | `/tahto/v1/pairing/complete` | Verify and complete enrolment |

The semantic operation kernels are not yet advertised as installed HTTP routes.
