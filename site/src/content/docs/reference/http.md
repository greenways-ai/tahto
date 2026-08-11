---
title: HTTP surface
description: Current discovery, status, diagnostics, and pairing routes.
---
# HTTP surface

| Method | Route | Purpose |
| --- | --- | --- |
| `GET` | `/.well-known/tahto` | Discover the node |
| `GET` | `/tahto/0-alpha/health` | Read provider health |
| `GET` | `/tahto/0-alpha/status` | Read installed component status |
| `POST` | `/tahto/0-alpha/diagnostics` | Return signed aggregate diagnostics |
| `POST` | `/tahto/0-alpha/pairing/prepare` | Prepare an enrolment intent |
| `POST` | `/tahto/0-alpha/pairing/complete` | Verify and complete enrolment |

The semantic operation kernels are not yet advertised as installed HTTP routes.
