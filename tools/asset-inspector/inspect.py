#!/usr/bin/env python3
"""Local metadata inventory; never copies game bytes. Python 3.11+, stdlib only."""
import argparse
import collections
import hashlib
import json
from pathlib import Path


def inspect(root, hashes=False):
    result = {"root": str(root), "files": [], "extensions": {}, "errors": []}
    counts = collections.Counter()
    wanted = {"GameAssembly.dll", "UnityPlayer.dll", "global-metadata.dat", "sr2-dump.cs", "sr2-script.json", "sr2-il2cpp.h", "sr2-stringliteral.json", "globalgamemanagers", "catalog.json"}
    for path in sorted(root.rglob("*")):
        try:
            if not path.is_file() or path.is_symlink():
                continue
            counts[path.suffix.lower() or "<none>"] += 1
            if path.name in wanted or path.suffix.lower() in {".bundle", ".assets"}:
                entry = {"path": str(path.relative_to(root)), "size": path.stat().st_size}
                if path.name == "global-metadata.dat":
                    with path.open("rb") as stream:
                        header = stream.read(8)
                    if len(header) == 8 and header[:4] == bytes.fromhex("af1bb1fa"):
                        entry["il2cpp_metadata_version"] = int.from_bytes(header[4:], "little")
                if hashes:
                    with path.open("rb") as stream:
                        entry["sha256"] = hashlib.file_digest(stream, "sha256").hexdigest()
                result["files"].append(entry)
        except OSError as exc:
            result["errors"].append({"path": str(path), "error": str(exc)})
    result["extensions"] = dict(counts)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--hash", action="store_true", help="Hash selected files; potentially slow")
    args = parser.parse_args()
    if not args.root.is_dir():
        parser.error("root must be an accessible local directory")
    report = inspect(args.root, args.hash)
    args.output.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(f"Wrote {len(report['files'])} selected file records; {len(report['errors'])} errors")


if __name__ == "__main__":
    main()
