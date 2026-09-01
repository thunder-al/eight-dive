# Eight Dive

a local-only, plug-and-play, cloud-agnostic desktop Kubernetes client which does not require on-cluster setup. built to be lightweight and fast.

## Goals

Eight Dive is inspired by k9s and Lens. it tries to combine the best of both.

k9s is a Go TUI. for everyday list, watch, and filter it works good enough. using k9s as a baseline for the ergonomics and a keyboard workflow.

Lens is a huge Kubernetes IDE, a web-based and proprietary app. UI approaches: a real window, resource detail, dashboards, the shape of a desktop client.

## Init

backend architecture, k8s client, GUI window, tokio-smol bridge.

## Basic UI: context, table, watch

context picker (basic kubveconfig support).
general api-resource abstract table with live watch.
the table: namespace all/one/non-namespaced, inline simple filter `/`, sorting.
handle common exceptions: forbidden, no items, api/connection errors.

## Resources And Lists

presets for well known resources: pod, deploy, sts, services, ingress, etc.
custom actions and views. ex: event click jumps to the object.
resource statuses, problems.
diff highlights: new/deleted rows, changed properties, numeric stats highlight

## Inspect: describe, YAML, logs

selected resource: describe, read-only YAML with highlight + watch, simple pod logs (pick container, follow).

## Keyboard and command palette

add keybinds to implemented actions and views (table selection, table sort, etc).
`Ctrl+K` palette (context, namespace, kind, resource, fuzz+alias resource search).

## Basic Eedit Actions

resource delete, scale, etc with options

## Resource Editor

open and edit resource with embedded YAML text editor.
openapi validation.
watch changes and invalidate on conflict.
code formatter.
dry-run and real apply, errors highlighter.

## Credential Bridge

env so helm/kubectl/shell do not get client keys on disk.
KUBECONFIG env + temp cred file with real config.
`user.exec` (aws, gcloud, az, oidc/kubelogin) support.

## Terminal, exec

embedded terminal with auto attached credentials via temp file+env.
exec/attach into a container.

## Tabs

add tabs (and maybe split tabs) support

## Port-forward

port-forward start/stop manager

## Dashboard

simple grafana like dashboards with cluster vitals:
* nodes, statuses
* warning+ events
* sum requests, limit, utilisation, overcommit check by nodes (kube-capacity)
* metrics-server graphs

## Helm Integration

list/history from `sh.helm.release.v1.*` ourselves (helm subcommand is super slow, list, history, details parsing is simple).
install/upgrade/uninstall/rollback via `helm` on PATH + bridge.
values editor, template/dry-run preview.
later: Artifact Hub explorer/install/upgrade. release progress viewer (track release secret + watch dependent resources)

## RBAC

RBAC editor (tables, rule builder / SSAR (?) / who-can).

## MCP + cli LLM agent support

MCP server for codex/claude/cursor cli agents.

## Dependency Graph aka xray

UI tree/graph (and text mode for MCP later).
workload, traffic, config/secrets, storage, HPA/PDB/NetworkPolicy, SA/RBAC, Helm labels.
broken-path filter.
custom connections (via annotations maybe)
add dashboard widget (?)

## Later/maybe

encrypted (os or password) credential valut, per-cluster configuration and connection view.
stub kubeconfig with proxy config + local server creds from memory + audit log (an upgrade of the temp cred file + env).
k8s audit log support.
custom logging integration + explorer (VMLogs, ELK, Loki).
custom metric explorer (viktoria, prom, ELK).
PV file manager with multiple modes (exec, sidecar): download, upload, inline edit.
argoci integration.
