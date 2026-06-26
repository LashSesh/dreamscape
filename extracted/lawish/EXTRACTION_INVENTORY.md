# EXTRACTION_INVENTORY — lawish

A complete inventory of what was extracted from the Kosmocrates monorepo to form
the `lawish` Wish-to-System engine, plus the dependency graph and the
core/host boundary. Grounded in the actual source (crate manifests, `cargo
tree`, ripgrep over source + tests, and CLI behavior), not names alone.

> Source: `kosmocrates-main.zip` (repo root), upstream commit
> `32a1d4c26bc9adde4aa03e62d010d40fc32436fc`.

## 1. Crate inventory (25 crates)

### Engine crates (copied verbatim, `kosmo-*` names retained)

| Crate | Layer | Role | Intra-workspace deps |
|---|---|---|---|
| `kosmo-core` | L0 | Deterministic substrate: `Digest` (SHA-256/JCS content addressing), `Q16` fixed-point, `GateResult`, `PolicyProfile` (ReportOnly default), `EvidenceBundle`/`ReplayStatus`, `RunDescriptor`, `Wish`/`WishFacet`/`WishCube`/`WishAssessment`, `assess_wish`/`assess_wish_layered`, `WishConvergenceTrace`/`LayeredConvergenceTrace`, `StagedClosureReport`/`StratumClosure`, `depends_on` | — |
| `kosmo-workbench` | L0 | Workspace scan, TaskSpec, ContextPack, RunReport | core |
| `kosmo-parseback` | L0/L3 | Real ParseBack executor: `cargo metadata` topology snapshot + drift | core |
| `kosmo-sandbox` | L0/L7 | Safe execution sandbox: hard-timeout, bounded capture, content-addressed `RuntimeWitness` | core |
| `kosmo-spectral` | L0 | Quarantined spectral math (Fiedler, Kuramoto); Q16/discrete public API | core |
| `kosmo-kcube` | L0/L8 | `.kcube` archive executor with roundtrip verification | core |
| `kosmo-foundry` | L0/L7 | Allowlisted `cargo` check executor in a policy-governed sandbox | core |
| `kosmo-store` | L0/L10 | Append-only on-disk stores (JSONL); host writes require `allow_host_write` | core, hyphae |
| `kosmo-hyphae` | L3 | Run-local topology assimilation; codematrix fingerprints; cross-language | core, workbench |
| `kosmo-intent` | L1–L4 | Wish compiler (`compile_wish`), workspace observation (`observe_workspace*`, polyglot Rust/Py/JS/Go/Java/C/C++), behavior/run probes, norms | core, hyphae, parseback, sandbox |
| `kosmo-intent-llm` | L1/L7 | LLM wish compiler (prose → Wish via Claude/OpenAI-compatible) | core, intent, llm |
| `kosmo-llm` | L7 | Shared LLM transport (Anthropic Messages / OpenAI-compatible) | — (reqwest) |
| `kosmo-pipeline` | L5/L8 | Integration pipeline: Workbench/HYPHAE/SystemCube dry-run, landscape, `propose_wishes` | core, workbench, hyphae, systemcube, store, spectral, pse-bridge |
| `kosmo-synthesizer` | L6 | `ActionSynthesizer`, `FacetScaffolder` (deterministic, write-free `Vec<FileChange>`), guided/grounded synthesis | core, hyphae, pipeline, pse-bridge |
| `kosmo-synthesizer-llm` | L7 | LLM `ActionSynthesizer` backends (Claude / OpenAI-compatible / swarm) | core, pipeline, synthesizer, pse-bridge |
| `kosmo-agent` | L6/L7 | Closed loop: pipeline → synthesize → validate → materialize → observe | core, pipeline, pse-bridge, synthesizer, materialize, intent |
| `kosmo-materialize` | L7 | Policy-gated patch application w/ backup/rollback + cargo validation | core, synthesizer, foundry |
| `kosmo-systemcube` | L8 | `SystemCube`, `BlueprintUnit`, `ContradictionEnergyReport`, `CompatibilityProfileReport`, `DDensityReport`, `KcubeExportReport` | core, kcube |
| `kosmo-pse-bridge` | L10 | Candidate-only bridge to PSE (`PseBridgeCandidate`, `MemoryRecall` trait). **kosmo-core only — no pse-core** | core |
| `kosmo-cdk-core` | L9 | CDK/ASCC calculus: `Stage`, `StageMetrics`, `QSR` predicates, `d_energy`, `Canonical` | core |
| `kosmo-coffindragger` | L9 | ASCC stack closure (`bind_systemcube`, `close_stack`/`FoldBundle`, `press_diamond`/`DiamondCubeCandidate`), KBL binding, residue exkalibration | core, cdk-core, systemcube |
| `kosmo-wings` | L9 | Wings/Ophanim condensing layer; MPK graph oracle (`classify_local`) | core, cdk-core, **pse-metatron** |

### Vendored to satisfy CDK (no kosmo deps; included under `cdk` feature)

| Crate | Role | Why vendored |
|---|---|---|
| `pse-metatron` | Periodic-Table-of-Graphs / graph property oracle (`InputGraph`, `compute_properties_with`) | `kosmo-wings::classify_local` calls it (shallow: 2 types + 1 fn) |
| `pse-types` | Shared PSE primitive types | `pse-metatron` depends on it |

### New crates (lawish-branded surface)

| Crate | Role |
|---|---|
| `crates/lawish` | **Facade** — stable public API + prelude (`parse_wish`, `assess_wish`, `render_wish_cube`, `observe_workspace*`, `plan_closure`, `build_system_cube`, `Materializer`, `FacetScaffolder`, `depends_on`, and—under `cdk`—`bind_systemcube`/`close_stack`/`press_diamond`) |
| `tools/lawish` (pkg `lawish-cli`, bin `lawish`) | **CLI** — copied `kosmo-run`, binary renamed, `--ledger`/`--plan` gated behind optional `ledger`/`plan` features |
| `tools/kosmo-cdk` (bin `kosmo-cdk`) | CDK CLI: `bind`/`stack`/`close`/`diamond`/`explain` (report-only) |

## 2. CLI inventory

`lawish` (flag-based, copied from `kosmo-run`) — selected surface:
`--wish <prose>`, `--wishlist <file>`, `--vocab`, `--layers`, `--staged`, `--mesh`,
`--insist`, `--blueprint`, `--scaffold`, `--apply`, `--commit`, `--guided`,
`--since <snap>`, `--wish-session <path>`, `--validated`, `--landscape`, `--adopt`,
`--geometry`, `--chat`, `--atelier`, `--venture`, `--doors`, `--foundry`,
`--witness`, `--parseback`, `--kcube`, `--codematrix`, `--alchemy`, `--behaviour`,
`--steward`, `--json`, `--no-color`. Adapter-gated: `--ledger` (`ledger` feature),
`--plan` (`plan` feature).

`kosmo-cdk`: `bind | stack | close | diamond | explain | serve` (report-only).

## 3. Test inventory (high level)

> Over 1,500 tests; all green on the current toolchain. Headline per-crate
> library unit-test counts: kosmo-core **445**, kosmo-hyphae **279**,
> kosmo-pipeline **135**, kosmo-intent **111**, kosmo-synthesizer **69**,
> kosmo-systemcube **54**, kosmo-pse-bridge **39**, kosmo-synthesizer-llm **31**,
> kosmo-agent **27**, kosmo-kcube **25**, kosmo-workbench **23**, kosmo-store
> **21**, kosmo-intent-llm **19**, kosmo-sandbox/parseback **17**, kosmo-llm
> **14**, kosmo-coffindragger/foundry **13**, kosmo-wings **12**,
> kosmo-materialize **11**, kosmo-cdk-core/spectral **10**, plus the CLI's
> in-process tests (**69**) and ~24 CLI integration-test binaries.

Invariant-bearing tests preserved (cited in EXTRACTION_REPORT §8):
- gate-before-score: `kosmo-cdk-core::qsr::a_failed_gate_is_a_hard_reject_no_score_overrides_it`
- fail-closed regression: `kosmo-cdk-core::qsr::density_falling_against_the_predecessor_fails_qsr`, `rising_contradiction_energy_fails_qsr`, `stack_qsr_holds_on_a_monotone_chain_and_fails_on_a_regression`; CLI `delta.rs` (regression → exit 2)
- counterfeit/suspect surfacing: `kosmo-systemcube` energy/compatibility *Insufficient* tests; CLI `--insist` (`quality.rs`, `capstone.rs`)
- report-only no-write: `kosmo-systemcube::cross_010_d_density_high_does_not_authorise_materialization`, `cross_013_export_dry_run_has_no_write_interface`
- no-pse-core: `kosmo-pse-bridge::r7_no_pse_core_dependency`
- guided-repair anti-duplicate: `kosmo-intent::catalog_rejects_reserved_and_duplicate_triggers`; CLI `suggest.rs` (did-you-mean)

## 4. Report / data-type inventory

`WishAssessment`, `WishCube`, `StagedClosureReport`, `WishConvergenceTrace`,
`LayeredConvergenceTrace`, `ValidationClosureReport`, `ParseBackReport`,
`FoundryExecutionReport`, `RuntimeWitness`, `KcubeWriteReport`, `SystemCube` +
`SystemCubeManifest`, `ContradictionEnergyReport`, `CompatibilityProfileReport`,
`DDensityReport`, `KcubeExportReport`, `MaterializeReport`; CDK: `Stage`,
`StageMetrics`, `FoldBundle`, `DiamondCubeCandidate`, residue reports; bridge:
`PseBridgeCandidate`, `PromotionRequestRecord`, `PromotionFeedback`.

## 5. Dependency graph (engine, no cycles)

```
kosmo-core ─┬─ kosmo-workbench ── kosmo-hyphae ─┬─ kosmo-store
            │                                    └─ kosmo-pipeline ─┬─ kosmo-synthesizer ── kosmo-synthesizer-llm
            ├─ kosmo-parseback                                       │      └─ kosmo-agent ── kosmo-materialize ── kosmo-foundry
            ├─ kosmo-sandbox        kosmo-intent ──(core,hyphae,parseback,sandbox)
            ├─ kosmo-spectral       kosmo-intent-llm ──(core,intent,kosmo-llm)
            ├─ kosmo-kcube ── kosmo-systemcube ──(pipeline, coffindragger)
            └─ kosmo-pse-bridge ──(pipeline, synthesizer, agent)

CDK (feature `cdk`):  kosmo-cdk-core ─ kosmo-coffindragger ─ kosmo-wings ─ pse-metatron ─ pse-types
```

## 6. Suspected core vs host boundary

- **Core (extracted, standalone):** all `kosmo-*` engine crates above + CDK +
  vendored pse-metatron/pse-types. Zero runtime dependency on the original repo.
- **Host/adapter (NOT extracted; documented):** `pse-adapter-kosmo` (powers
  `--ledger` ledger recall) and `pse-traverse` (powers `--plan` collapse plan).
  Both sit *above* the engine and are reached only through the `MemoryRecall`
  trait (defined in `kosmo-pse-bridge`, core-only) and the `traverse_bridge`
  module. Gated behind optional `ledger`/`plan` features.
- **Not relevant to the Wish-to-System machine (excluded):** the entire `pse-*`
  family (PSE crystallization, NxAlien, Infinity Ledger, HDAG), `kosmo-operator`,
  `kosmo-promote`, `kosmo-server`, `kosmo-tui`, `kosmo-substrate`, `kosmo-eval`,
  web/TUI/bindings.

## 7. Unresolved inspection questions (all resolved during extraction)

- *Does any engine crate hard-depend on `pse-core`?* No — confirmed by build +
  `r7_no_pse_core_dependency`.
- *Is `kosmo-wings → pse-metatron` deep?* No — 2 types + 1 call; vendoring
  pse-metatron+pse-types resolves it with no kosmo edits.
- *Do the CLI's PSE couplings reach the core loop?* No — only `--ledger`/`--plan`,
  cleanly isolatable and now feature-gated.
