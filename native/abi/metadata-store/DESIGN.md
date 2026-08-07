# Design constraints

- The ABI is dependency-free and transports only bounded canonical HTA state and transaction evidence.
- It defines no database schema, cryptography, key access, application payload semantics or executable catalogue.
- Concrete providers must recompute state digests and atomically compare revisions, install state and record receipts.
- Host installation and capability authority remain outside the ABI.
