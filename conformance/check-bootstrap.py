#!/usr/bin/env python3
"""Static conformance checks for the TAHTO-1 repository bootstrap."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED = (
    "src/tahto/node/app.hal",
    "src/tahto/protocol/descriptor.hal",
    "src/tahto/store/README.md",
    "src/tahto/sync/README.md",
    "src/tahto/backup/README.md",
    "src/tahto/service/README.md",
    "protocol/tahto.md",
    "conformance/routes.edn",
    "adapters/greenways-space/README.md",
    "bin/tahto",
    "bin/greenways-beacon",
)

CANONICAL_ROUTES = (
    "/.well-known/tahto",
    "/tahto/0-alpha/health",
    "/tahto/0-alpha/status",
)

COMPATIBILITY_ROUTES = (
    "/.well-known/greenways-beacon",
    "/beacon/v1/health",
    "/beacon/v1/status",
)


def main() -> None:
    missing = [path for path in REQUIRED if not (ROOT / path).is_file()]
    if missing:
        raise SystemExit("missing bootstrap files: " + ", ".join(missing))

    app = (ROOT / "src/tahto/node/app.hal").read_text(encoding="utf-8")
    descriptor = (ROOT / "src/tahto/protocol/descriptor.hal").read_text(
        encoding="utf-8"
    )
    source = app + "\n" + descriptor

    for route in CANONICAL_ROUTES + COMPATIBILITY_ROUTES:
        if route not in app:
            raise SystemExit(f"missing route: {route}")

    forbidden = {
        "Hoplite proxy declaration": ":proxies",
        "legacy Space proxy path": '"/space/"',
        "hard-coded hosted authority": "https://greenways.space",
    }
    for label, token in forbidden.items():
        if token in source:
            raise SystemExit(f"{label} remains in Tahto core: {token}")

    if '"remoteExecutableCatalogue" false' not in descriptor:
        raise SystemExit("descriptor must retain the no-remote-code boundary")
    if '"hostedSpaceRequired" false' not in descriptor:
        raise SystemExit("descriptor must keep hosted Space optional")
    if "preserve-divergent-heads" not in descriptor:
        raise SystemExit("descriptor must preserve application conflicts")

    print("TAHTO-1 bootstrap conformance passed")


if __name__ == "__main__":
    main()
