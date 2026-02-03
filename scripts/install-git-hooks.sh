#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

mkdir -p .githooks
git config core.hooksPath .githooks

chmod +x .githooks/pre-push

echo "Installed git hooks at .githooks (pre-push)."
