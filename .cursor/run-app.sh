#!/usr/bin/env bash
#
# Launch the Eight Dive GUI on the Cloud Agent desktop.
#
# The VM has no GPU (/dev/dri), so gpui renders through Mesa's software
# Vulkan (lavapipe/llvmpipe). It draws onto the existing XFCE desktop on
# DISPLAY :1 so the window gets a proper title bar / frame.
#
# Usage: .cursor/run-app.sh            # runs the debug build
#        BUILD=release .cursor/run-app.sh
set -uxo pipefail

# Desktop provided by the base image (XFCE + xfwm4).
export DISPLAY="${DISPLAY:-:1}"
export XAUTHORITY="${XAUTHORITY:-$HOME/.Xauthority}"
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp/runtime-$USER}"
mkdir -p "$XDG_RUNTIME_DIR" && chmod 700 "$XDG_RUNTIME_DIR"

# Software Vulkan (lavapipe). Rendered fine here; if a Vulkan swapchain ever
# fails, fall back to the GL backend by exporting:
#   export WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1
export VK_ICD_FILENAMES="${VK_ICD_FILENAMES:-/usr/share/vulkan/icd.d/lvp_icd.json}"
export VK_DRIVER_FILES="${VK_DRIVER_FILES:-$VK_ICD_FILENAMES}"

# The app connects to the local k3s cluster via the default kubeconfig
# (~/.kube/config), written by start.sh — no KUBECONFIG env needed.

cd "$(dirname "$0")/.."
if [ "${BUILD:-debug}" = "release" ]; then
  cargo run --release
else
  cargo run
fi
