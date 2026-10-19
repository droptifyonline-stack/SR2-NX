# Architecture

- `sr2-core`: inventory rules and suction eligibility without Bevy or platform APIs.
- `sr2-platform`: normalized player intent and frame budget. No claim of a full platform runtime.
- `sr2-gameplay`: Bevy entities, procedural visuals, player motion/collision, hopping, capture/release. Input adapter is selected by target. Rendering uses Bevy components; pure core rules can survive a backend change.
- `sr2-assets`: private root configuration, versioned manifest and scene transform validation.
- `sr2-desktop`: winit desktop window and optional glTF scene loading.
- `sr2-switch`: Horizon bootstrap and backend integration, separated from desktop window dependencies.
- `asset-converter`: decoded-image conversion and manifest validation, not a Unity converter.

The first prototype uses a kinematic player, ground height, bounded arena and one hardcoded obstacle AABB. Imported scene geometry currently has no generated collision. Slimes hop on the ground; suction overrides position but does not implement a rigid-body simulation. Inventory is one species with capacity 20. General collisions, animation playback, save/load, feeding, corrals and content streaming remain future work.

## Performance plan

Use the 33.3 ms budget as a measurement target, not a performance promise. Start with one directional light without shadows, low-poly placeholders and simple materials. Bevy supplies frustum culling. Profile CPU/GPU frame times separately on V1 hardware, track process and GPU allocations, then cap draw calls and texture budgets based on measurements. Investigate 512/1024 texture tiers and backend-supported BC formats before assuming ASTC support. Add distance-based AI scheduling, pooled effects, region streaming, mesh LOD and bounded physics only as entity/content counts justify them. The console's 4 GB is shared with the OS; homebrew title mode has less usable memory and GPU allocations share its heap.

## Current native hardware probe

`native/sr2-probe` is an independent C++17/libnx/OpenGL experiment. It uses procedural meshes and a small kinematic gameplay loop, plus SD logging for graphics/frame/memory diagnostics. It does not share Rust gameplay code or load Unity assets. The existing Rust implementation is a desktop reference; the full-game engine and authoritative gameplay implementation should be chosen after hardware evidence. This probe is intentionally small rather than a complete engine.
