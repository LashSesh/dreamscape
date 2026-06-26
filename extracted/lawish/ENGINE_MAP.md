# ENGINE_MAP — lawish

The implemented Wish-to-System machine mapped into functional layers (L0–L10),
with the extraction decision and dependency classification for every unit.

**Dependency classes:** `core` (extracted, intrinsic) · `adapter` (optional,
trait-bounded) · `host` (repo-specific, not extracted) · `boundary` (replaceable
seam) · `unresolved` (none remain).

**Extraction decisions:** `copy` (verbatim) · `new` (lawish-authored) ·
`vendor` (copied to satisfy a leaf dep) · `feature` (gated) · `stub` (no-op when
feature off) · `document` (not extracted, recorded).

## Layer → crate/module map

| Layer | Crate · key symbols (current path) | Purpose | Dep class | Decision | Justification |
|---|---|---|---|---|---|
| **L0** substrate | `kosmo-core` · `digest.rs:Digest`/`canonical_bytes`, `fixed_point.rs:Q16`, `run.rs:GateResult`/`RunDescriptor`, `policy.rs:PolicyProfile`, `evidence.rs:EvidenceBundle`/`ReplayStatus`, `closure.rs:StagedClosureReport`, `attractor.rs:WishConvergenceTrace`/`LayeredConvergenceTrace`, `precedence.rs:depends_on` | Deterministic IDs, Q16 scoring, canonical serialization, gates, policy, trace/replay, evidence, residue | core | copy | Root of the engine; zero deps; identity is content-based so names are irrelevant to determinism |
| **L0** infra | `kosmo-workbench`, `kosmo-parseback`, `kosmo-sandbox`, `kosmo-spectral`, `kosmo-kcube`, `kosmo-foundry`, `kosmo-store` | Workspace scan, ParseBack snapshots, sandboxed exec, spectral math, .kcube archive, foundry checks, append-only stores | core | copy | Substrate floors; depend only on core (store also on hyphae) |
| **L1** wish model | `kosmo-core::wish.rs` (`Wish`, `WishFacet`, `WishFacetKind`, `WishLayer`, `WishPredicate`); `kosmo-intent::compile_wish` / `compile_wish_with_norms`; vocabulary | Wish representation + prose parsing | core | copy | The attractor model and its grammar |
| **L2** hypercube | `kosmo-core::wish.rs` (`WishCube`, `WishLayerView`, `assess_wish_layered`), `attractor.rs` (`LayeredConvergenceTrace`, `RenderAnomaly`) | Layered render: existence/shape/wiring/verified/live; opacity, structural solidity, convergence | core | copy | Measurement geometry |
| **L3** observation | `kosmo-intent::observe_workspace{,_deep,_validated,_runtime,_service}`, `facets_from_{rust,python,js,clike}_dir`, `behavior_specs_from_source`, `run_probes_from_dir`; `kosmo-hyphae` (codematrix, cross-language) | Read-only workspace topology; polyglot structural extraction; run/behavior probes | core | copy | The reality side of the comparison; polyglot already implemented |
| **L4** assessment/honesty | `kosmo-core::wish.rs` (`assess_wish`, `WishAssessment`, `WishClosureStatus`); CLI honesty grade / `--insist` | Distance, met/unmet, genuine/suspect classification | core | copy (CLI honesty grade stays in `lawish-cli`) | Core measurement; honesty grading is an orchestration concern |
| **L5** project/wishlist | CLI `--wishlist`, `WishlistReading`/`WishlistDelta` (in `tools/lawish/src/main.rs`); `kosmo-pipeline::propose_wishes`, `landscape_geometry` | Wishlist aggregate, project delta, regression code, closure hints, landscape | core (lib) + boundary (CLI orchestration) | copy | Aggregation lives in the binary; the primitives are library-level |
| **L6** descent/repair | `kosmo-synthesizer` (`FacetScaffolder`, `scaffold_*`, guided/grounded); `kosmo-agent` (closed loop); CLI `--scaffold`/`--guided` | Deterministic scaffold (`Vec<FileChange>`), guided repair (rename over duplicate), directed repair | core | copy | The realization side; write-free until materialize |
| **L7** synthesis/materialize | `kosmo-materialize::Materializer` (policy-gated host writes, rollback); `kosmo-foundry`; `kosmo-llm`, `kosmo-intent-llm`, `kosmo-synthesizer-llm`; `kosmo-sandbox` | Provider fallback, LLM boundary, sandboxed exec, policy-gated mutation | core (LLM = `boundary` via reqwest/env keys) | copy | Host mutation strictly behind `PolicyProfile`; LLM keys read from env, offline `mock` provider exists |
| **L8** system topology | `kosmo-systemcube` (`SystemCube`, `BlueprintUnit`, `ContradictionEnergyReport`, `CompatibilityProfileReport`, `DDensityReport`, `KcubeExportReport`); `kosmo-kcube` | Blueprint diagnostics, contradiction energy, compatibility, dry-run export | core | copy | Required by the pipeline; report-only export proven by CROSS-013 |
| **L9** condensation/CDK | `kosmo-cdk-core` (`QSR`, `Stage`, `StageMetrics`, `d_energy`), `kosmo-coffindragger` (`bind_systemcube`, `close_stack`, `press_diamond`, `DiamondCubeCandidate`, residue), `kosmo-wings` (`classify_local` MPK), CLI `kosmo-cdk` | Condense observed structure into a QSR-certified irreducible diamond core; report-only | core (gated) + leaf-`vendor` for MPK | copy + feature `cdk` | Part of the realized machine; isolated behind a feature; pse-metatron vendored |
| **L9** MPK oracle | `pse-metatron` (`InputGraph`, `compute_properties_with`), `pse-types` | Local graph-property oracle used by `kosmo-wings` | boundary → vendor | vendor (under `cdk`) | Shallow coupling; no kosmo deps; vendoring keeps `cdk` standalone |
| **L10** bridge | `kosmo-pse-bridge` (`PseBridgeCandidate`, `PromotionRequest*`, `MemoryRecall` trait, `MemoryGroundingEntry`) | Candidate-only data boundary to PSE; recall trait | core (it is core-only by design) | copy | Pure data + trait; `r7_no_pse_core_dependency` |
| **L10** host adapters | `pse-adapter-kosmo::LedgerRecall` (`--ledger`), `pse-traverse` (`--plan`) | Ledger memory recall; collapse-plan over architecture | adapter / host | document + feature `ledger`/`plan` + `stub` | Sit above the engine; reached only via the `MemoryRecall` trait and `traverse_bridge`; not extracted to keep default build PSE-free |
| **L10** facade/CLI | `crates/lawish` (facade), `tools/lawish` (CLI `lawish`) | Stable public API + branded CLI | new | new | lawish-branded surface over the retained engine |

## Trait seams for repo-agnosticism (already present / available)

| Seam | Where | Status |
|---|---|---|
| `MemoryRecall` | `kosmo-pse-bridge` | trait present; default build has no impl wired (adapter supplies one under `ledger`) |
| `ActionSynthesizer` | `kosmo-synthesizer` | trait; `Mock`/`Contextual`/`Grounded`/LLM impls |
| `PatchValidator` | `kosmo-materialize` | trait; `CargoFoundryValidator` impl |
| LLM provider | `kosmo-llm` + `kosmo-synthesizer-llm` | provider enum (claude/openai-compatible/mock); env-keyed |
| Policy | `kosmo-core::PolicyProfile` | report-only default; operator-approved opt-in |
| Sandbox runner | `kosmo-sandbox::Sandbox`/`RunSpec` | concrete; bounded, content-addressed witness |

> No `unresolved` dependencies remain: the full workspace compiles, tests, and
> lints standalone. The only items classified `adapter`/`host` are the two
> optional CLI features, explicitly documented and gated.
