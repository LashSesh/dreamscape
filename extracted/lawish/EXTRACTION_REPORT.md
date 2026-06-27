# EXTRACTION_REPORT — lawish

## 1. Executive summary

`lawish` is the **Wish-to-System engine** of the Kosmocrates monorepo,
surgically extracted into a self-contained, repo-agnostic Cargo workspace under
`extracted/lawish/`. It compiles, tests, and lints **fully standalone**: the
default build has **zero dependency on the original repository** and zero `pse-*`
crates. 25 crates were extracted (19 engine + 3 CDK + 2 vendored MPK leaves + 1
facade), plus two CLIs. Over **1,500 tests pass**; `cargo fmt --check` and
`cargo clippy --workspace --all-targets -- -D warnings` are both clean. The
machine runs end-to-end: a wish parses, observes any workspace, renders a layered
hypercube, measures realization, reports missing facets and closure, detects
regressions/counterfeit progress, scaffolds under explicit policy, surfaces
architecture defects, builds SystemCube diagnostics, and (under `--features
cdk`) condenses structure into a QSR-certified diamond core.

**Verdict: mostly standalone with adapters** (see §13).

## 2. What the engine is

A machine that takes wishes as structured attractors, renders them through
layered/hypercube measurement against an observed workspace topology, descends
toward realization, verifies reality (gates, not just scores), repairs guided
defects, and optionally condenses system structure into an irreducible certified
core. It is designed as an **infrastructural exoskeleton**: any AI/agentic
framework can drive a wish from start to **100% Definition-of-Done** through the
`lawish` facade or CLI, with deterministic, content-addressed, replay-aware
artifacts instead of prompt-drift.

Preserved invariants: **gates decide, scores only rank**; **fail-closed**
(unobserved = unmet, report-only by default); **evidence/trace/replay** on durable
artifacts; **deterministic identity** (`SHA-256(JCS(content))`, no wall-clock /
path / randomness in IDs); **residue is visible, never swallowed**.

## 3. What was extracted

- **L0 substrate:** `kosmo-core` + `kosmo-workbench`/`parseback`/`sandbox`/
  `spectral`/`kcube`/`foundry`/`store`.
- **L1–L7 wish loop:** `kosmo-intent`(+`-llm`), `kosmo-llm`, `kosmo-hyphae`,
  `kosmo-pipeline`, `kosmo-synthesizer`(+`-llm`), `kosmo-agent`,
  `kosmo-materialize`.
- **L8 topology:** `kosmo-systemcube` (+`kosmo-kcube`).
- **L9 condensation (feature `cdk`):** `kosmo-cdk-core`, `kosmo-coffindragger`,
  `kosmo-wings`, with `pse-metatron` + `pse-types` vendored.
- **L10 bridge:** `kosmo-pse-bridge` (core-only).
- **Structural extraction (feature `structure`):** the real-but-previously-disjoint
  pieces of the full Wish-to-System vision — `crates/pse-traverse` (`MeshHolo`,
  `PhaseSpaceWindow`, `FieldCube`, `CollapsePlan`), `crates/phase-matrix`
  (`FieldTensorState`, `CouplingMatrix`), `adapters/pse-adapter-il` (5D
  `ResonanceTensor` + `HDAG`), `vendors/infinityledger/mef-hdag` (2D `HDAG`).
  Re-exported as-is, **not wired** into the wish loop (integration is future-spec
  work). Two cross-subsystem bridges decoupled at the manifest level only (see §10
  and `MIGRATION_NOTES.md`). See `STRUCTURE_GAP_ANALYSIS.md`.
- **Surface:** `lawish` facade crate + `lawish` CLI + `kosmo-cdk` CLI.

See `EXTRACTION_INVENTORY.md` and `ENGINE_MAP.md` for the full per-symbol map.

## 4. What was intentionally NOT extracted

- Most of the `pse-*` family (PSE crystallization, NxAlien governance, Infinity
  Ledger pipeline, QTIC, eval/bench) and the PSE-core closure (~18 crates) — a
  separate subsystem. **Exception:** the four *structural* crates listed above
  (`pse-traverse`, `phase-matrix`, `pse-adapter-il`, `mef-hdag`) were extracted
  under the `structure` feature because they carry the mesh / phase-space /
  tensor / HDAG structures of the full vision; their bridges *into* PSE-core /
  the IL ledger were decoupled so they stand alone.
- Host tools `kosmo-operator`, `kosmo-promote`, `kosmo-server`, `kosmo-tui`,
  `kosmo-substrate`, `kosmo-eval`; web UI, TUI, node/python bindings.
- **Adapter couplings** `pse-adapter-kosmo` (powers `--ledger`) and `pse-traverse`
  (powers `--plan`): gated behind optional `ledger`/`plan` features with
  fail-closed stubs, documented rather than vendored, to keep the default build
  100% PSE-free. See §10.

## 5. Dependency closure

External crates (default build): `serde`, `serde_json`, `serde_jcs`, `sha2`,
`thiserror`, `reqwest` (LLM transport; offline `mock` provider exists), `libc`
(unix sandbox). Under `--features cdk`, additionally `nalgebra`, `rayon`,
`ordered-float`, `chrono` (via vendored `pse-metatron`/`pse-types`); the
`kosmo-cdk` CLI adds `axum`/`tokio`. No path dependency escapes
`extracted/lawish/`. `Cargo.lock` is committed for reproducibility.

## 6. Public API (facade crate `lawish`)

Stable verbs (thin, faithful wrappers) + curated re-exports:
`parse_wish`, `observe_workspace`/`_deep`/`_validated`/`_runtime`, `assess_wish`,
`render_wish_cube`, `plan_closure`, `build_system_cube`, `depends_on`,
`FacetScaffolder`, `Materializer` (the only policy-gated host-write path), and —
under `cdk` — `bind_systemcube`/`close_stack`/`press_diamond`/
`DiamondCubeCandidate`. Engine crates are re-exported under module aliases
(`lawish::core`, `::intent`, `::systemcube`, …) and a `lawish::prelude`.

## 7. CLI usage

```sh
cargo run --bin lawish -- --vocab                      # wish vocabulary
cargo run --bin lawish -- --wish "a crate api" .       # measure one wish
cargo run --bin lawish -- --wishlist dod.wishes .      # project DoD gauge
cargo run --bin lawish -- --wish "…" --layers .        # hypercube render
cargo run --bin lawish -- --wishlist dod.wishes --since snap.json . # delta/regression
cargo run --bin lawish -- --wish "…" --scaffold .      # deterministic scaffold (no write)
cargo run --bin lawish -- --wish "…" --apply .         # policy-gated apply
cargo run --bin lawish -- --wishlist dod.wishes --blueprint . # architecture defects
# CDK (report-only):
cargo run -p kosmo-cdk -- explain
```

## 8. Test results

All green on cargo 1.94.1. Headline: `kosmo-core` 445, `kosmo-hyphae` 279,
`kosmo-pipeline` 135, `kosmo-intent` 111, `kosmo-synthesizer` 69, `kosmo-systemcube`
54, `kosmo-pse-bridge` 39, … plus the CLI's 69 in-process tests and ~24 CLI
integration-test binaries. `cargo fmt --check` clean; `cargo clippy --workspace
--all-targets -- -D warnings` clean; doc-tests ok.

Validation-criteria evidence (all present & passing):

| Criterion | Test |
|---|---|
| End-to-end single wish | facade `wish_against_empty_topology_fails_closed`; CLI `--wish` smoke (REALIZED) |
| Wishlist / project measurement | CLI `wishlist.rs::wishlist_reports_aggregate_and_gates_unrealized` |
| Delta / regression report | CLI `delta.rs` (regression → exit 2) |
| Missing facets + closure hints | facade `layered_render_and_empty_closure_are_failclosed`; CLI `--scaffold` |
| Gate-before-score | `kosmo-cdk-core::qsr::a_failed_gate_is_a_hard_reject_no_score_overrides_it` |
| Regressions fail closed | `kosmo-cdk-core::qsr::density_falling_against_the_predecessor_fails_qsr`, `rising_contradiction_energy_fails_qsr` |
| Counterfeit/suspect surfaced | `kosmo-systemcube` *Insufficient* energy/compat tests; CLI `--insist` (`quality.rs`, `capstone.rs`) |
| Guided repair ≠ duplicate | `kosmo-intent::catalog_rejects_reserved_and_duplicate_triggers`; CLI `suggest.rs` |
| Report-only no-write | `kosmo-systemcube::cross_010_…`, `cross_013_export_dry_run_has_no_write_interface` |
| Evidence/replay/identity | `kosmo-core` `*_id_is_independent_of_elapsed`, CROSS-006 bundle tests |
| No pse-core in default build | `kosmo-pse-bridge::r7_no_pse_core_dependency` + clean standalone build |

## 9. Known limitations

- `--ledger` and `--plan` require their feature + the upstream adapter crates
  (not vendored); default build returns a fail-closed "feature not enabled" error.
- LLM-backed modes need an API key (`ANTHROPIC_API_KEY` / `KOSMO_LLM_API_KEY`);
  the deterministic `mock` provider and all offline measurement work without one.
- Some runtime/observation tests shell out to `cargo`; they self-skip when the
  toolchain or workspace is unavailable ("observe unavailable, skipping").
- CLI runtime output still prints "Kosmocrates …" headers (see §10).

## 10. Remaining repo-specific couplings

| Coupling | Classification | Status |
|---|---|---|
| `--ledger` → `pse-adapter-kosmo::LedgerRecall` | adapter | gated behind `ledger`; trait `MemoryRecall` is core-only |
| `--plan` → `pse-traverse` | adapter | gated behind `plan`; `CollapsePlanView` types remain, kernel stubbed |
| Internal crate names `kosmo-*` | cosmetic | retained this pass; rename is the documented next refactor |
| CLI output strings `"Kosmocrates …"` | cosmetic | retained (asserted by ~6 tests); rebrand documented in MIGRATION_NOTES |

No functional/runtime coupling to the original repo remains in the default build.

## 11. Recommended next refactors

See `MIGRATION_NOTES.md §Recommended next refactors`: (1) internal `kosmo-* →
lawish-*` rename; (2) rebrand CLI output strings + their assertions; (3) in-tree
`MemoryRecall` adapter for `--ledger`; (4) vendor/reimplement `pse-traverse` for
`--plan`; (5) lift wishlist/delta into the facade library API.

## 12. Download / use instructions

```sh
# the extracted engine is self-contained under extracted/lawish/
cd extracted/lawish
cargo build                         # facade lib + lawish CLI (standalone)
cargo test --workspace              # ~1,500 tests
cargo run --bin lawish -- --vocab
# optional condensation machine:
cargo build -p lawish --features cdk
cargo run -p kosmo-cdk -- explain
```
As a library dependency: `lawish = { path = "…/extracted/lawish/crates/lawish" }`
then `use lawish::prelude::*;`.

## 13. Standalone verdict

**Mostly standalone with adapters.**

- **Compile-standalone:** yes — the entire workspace (engine + CDK + vendored MPK)
  builds with no original-repo path deps; `Cargo.lock` committed.
- **Runtime-standalone (core machine):** yes — wish parse/observe/assess/cube/
  delta/closure/scaffold/systemcube/CDK all run with no `pse-*` code in the
  default build.
- **Adapters:** the only couplings are the two optional CLI features
  (`--ledger`, `--plan`), trait-/module-isolated, fail-closed when absent, and
  documented. They do not affect the core engine or the default build.

This is effectively fully standalone for the Wish-to-System machine itself; the
"with adapters" qualifier covers the two intentionally-deferred, feature-gated
host integrations.
