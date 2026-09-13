#!/usr/bin/env bash
#
# Eight Dive — Cloud Agent environment provisioning (build-time).
#
# Installs the Rust toolchain, GPUI/wgpu build + software-rendering
# dependencies, kubectl, and pre-builds the project. Runtime services
# (Docker + k3s) are brought up separately by .cursor/start.sh.
#
# Safe to re-run; every step is idempotent.
set -euxo pipefail

# ---------------------------------------------------------------------------
# Rust toolchain
# The crate targets edition 2024, which needs rustc >= 1.85. The base image
# ships 1.83, so pin to the latest stable.
# ---------------------------------------------------------------------------
rustup default stable
rustup update stable
rustc --version

# ---------------------------------------------------------------------------
# System dependencies (trimmed to what the crate actually needs — verified by
# inspecting `ldd` of the binary and the -sys crates' build-script link flags).
#
#   build : cc/make (ring's C/asm, wayland-backend shim) + pkg-config
#   link  : the only libraries linked at build time are xcb, xkbcommon(+x11)
#           and fontconfig/freetype (libfontconfig1-dev pulls libfreetype-dev,
#           the owner of the libfreetype.so that `-lfreetype` resolves against)
#   run   : gpui renders through Mesa's software Vulkan (lavapipe) since there
#           is no /dev/dri; mesa-vulkan-drivers pulls the libvulkan1 loader.
#           libgl1-mesa-dri provides software GL/EGL (used for probing and for
#           the optional WGPU_BACKEND=gl fallback).
#   k3s   : docker.io + iptables (see start.sh)
#
# Not needed (all dlopen'd at runtime or unused): libwayland-dev, libx11-dev,
# the xcb render/shape/xfixes -dev pkgs, libvulkan-dev, GL -dev pkgs, cmake,
# libssl-dev (crate uses rustls), libasound2-dev/libudev-dev (no such -sys
# crates in the graph).
# ---------------------------------------------------------------------------
export DEBIAN_FRONTEND=noninteractive
sudo apt-get update -y
sudo apt-get install -y --no-install-recommends \
  build-essential pkg-config \
  libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev \
  libfontconfig1-dev libfreetype-dev \
  mesa-vulkan-drivers libgl1-mesa-dri \
  docker.io iptables \
  curl

# ---------------------------------------------------------------------------
# kubectl (client only; the API server is provided by the k3s container)
# ---------------------------------------------------------------------------
if ! command -v kubectl >/dev/null 2>&1; then
  KVER="$(curl -fsSL https://dl.k8s.io/release/stable.txt)"
  curl -fsSLo /tmp/kubectl "https://dl.k8s.io/release/${KVER}/bin/linux/amd64/kubectl"
  sudo install -m 0755 /tmp/kubectl /usr/local/bin/kubectl
  rm -f /tmp/kubectl
fi
kubectl version --client

# ---------------------------------------------------------------------------
# Pre-build so the first `cargo run` is fast.
# ---------------------------------------------------------------------------
cargo build

echo "install.sh complete"
