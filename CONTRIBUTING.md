# Contributing

## Local Git Hooks
You may install the project's git hooks so the same checks as CI run before you push:

```bash
./scripts/install-git-hooks.sh
```

This installs a `pre-push` hook that runs:
- `cargo fmt --all -- --check`
- `cargo clippy --all --all-targets -- -D warnings`
- `cargo build --all --verbose`
- `cargo test --all --verbose`

