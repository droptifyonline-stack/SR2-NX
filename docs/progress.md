# Progress

Current stage: native hardware-test milestone compiled; awaiting original Switch V1 execution. Updated 2026-10-19. Earlier desktop evidence dates to 2026-10-18.

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

- The earlier devkitPro access blocker is resolved in this running session. SDK packages install with verification enabled after supplying the session CA to devkitPro’s separate trust bundle. The native Rust toolchain and upstream capability test compiled and packaged successfully. NVK preparation was stopped after the user made Rust/Bevy optional; no NVK-backed cube or Bevy game NRO was built.
- No Switch is attached. V1 rendering, input, audio, filesystem, frame timing, memory use and 720p/30 FPS remain unverified. Upstream reports a lit Bevy 0.18 GPU hang; do not assume stable rendering from its demonstrations.
- Windows installation and dump paths are not mounted in this cloud session. No requested proprietary files were found under accessible workspace/temp locations. Real Unity version, bundles, extraction/conversion, shaders, skeletons and actual game scenes remain untested.
- Delivery: development branch push succeeded. [PR #1](https://github.com/droptifyonline-stack/SR2-NX/pull/1) was created successfully despite an earlier GitHub REST API read returning Forbidden. The initial implementation milestone is commit `27c3cf4`; later commits update this progress record.
- Prototype collision is hardcoded; imported geometry has no collision. Release places the slime ahead without ballistics. No saves, audio, feeding, ranch systems, streaming, final gameplay or measured console optimization.

## Next tasks

1. Run the C++ native probe on the original Switch V1 in title-override mode; collect the scene/control observations and its log.
2. Diagnose actual graphics/lifecycle/performance results before choosing the full game's engine. The Bevy/NVK route is optional research.
3. Inspect the user's installation/dumps locally, convert one slime privately and validate its materials, scale and animation in the chosen renderer.
4. Preserve the working Windows/Linux Rust reference. General collision, asset-driven entities and persistence follow the hardware/content gates.

## Native C++ milestone — 2026-10-19

- User removed the Rust/Bevy requirement. Full-game engine choice remains open; existing reference code/research is preserved.
- Built a C++17/libnx/EGL/OpenGL prototype with procedural ground, obstacle and hopping sphere; controller movement/look, jump/collision, suction/capture/release, reset, inventory markers and diagnostics.
- `tools/build-native-probe.sh` produces `target/switch-artifacts/sr2-native-probe.nro` using a pinned devkitPro base image. Actual AArch64 compilation/linking and NRO packaging succeeded. Repeated build, NACP title, NRO/ASET bounds and SHA-256 checks passed. Malformed NRO input was rejected. GLSL compilation/link validation passed. C++ formatting checked; no compiler warnings in the final build.
- **Not run on Switch.** No real hardware rendering, input, frame-rate, memory or lifecycle result is claimed. Native C++ gameplay has compile/static validation only; the six earlier Rust tests do not test this new implementation.
- The upstream Bevy capability-test NRO also built, with RomFS/ASET validation. It is an optional diagnostic, not the game. Its notices and build metadata accompany the artifact.
- Linux/Windows desktop executables remain intact. Generated desktop build intermediates were cleared for native-build disk space; later builds will recompile them.
- Next action: run the C++ NRO on V1 in title-override mode, verify visible scene/controls and collect `sdmc:/switch/sr2-native-probe/log.txt`. Then choose the engine based on measured platform behavior and inspect one real SR2 asset privately. See `docs/native-probe.md`.
