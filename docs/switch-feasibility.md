# Native Switch feasibility and procedure

**Current direction (2026-10-19):** Rust/Bevy are optional per the user. A C++/libnx/OpenGL probe now compiles into a native NRO; use [the current build/hardware procedure](native-probe.md). The Bevy/NVK notes below preserve earlier research and an optional route. NVK preparation was stopped intentionally after the scope change; it is not required by the C++ probe.

## Evidence and decision

A stock desktop Bevy build cannot simply target Horizon. `switchbrew_bevy` does not implement native rendering. The stronger candidate is [bevy-horizon](https://github.com/JohnUzoka/bevy-horizon/tree/3eac779b232ee47cd0eb957676c64386ac7b0f1d) with [mesa-switch](https://github.com/danfromtico/mesa-switch/tree/606441c90e459028a50fca352daebfc20211f890). Its upstream documentation reports Switch Lite hardware rendering and a Bevy 0.18 game, with a known lit-scene GPU hang. SR2-NX chooses Bevy 0.18.1 so that patch family is usable. V1 behavior, docked mode and sustained stability are open.

## Runtime responsibilities

- devkitA64, newlib and libnx supply native startup, system calls and SDK linkage.
- bevy-horizon replaces winit/gilrs with its window/frame loop, libnx controller input, lifecycle handling and logging.
- Patched raw-window-handle/wgpu and statically linked Mesa NVK provide the Vulkan path to Tegra GM20B. A plain wgpu dependency lacks this backend.
- Audio requires its patched cpal/libnx renderer; audio is intentionally disabled in this initial prototype and untested here.
- romfs and SD filesystem access are supplied by the runtime; asset roots and permissions need console testing.
- ELF output is converted with devkitPro's NRO tools by upstream scripts. A host binary is not an NRO.

`sr2-switch` has the target-specific bootstrap and controls: left stick movement, right stick look, South button jump, left trigger suction, right trigger release and Start exit. These have not been checked on a console. The display requests 1280×720; the backend may change it when docked. Frame timing and memory reporting still require integration and measurement.

## Build prerequisites and commands

The upstream workflow needs Linux/WSL2, Docker/Podman, about 20 GB free disk and sufficient RAM; cap jobs at four on this environment. Keep third-party checkouts outside this game repository. Pin the inspected revisions:

```sh
git clone https://github.com/JohnUzoka/bevy-horizon /path/to/bevy-horizon
git -C /path/to/bevy-horizon checkout 3eac779b232ee47cd0eb957676c64386ac7b0f1d
git clone https://github.com/danfromtico/mesa-switch /path/to/mesa-switch
git -C /path/to/mesa-switch checkout 606441c90e459028a50fca352daebfc20211f890
cd /path/to/bevy-horizon
docker build -t bevy-horizon-toolchain:local -f Containerfile .
JOBS=4 scripts/build_nvk.sh /path/to/mesa-switch
scripts/localize_nvk.sh
scripts/build_example.sh hello_switch
```

Test upstream hello_switch before the game. Build SR2-NX using the staging helper:

```sh
cd /path/to/SR2-NX
BEVY_HORIZON_DIR=/path/to/bevy-horizon SR2_SWITCH_STAGE=/path/to/new-sr2-stage JOBS=4 tools/build-switch.sh
```

The helper copies source into a new external staging directory, excluding private assets, Git, target and the desktop toolchain pin. Upstream rewrites the staged lockfile and uses its own nightly/build-std. Keep the tested desktop lockfile intact. Output, if the build succeeds: `<stage>/target/aarch64-nintendo-horizon/release/sr2-switch.nro`. The helper's complete native path remains experimental until it executes successfully; see progress for actual results.

## Hardware gate

Copy hello_switch into its own `sdmc:/switch/hello_switch/` folder. Launch in title override/full-memory mode (hold R while starting an installed game), not Album applet mode. Verify rotating geometry, controller buttons, HOME/suspend/resume and clean exit. Collect its `log.txt`, panic/crash output if any, and `[Exit] PASS`. Then test SR2-NX in a separate folder with placeholders only. Record V1 model, Atmosphère version, handheld/docked mode, clocks, resolution, renderer identity, startup time, 10-minute frame-time distribution, process/GPU memory and filesystem read/write outcome. Verify movement, jumping, capture/release and lifecycle. A screenshot or successful compile alone cannot pass this gate.

Do not claim 720p/30 FPS until measured. Investigate the upstream lit-scene GPU hang before trusting PBR stability. No Switch hardware is attached to this cloud environment.
