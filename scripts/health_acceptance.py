#!/usr/bin/env python3
"""Prove Tahto health initialization and restart recovery against real providers."""

from __future__ import annotations

import argparse
import base64
import datetime
import json
import os
import pathlib
import signal
import subprocess
import tempfile
import time
import urllib.request


ROOT = pathlib.Path(__file__).resolve().parents[1]
BASE_URL = "http://127.0.0.1:58100"


def request(path: str, *, pairing: dict[str, object] | None = None) -> tuple[int, dict[str, object]]:
    headers: dict[str, str] = {}
    data = None
    method = "GET"
    if pairing is not None:
        encoded = base64.urlsafe_b64encode(
            json.dumps(pairing, separators=(",", ":")).encode()
        ).decode().rstrip("=")
        headers["x-tahto-pairing"] = encoded
        data = b""
        method = "POST"
    call = urllib.request.Request(BASE_URL + path, data=data, headers=headers, method=method)
    with urllib.request.urlopen(call, timeout=5) as response:
        return response.status, json.loads(response.read())


def await_health(expected: str, timeout: float = 10) -> dict[str, object]:
    deadline = time.monotonic() + timeout
    last_error: Exception | None = None
    while time.monotonic() < deadline:
        try:
            status, body = request("/tahto/v1/health")
            if status == 200 and body.get("status") == expected:
                return body
        except Exception as error:  # The worker may still be starting.
            last_error = error
        time.sleep(0.1)
    raise RuntimeError(f"health did not become {expected}: {last_error}")


def start(nginx: pathlib.Path, storage: pathlib.Path) -> subprocess.Popen[bytes]:
    env = os.environ.copy()
    env["HOPLITE_STORE_PATH"] = str(storage / "tahto.sqlite")
    env["HOPLITE_BLOB_ROOT"] = str(storage / "blob")
    return subprocess.Popen(
        [
            str(nginx),
            "-p",
            str(ROOT),
            "-c",
            ".hoplite/conf/nginx.conf",
            "-g",
            "daemon off;",
        ],
        cwd=ROOT,
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


def stop(process: subprocess.Popen[bytes]) -> None:
    if process.poll() is None:
        process.send_signal(signal.SIGINT)
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.terminate()
            process.wait(timeout=5)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--nginx",
        type=pathlib.Path,
        default=ROOT.parent / "hoplite/core/target/nginx/sbin/nginx",
    )
    arguments = parser.parse_args()
    nginx = arguments.nginx.resolve()
    if not nginx.is_file() or not os.access(nginx, os.X_OK):
        raise SystemExit(f"executable Hoplite Nginx not found: {nginx}")
    if not (ROOT / ".hoplite/conf/nginx.conf").is_file():
        raise SystemExit("build Tahto with `hoplite serve build --mode prod --profile server .`")

    with tempfile.TemporaryDirectory(prefix="tahto-health-") as temporary:
        storage = pathlib.Path(temporary)
        (storage / "blob").mkdir()
        process = start(nginx, storage)
        try:
            initial = await_health("not-ready")
            now = datetime.datetime.now(datetime.UTC).replace(microsecond=0)
            invitation = {
                "protocol": "tahto.pairing-invitation/1",
                "id": "invite.health-acceptance",
                "node": "node.health-acceptance",
                "tokenDigest": "sha256:" + "a" * 64,
                "approvalDigest": "sha256:" + "b" * 64,
                "createdAt": now.isoformat().replace("+00:00", "Z"),
                "expiresAt": (now + datetime.timedelta(hours=1)).isoformat().replace("+00:00", "Z"),
                "createdSeconds": int(now.timestamp()),
                "expiresSeconds": int(now.timestamp()) + 3600,
                "status": "open",
            }
            status, _ = request("/tahto/v1/pairing/invitations", pairing=invitation)
            if status != 201:
                raise RuntimeError(f"pairing initialization returned HTTP {status}")
            initialized = await_health("ready")
        finally:
            stop(process)

        process = start(nginx, storage)
        try:
            recovered = await_health("ready")
        finally:
            stop(process)

    print(json.dumps({"initial": initial, "initialized": initialized, "recovered": recovered}))


if __name__ == "__main__":
    main()
