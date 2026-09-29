#!/usr/bin/env bash
#
# Eight Dive — Cloud Agent runtime bring-up (boot-time).
#
# Starts the Docker daemon (this VM has no systemd) and a single-node k3s
# cluster, then exports its kubeconfig so the app can connect out of the box.
#
# Safe to re-run; already-running services are detected and left alone.
set -uxo pipefail

LOG_DIR=/tmp/cursor
mkdir -p "$LOG_DIR"

# ---------------------------------------------------------------------------
# Docker daemon (no systemd → run dockerd directly, detached).
# ---------------------------------------------------------------------------
if ! sudo docker info >/dev/null 2>&1; then
  sudo mkdir -p /etc/docker
  echo '{"storage-driver":"overlay2"}' | sudo tee /etc/docker/daemon.json >/dev/null
  # setsid so dockerd survives this script exiting.
  sudo setsid bash -c 'dockerd >/tmp/cursor/dockerd.log 2>&1' &
  for _ in $(seq 1 30); do
    sudo docker info >/dev/null 2>&1 && break
    sleep 1
  done
fi
sudo docker info >/dev/null 2>&1 || { echo "ERROR: dockerd did not start"; exit 1; }

# ---------------------------------------------------------------------------
# k3s single-node cluster.
# Reuse the existing cluster across reboots: only create the container when it
# is missing, and just (re)start it when it exists but is stopped. Cluster
# state (namespaces, workloads) is preserved.
# --flannel-backend=host-gw is REQUIRED here: the default VXLAN backend dies
# with "operation not supported" inside this nested-Docker VM.
# ---------------------------------------------------------------------------
if sudo docker ps --format '{{.Names}}' | grep -qx k3s; then
  echo "k3s: container already running"
elif sudo docker ps -a --format '{{.Names}}' | grep -qx k3s; then
  echo "k3s: container exists but stopped — starting it"
  sudo docker start k3s
else
  echo "k3s: creating container"
  sudo docker run --name k3s --privileged --cgroupns=host \
    -v /sys/fs/cgroup:/sys/fs/cgroup:rw \
    --tmpfs /run --tmpfs /run/k3s \
    -p 6443:6443 \
    -d rancher/k3s:latest server \
    --tls-san=127.0.0.1 --tls-san=localhost \
    --flannel-backend=host-gw \
    --disable-network-policy
fi

# Wait for the node to report Ready.
for _ in $(seq 1 60); do
  if sudo docker exec k3s kubectl get nodes 2>/dev/null | grep -q ' Ready '; then
    break
  fi
  sleep 2
done

# ---------------------------------------------------------------------------
# Export kubeconfig straight to the default path (server is published on
# 127.0.0.1:6443). Both kubectl and the app (kube-rs reads ~/.kube/config by
# default) then pick it up with no KUBECONFIG env needed. The redirect runs as
# the current user, so the file is user-owned.
# ---------------------------------------------------------------------------
mkdir -p "$HOME/.kube"
sudo docker exec k3s cat /etc/rancher/k3s/k3s.yaml > "$HOME/.kube/config"
chmod 600 "$HOME/.kube/config"

# A few demo namespaces so the app's namespace list shows real, varied data.
for ns in eightdive-demo monitoring backend-services; do
  sudo docker exec k3s kubectl create namespace "$ns" >/dev/null 2>&1 || true
done

sudo docker exec k3s kubectl get ns || true
echo "start.sh complete"
