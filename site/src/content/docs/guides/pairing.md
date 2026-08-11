---
title: Pair devices
description: Prepare and complete identity-only device enrolment.
---
# Pair devices

1. An operator creates a short-lived invitation through the loopback-only `bin/tahto invite` boundary.
2. Greenways OS submits the invitation to `pairing/prepare` and receives an exact intent.
3. Greenways OS signs that intent with the enrolling device.
4. `pairing/complete` verifies the signature and consumes the invitation.

Only the invitation digest enters durable metadata. Enrolment establishes device identity; it does not grant administrator or application authority.
