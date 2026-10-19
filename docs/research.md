# Research evidence

Inspected repository snapshots on 2026-10-18 (client date). Upstream README claims are attributed below; they are not SR2-NX hardware measurements.

## Existing research

The initial checkout contained only `AGENTS.MD` at `10f710a51eddbc93041e98f3a4cd0da22a20597c`. Its previous file sizes and Unity/IL2CPP observations remain unverified historical notes. A filename search under the accessible `/workspace` and `/tmp` found no requested game binaries, metadata or dumps. Windows paths are not mounted in this cloud machine. Never infer method bodies from IL2CPP signatures.

## switchbrew_bevy

Source: https://github.com/ibrahimcesar/switchbrew_bevy at `a4350d63e3a453125f130e35c61fc28e4c93fe95`.

The manifest uses Bevy 0.17, edition 2024, and a nightly toolchain. Its Switch `nx` dependency is commented out; its feature flags do not supply a renderer. README marks Switch graphics, audio and filesystem work unfinished and emulator testing unchecked. Input/window abstractions are useful references, not proof of native 3D rendering. `no_std` ECS support does not establish render, asset or audio support.

## Skate 3 Rust Engine

Source: https://github.com/JohnUzoka/skate-3-rust-engine at `7f331624e34f61cb62d24852aac55ebf6b9534ef`.

The workspace pins Bevy 0.18.1 and patches several rendering crates. `crates/skate-game/Cargo.toml` has Horizon-specific features and a sibling bevy-horizon dependency. `src/horizon.rs` contains startup logging, input/backend hooks, upload pacing and memory tracking. `docs/switch/README.md` describes a shipped NRO workflow, University map testing and limitations (docked mode and other maps untested). These are considerably stronger evidence than a generic portability README, but no upstream binary was run here.

## bevy-horizon

Source: https://github.com/JohnUzoka/bevy-horizon at `3eac779b232ee47cd0eb957676c64386ac7b0f1d`.

Inspected README, crate manifest, Containerfile, build_game.sh and platform/porting documentation. Supports Bevy 0.18.1 and 0.19.1 patch families; this project selects 0.18.1 to match the existing game port. Desktop uses stable 1.95.0. Native container pins nightly-2026-10-04 and builds std for its custom Horizon target, not the stock desktop target.

The README reports hardware testing on ONE Switch Lite: hello_switch 60 FPS, render_check 24 cubes drawn, capability test 179 passes/2 warnings, Skate approximately 30 FPS. It also reports an unresolved GPU hang about ten seconds into a lit Bevy 0.18 scene. Original V1 and docked behavior remain unproven for SR2-NX. Stock emulators are documented as unable to run these Vulkan builds.

## Assets

- https://github.com/AssetRipper/AssetRipper: exports Unity serialized data into a project; README advertises Unity 3.5–6000.4 support with varying quality. It does not establish SR2 compatibility or Bevy-ready materials.
- https://github.com/K0lb3/UnityPy: Python inspection/export library for serialized Unity objects, textures, meshes and more. Its documentation warns APIs change between minor versions. Optional tooling pins 1.25.4, identified from PyPI metadata. No real SR2 bundle has been parsed here.
- glTF/GLB is the prototype interchange format supported by Bevy. Embedded materials/skins/animations are loader inputs, not an automatic translation of Unity HDRP shaders, animator controllers, Addressables or prefab behavior. Blender can serve as a manual mesh/animation conversion stage; no automated Blender pipeline is claimed.
