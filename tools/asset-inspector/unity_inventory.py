#!/usr/bin/env python3
"""Optional UnityPy bundle inspection, no game bytes or typetrees exported."""
import argparse
import collections
import json
from pathlib import Path
import UnityPy

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument("bundle", type=Path)
parser.add_argument("--output", type=Path, required=True)
args=parser.parse_args()
env=UnityPy.load(str(args.bundle))
counts=collections.Counter(obj.type.name for obj in env.objects)
versions=sorted({str(obj.assets_file.unity_version) for obj in env.objects})
args.output.write_text(json.dumps({"unitypy":UnityPy.__version__,"unity_versions":versions,"object_types":dict(counts)},indent=2),encoding="utf-8")
print("Wrote object-type inventory; no extraction or automatic conversion performed")
