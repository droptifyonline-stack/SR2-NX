# CODEX MASTER HANDOFF — SLIME RANCHER 2 NINTENDO SWITCH NATIVE PORT

## PROJECT IDENTITY

**Project name:** SR2-NX  
**Target game:** Slime Rancher 2 (PC/Steam)  
**Target hardware:** Original Nintendo Switch V1, unpatched, Atmosphère CFW  
**Development environment:** Windows, Alienware laptop  
**Primary language:** Rust  
**Primary engine:** Bevy  
**Target delivery:** Native Nintendo Switch homebrew executable (`.nro`), if supported by a working Switch backend  
**Performance goal:** 30 FPS at 720p docked or handheld where practical, with adaptive resolution and scalable graphical settings.

## 1. MISSION

Build a playable, native Nintendo Switch adaptation of Slime Rancher 2 using Rust and Bevy.

The user already possesses Slime Rancher 2 PC game files, extracted Slime Rancher 1 Nintendo Switch files, and extensive IL2CPP reverse-engineering outputs.

Do not restart the investigation from zero.

The intended approach is a new Rust/Bevy implementation that can load converted game assets and recreate the necessary gameplay systems.

Do not assume Unity IL2CPP code can be directly executed by Bevy.

### Non-negotiable goals

- Native Switch execution, not Android emulation, PC streaming, or cloud gaming.
- Reuse compatible game assets from the user's own game installation where technically feasible.
- Preserve recognizable Slime Rancher 2 gameplay, environments, slimes, and interactions.
- Support controller input, loading, saving, and offline gameplay.
- Target playable performance within the original Switch's hardware limits.
- Keep the Windows prototype runnable throughout development.
- Do not claim a working Switch port until it has actually been built and tested.

The user prefers minimal interruptions. Work autonomously through research, implementation, compilation, testing, and debugging. Ask questions only when blocked by missing files, required hardware testing, or decisions that materially change the project.

---

## 2. EXISTING FILES AND RESEARCH

### Slime Rancher 2 PC installation

Known location:

`C:\Users\jesse\Desktop\SLime Rancher 2`

Previously identified files:

| File | Previously reported size |
|---|---:|
| GameAssembly.dll | 86,650,368 bytes |
| UnityPlayer.dll | 34,107,824 bytes |
| global-metadata.dat | 22,284,980 bytes |
| globalgamemanagers | 3,323,236 bytes |

The installation also contains Unity Addressables assets, asset bundles, and catalog information.

Inspect the actual directory to verify the files, paths, Unity version, and contents.

### Existing Slime Rancher 2 IL2CPP research

Previous Il2CppDumper outputs were reportedly saved under:

`C:\Users\jesse\Desktop\Il2CppDumper`

Files include:

- `sr2-dump.cs`
- `sr2-script.json`
- `sr2-il2cpp.h`
- `sr2-stringliteral.json`

These files may provide:

- Class and method names
- Object and field layouts
- Method addresses and metadata
- Gameplay system identification
- References to player movement, slime behavior, inventory, and world systems

Important: `dump.cs` is a structural reference, not recovered original C# source code. Do not treat method signatures as complete gameplay implementations.

Inspect and index these outputs before attempting to recreate gameplay.

### Slime Rancher 1 Nintendo Switch reference

Existing extracted files:

`C:\Users\jesse\Desktop\SR1-Plortable`

Previously inspected:

`SR1-Plortable\exefs\main`

`SR1-Plortable\romfs\Data\Managed\Metadata\global-metadata.dat`

SR1 metadata was previously reported as 7,483,232 bytes.

Il2CppDumper was successfully used against the Switch executable and metadata.

The output included:

- `dump.cs`
- `script.json`
- Generated dummy DLLs

The earlier dump process identified IL2CPP metadata version 24.2 and used a 24.3 compatibility setting.

Use the SR1 Switch files for comparative research into Unity structures, content layout, and optimization choices.

Do not assume SR1 binaries, assets, or engine internals can directly execute SR2.

Do not overwrite existing dump files.

---

## 3. IMPORTANT PREVIOUS PROJECT — SKATE 3

The user previously spent substantial time attempting a native Nintendo Switch Skate 3 port using ReXGlue and Xbox 360 recompilation techniques.

Previous issues included:

- ARM64 linking failures
- `aarch64-none-elf-g++` failures
- ReXGlue SDK integration difficulties
- Atmosphère crashes
- A build that reached the title screen but crashed during autosave
- Regression after subsequent builds

The user recently discovered:

https://github.com/JohnUzoka/skate-3-rust-engine

This inspired the Rust/Bevy direction.

Investigate that project's architecture and its actual Nintendo Switch support.

Check the relevant branch, build instructions, dependencies, rendering backend, and demonstrated hardware functionality.

Do not assume the project is complete or that its Switch compatibility automatically transfers to Slime Rancher 2.

Extract reusable architectural lessons, but do not copy its gameplay-specific systems blindly.

The user wants a more maintainable approach with measurable milestones, automated testing, and fewer unstable builds.

---

## 4. ENGINE STRATEGY — RUST + BEVY

Investigate the current Bevy ecosystem and Nintendo Switch homebrew compatibility.

Potential areas of research:

- Bevy ECS
- Bevy rendering architecture
- wgpu and graphics backend requirements
- Horizon OS homebrew tooling
- devkitPro and libnx
- Rust AArch64 Switch targets
- Switch GPU rendering interfaces
- Experimental Bevy Switch forks
- Existing Rust Switch homebrew projects

Research this project as a starting point:

https://github.com/ibrahimcesar/switchbrew_bevy

Determine whether it provides a functional 3D rendering backend or only an experimental foundation.

### Critical feasibility requirement

Stock Bevy cannot be assumed to compile into a fully functional native Switch game.

Before significant game development, establish:

1. Whether an existing Switch-compatible Bevy renderer works.
2. Whether a minimal Bevy application can compile for the target.
3. Whether it can initialize graphics on real hardware.
4. Whether a simple 3D scene renders.
5. Whether controller input and filesystem access work.

If no functional Bevy renderer exists, investigate the smallest viable platform-specific rendering integration.

Keep gameplay systems separate from rendering so the architecture can survive a renderer change.

Do not spend weeks rebuilding gameplay before validating Switch graphics.

---

## 5. ASSET EXTRACTION AND CONVERSION

Inspect the user's existing Slime Rancher 2 installation.

Identify:

- Unity version
- IL2CPP metadata version
- Rendering pipeline
- Asset bundle layout
- Addressables catalog
- Mesh formats
- Texture formats
- Material and shader dependencies
- Animation data
- Scene references
- Audio assets
- Terrain data
- Prefab relationships

Evaluate tools such as AssetRipper, AssetStudio, UnityPy, and other relevant open-source utilities.

Do not assume these tools are installed.

Prefer automated extraction and conversion pipelines rather than manual processing of every asset.

### Target formats

Where practical, convert:

- Models → glTF/GLB
- Textures → PNG or GPU-compatible compressed formats
- Animations → glTF-compatible animation data
- Scene placement → Structured scene manifests
- Object properties → JSON, RON, or equivalent structured data

Preserve object relationships, coordinates, hierarchy, scale, materials, and animation bindings where possible.

Do not assume Unity shaders or HDRP materials can run directly in Bevy.

Implement simplified, performance-conscious materials.

### First asset milestone

Extract one slime model and its textures.

Load it into a Bevy 3D scene on Windows.

Validate scale, materials, animation, and rendering.

Then establish whether the same asset can render on Switch.

---

## 6. GAMEPLAY SYSTEMS

Create modular Rust systems rather than a monolithic recreation.

### Player

- First-person camera
- Walking and running
- Jumping
- Collision detection
- Controller support
- Camera sensitivity
- Interactions

### Vacpack

- Suction
- Object attraction
- Item capture
- Inventory slots
- Object launching
- Item categorization
- Capacity limits

### Slimes

- Slime entities
- Movement and hopping
- Physics interactions
- Feeding
- Hunger states
- Plort production
- Species-specific behavior
- Largo transformations
- Environmental interactions

### World

- Scene loading
- Static geometry
- Collision meshes
- Interactable objects
- Terrain
- Resource spawning
- Region streaming
- Object persistence

### Ranch systems

- Corrals
- Feeding
- Resource collection
- Gardens
- Storage
- Upgrades

### Persistence

- Save slots
- Player position
- Inventory
- World state
- Slime states
- Ranch progression

Use the IL2CPP metadata as a research reference for identifying systems, not as an automatically executable implementation.

---

## 7. SWITCH OPTIMIZATION

Target the original Nintendo Switch V1, not newer hardware.

Important constraints:

- Approximately 4 GB shared system memory
- ARM Cortex-A57 CPU
- NVIDIA Tegra X1 GPU
- Limited memory bandwidth
- Tight texture and shader budgets

Implement or plan for:

- Asset streaming
- Texture compression
- Reduced draw calls
- Mesh LOD
- Frustum culling
- Simplified lighting
- Reduced particle effects
- Physics optimization
- Entity update throttling
- Dynamic resolution where supported
- Configurable graphics quality

Prioritize memory stability and actual gameplay over graphical fidelity.

### Performance targets

Initial:

- Load one scene successfully.
- Render one slime.
- Maintain stable memory usage.
- Avoid crashes.

Intermediate:

- Basic player movement.
- Interactable slime.
- Functional Vacpack.
- Small environment.

Long-term:

- Stable gameplay.
- Larger environments.
- Save/load.
- Target 30 FPS where feasible.

Record real performance rather than estimating it.

---

## 8. DEVELOPMENT STRUCTURE

Proposed repository:

```text
SR2-NX/
├── Cargo.toml
├── README.md
├── docs/
│   ├── research.md
│   ├── feasibility.md
│   ├── progress.md
│   ├── switch-build.md
│   └── architecture.md
├── crates/
│   ├── sr2-core/
│   ├── sr2-gameplay/
│   ├── sr2-assets/
│   ├── sr2-physics/
│   ├── sr2-save/
│   └── sr2-platform/
├── apps/
│   ├── sr2-desktop/
│   └── sr2-switch/
├── tools/
│   ├── asset-inspector/
│   └── asset-converter/
├── assets/
│   └── README.md
└── tests/
```

Keep proprietary extracted assets outside the public source repository.

Use a local configuration file or environment variable to reference the original game installation.

Do not redistribute proprietary game files.

---

## 9. AUTONOMOUS WORKFLOW

Codex should operate as an active development agent, not merely produce a research report.

For every milestone:

1. Inspect existing source and files.
2. Research missing technical details.
3. Choose a defensible implementation.
4. Write the necessary code.
5. Run formatting and static checks.
6. Compile where the toolchain is available.
7. Execute relevant tests.
8. Fix errors.
9. Record results.
10. Continue to the next unblocked task.

When a build fails, inspect the actual compiler output and fix the underlying cause.

Do not repeatedly change dependencies without understanding compatibility.

Maintain working checkpoints.

Avoid claiming success from compilation alone when hardware behavior remains untested.

If hardware testing is required, produce an exact build and testing procedure and explain what logs are needed.

Do not request files already accessible in the workspace.

---

## 10. FIRST DEVELOPMENT SPRINT

Start immediately with the following work.

### Phase A — Inventory and feasibility

- Locate the SR2 installation.
- Inspect existing IL2CPP dumps.
- Identify Unity version and asset layout.
- Locate SR1 Switch reference files.
- Review the Skate 3 Rust engine project.
- Review existing Bevy Switch compatibility work.
- Document feasible and infeasible approaches.

### Phase B — Desktop prototype

- Initialize Rust workspace.
- Select a compatible Bevy version.
- Build a simple 3D scene.
- Add camera and controller architecture.
- Build a basic asset inspection utility.
- Convert and display one SR2 model.

### Phase C — Switch proof of concept

- Validate toolchain.
- Attempt a minimal native executable.
- Establish graphics initialization.
- Render a 3D scene.
- Test input and basic filesystem access.
- Record memory and performance data.

### Phase D — First gameplay prototype

Only after the foundational platform risks are understood:

- Implement player movement.
- Implement one slime entity.
- Implement basic suction.
- Implement object capture and release.
- Load a small converted environment.

---

## 11. PROGRESS AND REPORTING

Maintain `docs/progress.md` with:

- Completed tasks
- Current milestone
- Known blockers
- Build status
- Test results
- Next actions

Distinguish clearly between:

- Researched
- Implemented
- Compiles
- Runs on Windows
- Runs on Switch
- Hardware verified

Never mark something as Switch-compatible based only on Windows results.

Do not delete, replace, or modify existing game files or previous research without an explicit reason.

---

## 12. START NOW

Begin by inspecting the existing files and development environment.

Use the previously established paths rather than asking the user to upload everything again.

Then:

1. Summarize what is actually available.
2. Identify the strongest Bevy-to-Switch implementation path.
3. Create the SR2-NX Rust workspace.
4. Implement the first desktop 3D prototype.
5. Start an automated asset inspection/conversion workflow.
6. Run available builds and tests.
7. Document any blockers.
8. Continue independently until hardware access, missing dependencies, or a major architectural decision requires user input.

**Do not stop at planning. Write code, compile it, test it, fix errors, and make measurable progress.**

The ultimate goal is a real, playable Slime Rancher 2 experience running natively on the original Nintendo Switch.


## CURRENT USER DIRECTION — 2026-10-19

The user clarified: "We dont have to use bevy or rust it was just an suggestion". Engine/language statements above describe the earlier proposal. Rust and Bevy are optional. The current hardware proof is `native/sr2-probe`, using C++17/libnx/OpenGL; the full game's engine remains undecided. Preserve the Rust desktop reference and historical research. Prioritize testing the native NRO on the original Switch before expanding gameplay or choosing a full engine. See `docs/native-probe.md`.
