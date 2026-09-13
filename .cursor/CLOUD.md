# Cloud Agent instructions

This file is read only by Cursor Cloud / Background agents (the local IDE
chat ignores it). It documents how this VM is provisioned and how to run,
test, and manage the app's dependencies here. See `.cursor/environment.json`,
`.cursor/install.sh`, and `.cursor/start.sh` for the actual provisioning.

## What this project is

`eight-dive` — a Rust GPUI (wgpu) desktop Kubernetes client. It reads the
kubeconfig, opens a window, and lists cluster resources.

## Environment (already provisioned)

- `install.sh` (build-time) pins Rust stable (edition 2024 needs >= 1.85),
  installs the minimal build + software-rendering deps, kubectl, and builds.
- `start.sh` (boot-time) starts Docker and the k3s cluster and writes the
  kubeconfig to `~/.kube/config`.
- You normally do NOT need to re-run these; they run automatically. Re-run a
  script only if something is actually missing.

## Building & running the app

- Build ON THE HOST: `cargo build` / `cargo run`. Docker is ONLY for k3s —
  do not try to build or run the app inside a container.
- There is no GPU (`/dev/dri`); gpui renders through Mesa's software Vulkan
  (lavapipe). Launch the GUI with `.cursor/run-app.sh`, which opens it on the
  XFCE desktop (`DISPLAY=:1`) with a real window frame.
- If a Vulkan swapchain ever fails, fall back to GL:
  `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1`.

## GUI testing & artifacts

- Use the built-in Cursor computer-use / screen-recording tools for clicking,
  screenshots, and recordings — not `xdotool`/`import`/`ffmpeg`. The app runs
  on the visible desktop (`:1`), so those tools capture it directly.

## Kubernetes (k3s) management

- A single-node k3s cluster runs in the Docker container named `k3s`;
  kubeconfig is at `~/.kube/config` (context `default`, API on
  `127.0.0.1:6443`). No `KUBECONFIG` env is needed — kube-rs and kubectl read
  the default path.
- The cluster is reused across reboots (`start.sh` starts the existing
  container instead of recreating it), so its state persists.
- Do NOT waste time debugging a broken cluster. If it misbehaves, just
  recreate it:
  ```bash
  sudo docker rm -f k3s
  bash .cursor/start.sh
  ```
- Feel free to create namespaces, pods, deployments, and any other resources
  you need while verifying a task (`kubectl apply/create ...`). It's a
  throwaway local cluster — treat it as disposable test infrastructure.
- k3s here MUST use `--flannel-backend=host-gw` (already set in `start.sh`);
  the default VXLAN backend dies with "operation not supported" in this
  nested-Docker VM.
