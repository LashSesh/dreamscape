# MIGRATION_NOTES — lawish

How the extracted `lawish` workspace maps back to the Kosmocrates monorepo, how
to re-pull upstream changes, and the recommended follow-up refactors.

## Provenance

- **Source:** `kosmocrates-main.zip` at repo root, upstream commit
  `32a1d4c26bc9adde4aa03e62d010d40fc32436fc`.
- **Toolchain:** Rust stable (extracted & tested on cargo/rustc 1.94.1; upstream
  MSRV 1.82, preserved in `[workspace.package].rust-version`).
- The original repository is unchanged except for this `extracted/lawish/`
  folder and the extraction docs.

## Crate mapping (extracted ⇐ source path)

| lawish path | source path |
|---|---|
| `crates/kosmo-*` (19 engine crates) | `crates/kosmo-*` (verbatim) |
| `crates/kosmo-cdk-core`, `kosmo-coffindragger`, `kosmo-wings` | `crates/…` (verbatim) |
| `crates/pse-metatron`, `crates/pse-types` | `crates/…` (verbatim, vendored) |
| `tools/kosmo-cdk` | `tools/kosmo-cdk` (verbatim) |
| `tools/lawish` (pkg `lawish-cli`, bin `lawish`) | `tools/kosmo-run` (renamed; `--ledger`/`--plan` gated) |
| `crates/lawish` (facade) | *new* |
| `Cargo.toml` (workspace) | distilled from root `Cargo.toml` (`[workspace.package]`/`[workspace.dependencies]`) |
| `clippy.toml` | `clippy.toml` (verbatim — `too-many-arguments-threshold = 12`) |
| `Cargo.lock` | root `Cargo.lock` (pruned to this subset by cargo) |

## What changed vs. the source (exhaustive)

1. **Workspace manifest** — new, lists only the extracted members; lawish-branded
   `[workspace.package]`; `default-members = [lawish, lawish-cli]`.
2. **`tools/lawish/Cargo.toml`** — package `kosmo-run` → `lawish-cli`; `[[bin]]`
   `kosmo-run` → `lawish`; removed `pse-adapter-kosmo` + `pse-traverse` deps;
   added `[features] ledger`, `plan`.
3. **`tools/lawish/src/main.rs`** — `use pse_adapter_kosmo::LedgerRecall` and its
   single use site in `open_recall` gated behind `#[cfg(feature = "ledger")]`
   with a fail-closed stub.
4. **`tools/lawish/src/traverse_bridge.rs`** — `pse_traverse`-backed functions
   gated behind `#[cfg(feature = "plan")]`; the plain `CollapsePlanView` types
   stay always-available; a `#[cfg(not(feature = "plan"))]` stub returns a
   fail-closed error.
5. **CLI tests** — `env!("CARGO_BIN_EXE_kosmo-run")` → `…_lawish` (binary rename);
   `missing_ledger_is_a_hard_error` gated to `feature = "ledger"`;
   `wishlist_plan_renders_collapse_plan_and_excises_a_heap` gated to
   `feature = "plan"`.
6. **`tools/lawish/kosmo-run.wishes`** — self-DoD wish `a crate kosmo-run` →
   `a crate lawish-cli` (matches the renamed package).
7. **`cargo fmt`** — the fresh workspace was normalized once to the current
   rustfmt (cosmetic only; upstream was formatted with a different rustfmt
   version). No semantic changes.

No engine logic, type, gate, or identity behavior was altered.

## Structural extraction (second pass — the `structure` feature)

Four real-but-previously-disjoint crates carrying the mesh / phase-space / tensor / HDAG structures
were extracted verbatim and exposed behind the facade's non-default `structure` feature:

| lawish path | source path | structure |
|---|---|---|
| `crates/pse-traverse` | `crates/pse-traverse` | MeshHolo, PhaseSpaceWindow, FieldCube, CollapsePlan |
| `crates/phase-matrix` | `crates/phase-matrix` | FieldTensorState, CouplingMatrix |
| `adapters/pse-adapter-il` | `adapters/pse-adapter-il` | 5D ResonanceTensor + HDAG (`src/hdag.rs`) |
| `vendors/infinityledger/mef-hdag` | `vendors/infinityledger/mef-hdag` | 2D phase/time HDAG (leaf) |

Two **manifest-only** trims decoupled the cross-subsystem bridges so these compile standalone against
`pse-types` + external crates (no source/logic change; the gated code is left intact but uncompiled):

8. **`crates/pse-traverse/Cargo.toml`** — removed the optional `pse-core` / `pse-graph` path deps;
   `default` features changed from `["pse-commit","horizon","cognition","topology"]` to
   `["horizon","cognition","topology"]`; `pse-commit = []` kept declared (its `bridge.rs` is left in
   place but only compiled when `pse-commit` is enabled, which now requires the PSE-core crates from the
   original repo — enabling `pse-commit` would pull ~18 PSE-core crates).
9. **`adapters/pse-adapter-il/Cargo.toml`** — removed the six optional `mef-*` Infinity-Ledger path deps;
   `il-pipeline = []` kept declared (the `il-pipeline`-gated code remains but is uncompiled; enabling it
   requires vendoring the mef-* crates).

These are re-exported as `lawish::{topology, tensor_cells, hdag, hdag_mef}` under `--features structure`
and are **NOT wired** into the wish loop — the integration (Wish→tensor, WishCube→SystemCube, HDAG
weaving, diamond→domain) is the forthcoming-spec work and was deliberately not attempted.

## Re-pulling upstream changes

The engine crates are byte-faithful copies. To sync a newer Kosmocrates:

1. Re-extract the new `kosmocrates-main.zip`.
2. `cp -r` the changed `crates/kosmo-*` (and `pse-metatron`/`pse-types` if CDK
   changed) over `extracted/lawish/crates/…`.
3. Re-apply the 7 deltas above to `tools/lawish` (they are small and localized;
   `git diff` against the copied `kosmo-run` shows them).
4. `cargo fmt && cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`.

A future `lawish-*` rename (below) would make step 3 the only manual part.

## Recommended next refactors

1. **Internal `kosmo-* → lawish-*` rename.** Rename crate dirs and `name =`
   fields, and rewrite `use kosmo_*` / `kosmo_*::` references. This is the only
   remaining "kosmocrates" trace in crate identifiers. It is purely mechanical
   and does **not** affect content-addressed identity (IDs are
   `SHA-256(JCS(content_fields))`, independent of crate names). Recommended via
   `cargo`-aware rename tooling, one crate at a time, leaf-first.
2. **Rebrand CLI runtime output strings** (`"Kosmocrates wish"`, `"Kosmocrates
   wishlist"`, `"Kosmocrates collapse plan"`, `kosmo-cdk explain` header). These
   are user-facing but currently asserted on by ~6 CLI tests; rebrand the strings
   and the corresponding `stdout.contains("Kosmocrates …")` assertions together.
3. **First-class `MemoryRecall` adapter** in-tree (e.g. a file/JSONL recall) so
   `--ledger` works without vendoring `pse-adapter-kosmo`.
4. **Optionally vendor `pse-traverse`** (default-features-off) to make `--plan`
   work standalone, or reimplement the small collapse-plan kernel against
   `kosmo-core` types behind the `plan` feature.
5. **Library-level wishlist/delta API** — lift `WishlistReading`/`WishlistDelta`
   out of `tools/lawish/src/main.rs` into the facade so non-CLI consumers get
   project-delta/regression detection directly.
