#!/usr/bin/env python3
"""Validate TAHTO-2 schemas and application-neutral conformance fixtures."""

from __future__ import annotations

import json
import re
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "protocol" / "schema" / "tahto-core-1.schema.json"
VALID_PATH = ROOT / "conformance" / "fixtures" / "protocol" / "valid-core-records.json"
INVALID_PATH = ROOT / "conformance" / "fixtures" / "protocol" / "invalid-core-records.json"

EXPECTED_PROTOCOLS = {
    "tahto.node/0-alpha",
    "tahto.device/0-alpha",
    "tahto.application/0-alpha",
    "tahto.namespace/0-alpha",
    "tahto.collection/0-alpha",
    "tahto.object/0-alpha",
    "tahto.commit/0-alpha",
    "tahto.head/0-alpha",
    "tahto.backup/0-alpha",
    "tahto.receipt/0-alpha",
    "tahto.service/0-alpha",
    "tahto.job/0-alpha",
}

EXPECTED_MODES = {
    "snapshot/1",
    "event-log/1",
    "object-graph/1",
    "git-dag/1",
    "derived/1",
}

COMMIT_FIELDS = {
    "protocol",
    "root",
    "application",
    "namespace",
    "collection",
    "schema",
    "schemaVersion",
    "device",
    "parents",
    "objects",
    "tombstones",
    "sequence",
    "timestamp",
    "signature",
}

FORBIDDEN_DOMAIN_FIELDS = {
    "conversation",
    "conversationId",
    "historiaConversationId",
    "room",
    "mandate",
    "scene",
    "world",
    "ledgerTransaction",
    "spaceMembership",
}


class ValidationError(ValueError):
    pass


def load(path: Path):
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def resolve(schema_root: dict, fragment: dict) -> dict:
    while "$ref" in fragment:
        reference = fragment["$ref"]
        prefix = "#/$defs/"
        if not reference.startswith(prefix):
            raise ValidationError(f"unsupported reference: {reference}")
        fragment = schema_root["$defs"][reference[len(prefix) :]]
    return fragment


def fail(path: str, message: str) -> None:
    raise ValidationError(f"{path}: {message}")


def validate(schema_root: dict, fragment: dict, value, path: str = "$") -> None:
    fragment = resolve(schema_root, fragment)

    for clause in fragment.get("allOf", []):
        condition = clause.get("if")
        if condition is None:
            validate(schema_root, clause, value, path)
            continue
        try:
            validate(schema_root, condition, value, path)
        except ValidationError:
            continue
        consequence = clause.get("then")
        if consequence is not None:
            try:
                validate(schema_root, consequence, value, path)
            except ValidationError as error:
                if value.get("protocol") == "tahto.collection/0-alpha":
                    fail(path, f"derived collection constraint: {error}")
                raise

    if "const" in fragment and value != fragment["const"]:
        fail(path, f"const mismatch: expected {fragment['const']!r}")

    if "enum" in fragment and value not in fragment["enum"]:
        fail(path, f"enum mismatch: {value!r}")

    expected_type = fragment.get("type")
    if expected_type == "object":
        if not isinstance(value, dict):
            fail(path, "expected object")
        required = fragment.get("required", [])
        for key in required:
            if key not in value:
                fail(path, f"required property missing: {key}")
        properties = fragment.get("properties", {})
        if fragment.get("additionalProperties") is False:
            for key in value:
                if key not in properties:
                    fail(path, f"additional property: {key}")
        for key, item in value.items():
            if key in properties:
                validate(schema_root, properties[key], item, f"{path}.{key}")
        return

    if expected_type == "array":
        if not isinstance(value, list):
            fail(path, "expected array")
        minimum = fragment.get("minItems")
        if minimum is not None and len(value) < minimum:
            fail(path, f"minItems violation: {minimum}")
        if fragment.get("uniqueItems"):
            canonical = [
                json.dumps(item, sort_keys=True, separators=(",", ":"))
                for item in value
            ]
            if len(canonical) != len(set(canonical)):
                fail(path, "uniqueItems violation")
        item_schema = fragment.get("items")
        if item_schema:
            for index, item in enumerate(value):
                validate(schema_root, item_schema, item, f"{path}[{index}]")
        return

    if expected_type == "string":
        if not isinstance(value, str):
            fail(path, "expected string")
        if len(value) < fragment.get("minLength", 0):
            fail(path, f"minLength violation: {fragment['minLength']}")
        pattern = fragment.get("pattern")
        if pattern and re.fullmatch(pattern, value) is None:
            fail(path, f"pattern mismatch: {pattern}")
        format_name = fragment.get("format")
        if format_name == "date-time":
            if re.fullmatch(
                r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z", value
            ) is None:
                fail(path, "date-time format mismatch")
        elif format_name == "uri":
            parsed = urlparse(value)
            if not parsed.scheme or not parsed.netloc:
                fail(path, "URI format mismatch")
        return

    if expected_type == "integer":
        if isinstance(value, bool) or not isinstance(value, int):
            fail(path, "expected integer")
        minimum = fragment.get("minimum")
        if minimum is not None and value < minimum:
            fail(path, f"minimum violation: {minimum}")
        return

    if expected_type is not None:
        fail(path, f"unsupported schema type: {expected_type}")


def record_definitions(schema: dict) -> dict[str, dict]:
    records = {}
    for definition in schema["$defs"].values():
        protocol = definition.get("properties", {}).get("protocol", {}).get("const")
        if protocol:
            records[protocol] = definition
    return records


def check_schema_contract(schema: dict) -> dict[str, dict]:
    records = record_definitions(schema)
    if set(records) != EXPECTED_PROTOCOLS:
        missing = EXPECTED_PROTOCOLS - set(records)
        extra = set(records) - EXPECTED_PROTOCOLS
        fail(
            "$schema",
            f"protocol registry mismatch; missing={sorted(missing)}, extra={sorted(extra)}",
        )

    modes = set(schema["$defs"]["collectionMode"]["enum"])
    if modes != EXPECTED_MODES:
        fail(
            "$schema.$defs.collectionMode",
            f"closed mode vocabulary changed: {sorted(modes)}",
        )

    for protocol, definition in records.items():
        if definition.get("additionalProperties") is not False:
            fail(f"$schema.{protocol}", "core records must reject additional properties")
        required = set(definition.get("required", []))
        if "protocol" not in required:
            fail(f"$schema.{protocol}", "protocol discriminator must be required")
        fields = set(definition.get("properties", {}))
        overlap = fields & FORBIDDEN_DOMAIN_FIELDS
        if overlap:
            fail(
                f"$schema.{protocol}",
                f"application-specific fields present: {sorted(overlap)}",
            )

    commit_required = set(records["tahto.commit/0-alpha"]["required"])
    if commit_required != COMMIT_FIELDS:
        fail(
            "$schema.tahto.commit/0-alpha",
            "commit contract changed; "
            f"expected={sorted(COMMIT_FIELDS)}, actual={sorted(commit_required)}",
        )

    return records


def check_valid_fixtures(schema: dict, records: dict[str, dict]) -> None:
    fixture = load(VALID_PATH)
    seen = set()
    divergent_head = False

    for index, record in enumerate(fixture["records"]):
        protocol = record.get("protocol")
        if protocol not in records:
            fail(f"$valid[{index}]", f"unknown protocol: {protocol!r}")
        if protocol in seen:
            fail(f"$valid[{index}]", f"duplicate fixture protocol: {protocol}")
        validate(schema, records[protocol], record, f"$valid[{index}]")
        seen.add(protocol)
        if protocol == "tahto.head/0-alpha" and len(record["commits"]) > 1:
            divergent_head = True

    if seen != EXPECTED_PROTOCOLS:
        fail("$valid", "one valid fixture is required for every core record")
    if not divergent_head:
        fail("$valid", "fixtures must prove divergent heads are representable")


def check_invalid_fixtures(schema: dict, records: dict[str, dict]) -> None:
    fixture = load(INVALID_PATH)
    for index, case in enumerate(fixture["cases"]):
        record = case["record"]
        protocol = record.get("protocol")
        definition = records.get(protocol)
        if definition is None:
            fail(f"$invalid[{index}]", f"fixture uses unknown protocol: {protocol!r}")
        try:
            validate(schema, definition, record, f"$invalid[{index}]")
        except ValidationError as error:
            expected = case["errorContains"].lower()
            if expected not in str(error).lower():
                fail(
                    f"$invalid[{index}]",
                    f"wrong validation failure; expected {expected!r}, got {str(error)!r}",
                )
        else:
            fail(f"$invalid[{index}]", "invalid fixture unexpectedly passed")


def main() -> None:
    schema = load(SCHEMA_PATH)
    records = check_schema_contract(schema)
    check_valid_fixtures(schema, records)
    check_invalid_fixtures(schema, records)
    print("TAHTO-2 protocol conformance passed")


if __name__ == "__main__":
    main()
