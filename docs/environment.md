# Cloud development environment

Use the existing `/workspace/SR2-NX` checkout; each cloud task is already isolated. Do not create a Git worktree unless explicitly requested.

This machine stores rustup/cargo under `/workspace/toolchains` to respect filesystem permissions:

```sh
export CARGO_HOME=/workspace/toolchains/cargo
export RUSTUP_HOME=/workspace/toolchains/rustup
export PATH="$CARGO_HOME/bin:$PATH"
cd /workspace/SR2-NX
cargo check --locked --workspace
cargo test --locked --workspace
cargo run --locked -p sr2-desktop
```

A graphical desktop or virtual X server and Vulkan driver are required for rendering. The cloud setup uses an existing Xorg dummy driver plus Debian's signature/hash-verified Mesa Lavapipe package extracted into `/workspace/sr2-system/root` without root installation. This software renderer is a functional check only, not a Switch performance estimate.

For this prepared machine, use `/workspace/sr2-system/xorg.conf` for Xorg `:99`, `DISPLAY=:99`, `XDG_RUNTIME_DIR=/workspace/sr2-system/runtime`, `VK_DRIVER_FILES=/workspace/sr2-system/lvp.json`, and `WGPU_BACKEND=vulkan`. Start a new Xorg process after task restoration; running processes do not survive snapshots. Optional `SR2_SMOKE_SCREENSHOT=/tmp/sr2-smoke.png` captures frame 90 during `--smoke`.

The MinGW cross compiler is extracted in `/workspace/sr2-system/root`, with command aliases in `/workspace/sr2-system/bin`. To build a Windows GNU executable here:

```sh
export PATH="/workspace/sr2-system/bin:/workspace/sr2-system/root/usr/bin:$PATH"
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=/workspace/sr2-system/root/usr/bin/x86_64-w64-mingw32-gcc-posix
export CC_x86_64_pc_windows_gnu="$CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER"
cargo build --locked -p sr2-desktop --target x86_64-pc-windows-gnu --target-dir target/windows-build -j 2
```

A PE executable proves cross-compilation, not Windows execution. Windows CI builds with its native toolchain. Native Switch instructions and blockers are separate in `switch-feasibility.md`.

The saved configuration draft adds `pkg.devkitpro.org` for native SDK packages and `api.github.com` for PR API access. Review and save these changes in Environment Settings, then publish the environment. Saving the draft alone did not update this running machine's network policy.
