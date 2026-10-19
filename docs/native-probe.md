# Native Switch hardware probe

On 2026-10-19 the user clarified that Rust and Bevy were suggestions rather than requirements. The current hardware proof uses **C++17, libnx, EGL/OpenGL and devkitPro's Switch Mesa**, following the public-domain Switchbrew examples. The full game's engine remains undecided. The existing Rust/Bevy desktop prototype and previous research are preserved.

This small prototype compiles directly into an NRO. It contains procedural ground, an obstacle, a pink hopping sphere, first-person movement/look, jumping, simple collision, suction/capture and release. One slime exists in this probe. Inventory markers appear near the bottom of the view after capture; inventory is also written to the log. There are no proprietary assets, automatic Unity conversion, save files or audio.

## Build

Requires Linux/WSL2, Docker, Bash and Python 3. The toolchain container installs the public devkitPro homebrew SDK and OpenGL libraries. Rust, Bevy and NVK are unnecessary for this target.

```sh
bash tools/build-native-probe.sh
```

In the prepared managed cloud environment:

```sh
CONTAINER_ENGINE=/workspace/sr2-system/native/docker-proxy bash tools/build-native-probe.sh
```

The managed wrapper supplies session proxy/CA trust without disabling verification. Compilation occurs in `target/native-switch`, preserving tracked files. The helper copies the result to `target/switch-artifacts/sr2-native-probe.nro` and validates its NRO segments, ASET section, icon and NACP bounds. An adjacent SHA-256 file records the result. The Makefile and EGL setup are adapted from public-domain `switchbrew/switch-examples` at `669786898205b7beb25ff1731e72982e6d0397d3`.

A successful compiler/package check establishes an AArch64 homebrew executable. It does not establish that graphics initialize, controls work or performance meets a target on a console. GLSL vertex/fragment compilation and linking have also been checked with glslangValidator; GPU execution remains untested.

## Run on the original Switch V1

1. Create `switch/sr2-native-probe` on the SD card.
2. Copy `sr2-native-probe.nro` into it. No asset folder is needed.
3. Launch the homebrew menu in full-memory/title-override mode: hold **R** while starting an installed game, then select the probe. Avoid Album applet mode for this test.
4. Look for green ground, a brown obstacle and a pink hopping sphere. Test the controls below.
5. Run for several minutes; press **+** to return to the launcher.
6. Retrieve `sdmc:/switch/sr2-native-probe/log.txt` and report the console/Atmosphere versions and whether the scene rendered. A previous log is retained as `log.prev.txt`.

| Control | Action |
|---|---|
| Left stick | Move |
| Right stick | Look |
| A | Jump |
| Hold ZL, aimed at the sphere | Suction/capture |
| ZR | Release the captured sphere |
| X | Reset the sphere and inventory |
| + | Exit |

The log records graphics vendor/renderer/version, shader/context errors, capture/release, actual surface dimensions, presented frames, average FPS/frame duration and process memory-query status. These are diagnostics; the FPS mean is not a frame-time distribution or a guarantee of sustained 30 FPS. A 1280×720 window is requested with vsync, and actual resolution is logged. GPU-memory accounting, sleep/resume, docked transitions and crash recovery still need dedicated tests. If initialization fails, a text screen should show the last error after graphics cleanup; photograph that message if no log is available.

The optional `switch_captest.nro` in the same artifact directory was built from bevy-horizon to test broader runtime capabilities. It is a separate upstream diagnostic, not SR2 gameplay, and retains its upstream license/credit files. Its hardware results are still unknown.
