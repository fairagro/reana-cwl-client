# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`reana-cwl-client` is a Rust client library (and small CLI) for [REANA](https://reanahub.io/), a reproducible research data-analysis platform developed by CERN. It builds CWL documents using the [`commonwl`](https://github.com/fairagro/commonwl) CWL parsing library (sibling repo `../commonwl`) and is itself used by [SciWIn](https://github.com/fairagro/sciwin) (sibling repo `../sciwin`) to talk to REANA instances. REANA can run workflows in several languages (CWL, Snakemake, Yadage, Serial) at scale; this client targets the CWL case.

Cargo workspace members (`crates/`):
- `reana` — the core client library (crate name `reana`): `api/` (REANA HTTP API — `client.rs`, `workflows.rs`, `response.rs`), `client.rs` (higher-level client), `models/` (REANA data models, e.g. `workflows.rs`), `storage.rs` (workflow file staging), `logs.rs`, `error.rs`, `io.rs`. Re-exports `reana_auth` as the `auth` module. Also defines `wrap_tools()` in `lib.rs`: since REANA has no concept of a standalone CWL `CommandLineTool`/`ExpressionTool`, this wraps one into a single-step `Workflow` before submission.
- `reana-auth` — authentication abstraction: the `TokenProvider` async trait and `ReanaAccessToken`, used to supply REANA access tokens without the core `reana` crate needing to know how tokens are obtained/stored (token values are wrapped in `secrecy::SecretString`).
- `reana-client` (bin/crate name `reana-cwl`) — a CLI (`cli.rs`, `commands/workflows.rs`) built on top of the `reana` library.

## Common commands

```bash
cargo build --workspace
cargo clippy --workspace -- -W clippy::pedantic     # matches CI lint step
cargo nextest run --workspace --no-fail-fast        # matches CI test step (requires cargo-nextest)
cargo test -p reana some_test_name                  # run a single test
```

CI (`.github/workflows/ci.yaml`) runs clippy, `cargo nextest`, then coverage via `cargo tarpaulin`, on Ubuntu only. `.github/workflows/publish.yaml` publishes to crates.io (`cargo publish --all-features`) on `v*.*.*` tags.
