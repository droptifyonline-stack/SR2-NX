# Progress

Current stage: first procedural desktop milestone; native Switch feasibility/integration remains experimental. Client date: 2026-10-18.

## Implemented

- Rust workspace, stable 1.95.0, pinned Bevy 0.18.1, frozen dependency lockfile, Windows/Linux CI definition.
- First-person 3D camera, WASD/mouse, jump/gravity, floor and arena collision, simple solid cube obstacle.
- Procedural ground, light, pink placeholder slime and hopping.
- Vacpack cone/range attraction, capture, bounded one-species inventory, release, inventory title display.
- Private asset root, manifest/scene validation, glTF scene placements, embedded texture/material and animation import features. Animation playback/retargeting is not implemented.
- PNG/JPEG/TGA/BMP-to-PNG conversion, local metadata inventory, optional UnityPy bundle inventory script.
- Experimental Horizon bootstrap, controller adapter, pinned bevy-horizon backend and isolated native build staging helper.
- Research of both requested projects, actual bevy-horizon/NVK path, optimization plan and hardware acceptance procedure. Original `AGENTS.MD` preserved. A case-only duplicate `AGENTS.md` was avoided for Windows checkout compatibility.

## Verified development capabilities

- `cargo fmt --all -- --check`, workspace check/build and six unit/system tests passed in the final saved-setup run. Tests exercise inventory/cone rules, traversal rejection, capture/release, full-inventory entity preservation, jumping/landing and obstacle collision.
- Linux desktop rendered via Xorg + Mesa Lavapipe; 120-frame smoke completed successfully. Screenshot inspected: ground, lit obstacle and pink slime visible (`desktop-prototype.png`). This is software rendering, not a performance benchmark.
- A synthetic glTF triangle loaded through scene JSON, spawned scene children and rendered in a second 120-frame smoke. No SR2 asset was used. A further animated fixture smoke retained and verified one imported animation clip after fixing full-model handle lifetime; playback is not implemented. The glTF loader emitted a hierarchy warning during scene construction, which remains to investigate with production assets.
- Windows x86_64 GNU `.exe` cross-compiled with the extracted MinGW toolchain. Windows execution is untested; CI has not run here.
- Horizon controller adapter type-checked on host using `cargo check -p sr2-gameplay --features horizon-input`; this does not validate Horizon linkage/runtime.
- Synthetic IL2CPP header inventory recognized metadata version 29. Synthetic TGA converted to PNG and 2×2 dimensions verified. Converted model manifest validator ran successfully.
- Saved environment `install_script` and `start_skill`; setup commands exercised on this machine. Added draft domains `pkg.devkitpro.org` and `api.github.com`. Draft persistence confirmed; runtime application, environment publication and restoration into a fresh task are not verified.

## Blockers and limits

- Native toolchain container build failed while synchronizing devkitPro packages: `pkg.devkitpro.org` could not resolve in the container; direct proxy request returned HTTP 403. No NVK build, native compilation or `.nro` was completed. Network settings require review/save/publication before retrying.
- No Switch is attached. V1 rendering, input, audio, filesystem, frame timing, memory use and 720p/30 FPS remain unverified. Upstream reports a lit Bevy 0.18 GPU hang; do not assume stable rendering from its demonstrations.
- Windows installation and dump paths are not mounted in this cloud session. No requested proprietary files were found under accessible workspace/temp locations. Real Unity version, bundles, extraction/conversion, shaders, skeletons and actual game scenes remain untested.
- Delivery: development branch push succeeded. [PR #1](https://github.com/droptifyonline-stack/SR2-NX/pull/1) was created successfully despite an earlier GitHub REST API read returning Forbidden. The initial implementation milestone is commit `27c3cf4`; later commits update this progress record.
- Prototype collision is hardcoded; imported geometry has no collision. Release places the slime ahead without ballistics. No saves, audio, feeding, ranch systems, streaming, final gameplay or measured console optimization.

## Next tasks

1. Apply saved network settings; rebuild the pinned native toolchain, NVK and upstream hello_switch, then attempt staged SR2-NX native compilation and fix actual diagnostics.
2. Run upstream and SR2-NX proof-of-concept on the original Switch V1, collect logs/crash reports and sustained frame-time/memory evidence. Resolve upstream lit-scene instability before expanding rendering.
3. Run the Windows executable; manually verify mouse lock/look, movement, suction and release. CI should execute on GitHub once the branch is available.
4. Run the local file inventory against the user's installation/dumps, inspect one real bundle privately, convert one slime to GLB, verify materials/scale/animation on desktop, then on Switch.
5. Add general collision, asset-driven slime archetypes and animation playback only after the platform/content gates produce evidence.
