# lawish-contractor ("RigidRick")

**lawish-contractor** is an isolated, repo-agnostic **Wish-to-Closure engine** — an
infrastructural exoskeleton that any AI or agentic framework can integrate with
to drive a *wish* from its starting point to **100% Definition-of-Done (DoD)
fulfillment** without prompt-drift or context-loss across that distance.

## The machine

```text
parse a wish ─▶ observe a workspace ─▶ render it as a layered hypercube ─▶
measure realization ─▶ report missing facets + closure hints ─▶ detect
regressions / counterfeit progress ─▶ (policy-gated) scaffold & apply ─▶
surface architecture defects ─▶ build SystemCube diagnostics ─▶
(optional) condense to a certified diamond core.
```

### Invariants (preserved exactly)

- **Scores rank, gates decide.** A failed gate is a hard reject no score overrides.
- **Fail-closed.** Unobserved facets are *unmet*; the default policy is
  report-only and never writes host files.
- **Evidence / trace / replay.** Durable artifacts are content-addressed
  (`SHA-256(JCS(content))`) and carry evidence + replay status; identity is
  deterministic — no wall-clock, path, or randomness in IDs.
- **Residue is visible**, never silently swallowed.

## Quick start

```sh
cd extracted/lawish
cargo build                                   # facade lib + lawish CLI (standalone)
cargo test --workspace                         # ~1,500 tests
cargo run --bin lawish -- --vocab              # how to phrase a wish

# measure one wish against any workspace:
cargo run --bin lawish -- --wish "a module parser" examples/simple_wish

# a project definition-of-done (expect realized 2/3, exit 1):
cargo run --bin lawish -- --wishlist examples/wishlist_project/dod.wishes examples/wishlist_project

# layered hypercube render, delta vs a prior snapshot, scaffold, blueprint:
cargo run --bin lawish -- --wish "a function add" --layers examples/simple_wish
cargo run --bin lawish -- --wishlist dod.wishes --since snapshot.json .
cargo run --bin lawish -- --wish "a module x" --scaffold .
cargo run --bin lawish -- --wishlist dod.wishes --blueprint .
```

### Library use

```toml
[dependencies]
lawish = { path = "extracted/lawish/crates/lawish" }
```
```rust
use lawish::prelude::*;
let wish = parse_wish("a crate api", Digest::ZERO, Digest::ZERO);
let observed = observe_workspace_deep(".").unwrap();
let cube = render_wish_cube(&wish, &observed, Digest::ZERO);
```

## Optional features

- **`cdk`** — the L9 condensation machine (CDK / CoffinDragger / Diamond),
  report-only and fail-closed: `cargo build -p lawish --features cdk`.
  CLI demo: `cargo run -p kosmo-cdk -- explain`.
- **`ledger` / `plan`** (on the CLI) — adapter integrations (`--ledger` ledger
  recall, `--plan` collapse plan) that depend on upstream crates not vendored
  here; the default build is PSE-free and these report a fail-closed message.
- **`structure`** — the real-but-not-yet-wired pieces of the full Wish-to-System
  vision: topological mesh + 5D phase space (`pse-traverse`), tensor substrate
  (`phase-matrix`), and the 5D/2D HDAG (`pse-adapter-il`, `mef-hdag`). Re-exported
  as `lawish::{topology, tensor_cells, hdag, hdag_mef}` for study; **not** wired
  into the wish loop (integration pending the formal spec). Build:
  `cargo build -p lawish --features structure`. See `STRUCTURE_GAP_ANALYSIS.md`.

## Layout

```
crates/kosmo-*        engine crates (verbatim from source; names retained this pass)
crates/lawish         facade — stable public API + prelude
crates/pse-metatron   vendored MPK oracle (only for the `cdk` feature)
crates/pse-types      vendored (dep of pse-metatron)
tools/lawish          the `lawish` CLI  (package `lawish-cli`)
tools/kosmo-cdk       the report-only CDK CLI
examples/             runnable demo workspaces
```

## Documentation

- [`EXTRACTION_INVENTORY.md`](EXTRACTION_INVENTORY.md) — crates, CLIs, tests, deps, boundary.
- [`ENGINE_MAP.md`](ENGINE_MAP.md) — every unit mapped to layers L0–L10 with decisions.
- [`EXTRACTION_REPORT.md`](EXTRACTION_REPORT.md) — what was/wasn't extracted, results, **standalone verdict**.
- [`MIGRATION_NOTES.md`](MIGRATION_NOTES.md) — mapping back to the monorepo + next refactors.

**Status:** *mostly standalone with adapters* — the Wish-to-System machine is
fully standalone (compile + runtime); only the two optional CLI adapter features
remain documented couplings. See `EXTRACTION_REPORT.md §13`.

## License

MIT (inherited from the source project).
