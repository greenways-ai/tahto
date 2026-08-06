"""Operator CLI for the Tahto object vault."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from .vault import Vault


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(prog="tahto-vault")
    root.add_argument("--root", type=Path, required=True)
    commands = root.add_subparsers(dest="command", required=True)

    commands.add_parser("init")

    quota = commands.add_parser("quota-set")
    quota.add_argument("application")
    quota.add_argument("namespace")
    quota.add_argument("max_bytes", type=int)

    put = commands.add_parser("put")
    put.add_argument("application")
    put.add_argument("namespace")
    put.add_argument("path", type=Path)
    put.add_argument("--media-type")
    put.add_argument("--staging", action="store_true")

    missing = commands.add_parser("missing")
    missing.add_argument("digests", nargs="+")

    stat = commands.add_parser("stat")
    stat.add_argument("digest")

    verify_object = commands.add_parser("verify-object")
    verify_object.add_argument("digest")

    read = commands.add_parser("read")
    read.add_argument("digest")
    read.add_argument("--start", type=int, default=0)
    read.add_argument("--end", type=int)

    verify = commands.add_parser("verify-closure")
    verify.add_argument("roots", nargs="+")

    gc = commands.add_parser("gc")
    gc.add_argument("--apply", action="store_true")
    return root


def main(argv: list[str] | None = None) -> int:
    args = parser().parse_args(argv)
    with Vault(args.root) as vault:
        if args.command == "init":
            print(json.dumps({"root": str(vault.root), "status": "ready"}))
        elif args.command == "quota-set":
            vault.set_quota(args.application, args.namespace, args.max_bytes)
            print(json.dumps({"maxBytes": args.max_bytes}))
        elif args.command == "put":
            info = vault.put_file(
                args.application,
                args.namespace,
                args.path,
                args.media_type,
                role="staging" if args.staging else "root",
            )
            print(json.dumps(vars(info), sort_keys=True))
        elif args.command == "missing":
            print(json.dumps({"missing": vault.missing(args.digests)}))
        elif args.command == "stat":
            print(json.dumps(vars(vault.object_info(args.digest)), sort_keys=True))
        elif args.command == "verify-object":
            print(json.dumps(vars(vault.verify_object(args.digest)), sort_keys=True))
        elif args.command == "read":
            for chunk in vault.iter_range(
                args.digest, start=args.start, end_exclusive=args.end
            ):
                sys.stdout.buffer.write(chunk)
        elif args.command == "verify-closure":
            print(json.dumps({"objects": vault.verify_closure(args.roots)}))
        elif args.command == "gc":
            print(json.dumps({"garbage": vault.collect_garbage(dry_run=not args.apply)}))
        else:  # pragma: no cover - argparse makes this unreachable
            raise AssertionError(args.command)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
