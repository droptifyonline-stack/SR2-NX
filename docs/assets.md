# Private asset workflow

Keep original installations, IL2CPP dumps and exports outside the checkout. Never add proprietary data to Git, including metadata reports without reviewing their contents. The repository includes only synthetic examples and procedural geometry.

## Inspect the user's installation locally

In Windows PowerShell, from this repository:

```powershell
py -3 tools/asset-inspector/inspect.py "C:\Users\jesse\Desktop\SLime Rancher 2" --output "$env:TEMP\sr2-inventory.json"
py -3 tools/asset-inspector/inspect.py "C:\Users\jesse\Desktop\Il2CppDumper" --output "$env:TEMP\sr2-dumps-inventory.json"
py -3 tools/asset-inspector/inspect.py "C:\Users\jesse\Desktop\SR1-Plortable" --output "$env:TEMP\sr1-inventory.json"
```

In MSYS2 use `python tools/asset-inspector/inspect.py '/c/Users/jesse/Desktop/SLime Rancher 2' --output /tmp/sr2-inventory.json`. Python must be available in that shell. The tool records selected names/sizes, extension counts and IL2CPP metadata version when its header is recognized. Optional `--hash` calculates SHA-256 without copying content. It does not discover Unity version from arbitrary binary strings or claim recovered source.

Optional bundle inventory in a separate virtual environment:

```sh
python -m venv /path/to/private/venv
# Activate this venv using your shell's activation command.
python -m pip install -r tools/asset-inspector/requirements.txt
python tools/asset-inspector/unity_inventory.py /path/to/bundle --output /path/to/private/bundle-summary.json
```

Start with one bundle and inspect version/type counts. Use AssetRipper exports or UnityPy's documented export interfaces to extract one mesh and texture privately. Confirm dependencies, skeleton, animation bindings and materials before converting through Blender to GLB. Prefer self-contained GLB so referenced textures and buffers travel with the model. Recreate shaders as simplified PBR materials. Coordinate conversion must be deliberate: Bevy is right-handed Y-up, Unity is left-handed Y-up; verify winding, normals, handedness, meters and animation tracks with one known pose. No automatic SR2 conversion is verified.

## Load converted content

Create a private root with `models/pink_slime.glb` and adapt `assets/manifest.example.json`. Run:

```sh
cargo run -p asset-converter -- /private/manifest.json /private/converted
cargo run -p asset-converter -- texture /private/image.tga /private/converted/image.png
```

The validator checks version, identifiers, safe relative paths, existence and supported extensions. It does not certify that glTF dependencies, textures or animation bindings are correct. The image command decodes PNG/JPEG/TGA/BMP and writes PNG without overwriting existing output. It does not read Unity containers.

Set `SR2_ASSET_DIR` to that root. Copy/adapt `assets/scene.example.json` privately and set `SR2_SCENE_FILE` to its absolute filename. `cargo run -p sr2-desktop` loads scene 0 from each GLB/glTF and applies translation, Y rotation in radians and uniform scale. Embedded textures/materials load via Bevy's glTF loader. Animation clips can be imported by that loader; playback graphs and retargeting are not implemented. JSON placements do not yet generate collision or gameplay entities. Standalone texture records are validated but not assigned to imported materials automatically.

Windows PowerShell: `$env:SR2_ASSET_DIR='C:\private\converted'` and `$env:SR2_SCENE_FILE='C:\private\scene.json'` before running Cargo.
