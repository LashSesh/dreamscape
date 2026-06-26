# STRUCTURE_GAP_ANALYSIS — the "coil-and-mesh" Wish-to-System vision vs. the implemented code

This document verifies **whether and how far** a specific described architecture exists in the
Kosmocrates source (the monorepo from which `lawish` was extracted). It is an **analysis artifact
only** — no integration is implemented here.

Verified against the unzipped monorepo (upstream commit
`32a1d4c26bc9adde4aa03e62d010d40fc32436fc`) by three independent read-only audits.

## The described machine (intent)

> The engine maps a **wish as a tensor**; then phase by phase renders a **hypercube** until it
> reaches **full opacity** and the "hologram" **materializes** as a **100% deterministic blueprint
> artifact**. That blueprint then realizes the wish by being **unwound & wound like two counter-phase
> coils** — as if **two cubes**, each made of many **sediments woven internally with a hyperdimensional
> Directed Acyclic Graph** (a *topological mesh* — a "closed phase space with its own 3D-time
> semantics"), **roll head-to-head like two opposing caterpillar/tank tracks**, in order to
> **translate into a domain**.

This is the unifying vision. Six stages follow.

## Per-stage verdict

Legend: ✅ fully implemented · 🟡 partial / present-but-isolated · ⚠️ named-only/metaphor · ❌ absent.

| # | Stage | Verdict | Real code (file · symbol) |
|---|---|---|---|
| 1 | Wish → **tensor** | ⚠️ named-only | The word `tensor` is a section comment in `crates/kosmo-core/src/attractor.rs:243` ("the per-layer (tensor) descent"). **No Wish→tensor encoder exists.** Genuine tensor types exist but are unlinked to the wish: `crates/phase-matrix` (`FieldTensorState`, `CouplingMatrix`), `adapters/pse-adapter-il/src/hdag.rs` (5D `ResonanceTensor` = [temporal, morphic, relational, topological, entropic]). `kosmo-core` has **zero** references to any tensor type. |
| 2 | Phase-by-phase **hypercube → full opacity** | ✅ fully implemented | `crates/kosmo-core/src/wish.rs:642-800` `WishCube` / `assess_wish_layered` — per-layer `opacity = ONE − distance`, `structural_solidity` (geomean), 5 strata (Existence→Shape→Wiring→Verified→Live). `crates/kosmo-core/src/closure.rs:29-232` `StagedClosureReport` / `StratumClosure` (`Pending→Solving→Gated→Coagulated`/`Fractured`) with `ClosureStage` Solve→Gate→Coagula and `fully_coagulated`. `crates/kosmo-core/src/attractor.rs:309-368` `LayeredConvergenceTrace` (per-layer trajectories, `RenderAnomaly`). Tested. |
| 3 | Materialize → **100% deterministic blueprint artifact** | ✅ fully implemented (isolated) | `crates/kosmo-systemcube/src/lib.rs:124` `SystemCube`, `blueprint_unit.rs:64` `BlueprintUnit`, `manifest.rs` `SystemCubeManifest`; `crates/kosmo-kcube/src/lib.rs` `.kcube` write + roundtrip. All content-addressed (`Digest`), export report-only/policy-gated (CROSS-010/013). |
| 4 | Realize via **two counter-phase coils / two cubes rolling head-to-head** | 🟡 one-way only; gears/coils metaphorical | Forward condensation is real: `crates/kosmo-cdk-core/src/types.rs` `Stage` / `AttractorStack` / `StageMetrics`, `qsr.rs` `qsr_stage`/`qsr_stack`; `crates/kosmo-coffindragger/src/{binding.rs:bind_systemcube, stack.rs:close_stack/FoldBundle, diamond.rs:press_diamond/DiamondCubeCandidate, run.rs:purge}`. The Ophanim cycle is **real and operational**: `crates/kosmo-wings/src/ophanim.rs` `OphanimCycle::converges()` (`d(Ω^t(x),A⋆)→0`) + `roundtrip()` replay-gate — the closest thing to "rolling," but a convergence check, not counter-rotation. "Two gears" = advisory UI readout only: `tools/kosmo-run/src/main.rs` `mesh_report()` (wish-solidity ⟷ topology D-density). **No reverse** (`unpress`/`expand`/`realize`) function exists. `caterpillar`/`tread`/`tank_track` = 0 source hits. |
| 5 | Sediments woven with **hyperdimensional DAG (topological mesh; closed phase space w/ 3D-time)** | 🟡 pieces real but disjoint | "Sediments" ≈ `Stage`/`AttractorStack` accretion: `crates/kosmo-coffindragger/src/stack.rs` `verify_accretion` (ASCC-2), `verify_contraction` (ASCC-3) — residue-visible, real (the literal word "sediment" is **not** in code). HDAG real but elsewhere: `adapters/pse-adapter-il/src/hdag.rs` (`HDAGNode`/`HDAGEdge`, 5D resonance, acyclicity via coherence-gate) and `vendors/infinityledger/mef-hdag/src/graph.rs` (2D phase/time, explicit `would_create_cycle`). Topological mesh real: `crates/pse-traverse/src/topology/mesh_holo.rs` `MeshHolo` (simplicial complex, Betti numbers) and `crates/kosmo-wings/src/mesh.rs` `WingMesh` (composition algebra). Phase space real: `crates/pse-traverse/src/topology/phase_window.rs` `PhaseSpaceWindow`/`TptPoint` (5D). **"Closed" + a separate "3D-time" coordinate system is aspirational** — the space is 5D and is an open sampling container, not a closed manifold with 3 spatial + 1 temporal axes. |
| 6 | **Translate into a domain** | ❌ absent (from the condensed core) | No `DiamondCubeCandidate`→domain realization function. `crates/kosmo-materialize/src/lib.rs` `Materializer` writes synthesized **patches**, not diamonds. `crates/kosmo-wings/src/mpk_bridge.rs` `classify_local` is a finite (1≤n≤8) local graph oracle, not a domain translator. |

## The three critical wiring gaps

The individual blocks are real; the **connections that make them one machine are not built**.

1. **WishCube ↮ SystemCube (stage 2 → 3 link is missing).**
   `crates/kosmo-systemcube/` imports no `Wish*` type. `BlueprintUnit`s are built from HYPHAE topology
   decisions in `crates/kosmo-pipeline/src/lib.rs` (~`raw_units` from `hyphae.decisions`), and
   `StagedClosureReport` is never consumed by the pipeline. The realized wish hypercube and the
   materialized blueprint are **two separate cubes in separate address spaces**.

2. **CDK (kosmo-\*) ↮ HDAG / mesh / phase-space (pse-\*) (stage 5 weaving is missing).**
   The CDK condensation crates do not import the HDAG, `MeshHolo`, or `PhaseSpaceWindow`. The only link is
   one-way and asynchronous: `crates/kosmo-pse-bridge` (`PseBridgeCandidate`) → PSE → `adapters/pse-adapter-il`
   → HDAG. No sediments/stages are structurally woven with the HDAG.

3. **No reverse direction anywhere (stage 4/6 are missing).**
   The entire machine is condensation-only. There is no "unwind/rewind," no counter-rotating realization,
   and no diamond→domain expansion.

## What is in `lawish` vs. only in the full monorepo

| Building block | In extracted `lawish`? |
|---|---|
| WishCube / opacity / Solve→Gate→Coagula closure | ✅ (`kosmo-core`) |
| SystemCube / BlueprintUnit / `.kcube` blueprint | ✅ (`kosmo-systemcube`, `kosmo-kcube`) |
| Stage / AttractorStack / QSR / Diamond condensation | ✅ under the `cdk` feature (`kosmo-cdk-core`, `kosmo-coffindragger`) |
| Ophanim cycle / WingMesh | ✅ under the `cdk` feature (`kosmo-wings`) |
| HDAG (5D `ResonanceTensor`) | ✅ under the `structure` feature (`adapters/pse-adapter-il`, `src/hdag.rs`) |
| HDAG (2D phase/time) | ✅ under the `structure` feature (`vendors/infinityledger/mef-hdag`) |
| `MeshHolo` / `PhaseSpaceWindow` (5D phase space) | ✅ under the `structure` feature (`crates/pse-traverse`) |
| Tensor math (`FieldTensorState`, `CouplingMatrix`) | ✅ under the `structure` feature (`crates/phase-matrix`) |

## Update — structural extraction completed (`structure` feature)

The HDAG / phase-space / mesh / tensor structures, previously left in the `pse-*` family and vendors,
have now been **faithfully extracted** into `lawish` behind a non-default **`structure`** cargo feature
(`cargo build -p lawish --features structure`). They are **re-exported as-is and NOT wired** into the
wish loop — the integration is the future-spec work and was deliberately not attempted.

Extracted crates (all compile + test standalone; closures need only `pse-types` + external crates):

| Crate (path) | Structure |
|---|---|
| `crates/pse-traverse` | `MeshHolo` (simplicial complex, Betti), `PhaseSpaceWindow` (5D), `FieldCube`, `DoFGraph`, `CollapsePlan` |
| `crates/phase-matrix` | `FieldTensorState`, `CouplingMatrix`, resonance cells / funnel graphs / convergence fields |
| `adapters/pse-adapter-il` | 5D `ResonanceTensor` + acyclic `HDAG` (coherence-gate acyclicity) |
| `vendors/infinityledger/mef-hdag` | 2D phase/time `HDAG` (explicit cycle check) |

To keep these standalone, two **cross-subsystem bridges were decoupled at the manifest level only**
(no source/logic changes; the gated code is left intact but uncompiled — see `MIGRATION_NOTES.md`):
1. `pse-traverse`: the `pse-commit` PSE-core bridge (`bridge.rs`) dropped from default features; optional
   `pse-core`/`pse-graph` deps removed. (Enabling `pse-commit` would pull ~18 PSE-core crates.)
2. `pse-adapter-il`: the optional `il-pipeline` mef-* deps removed; the HDAG/`ResonanceTensor` structure
   is independent of them.

> The three wiring gaps below are unchanged: the structures are now **present in lawish** but still
> **not woven** into the wish/CDK cubes — that remains the integration frontier.

## Bottom line

Every building block of the described "coil-and-mesh" machine **exists** in the repository, and the
forward measurement spine — wish → layered hypercube → full opacity → deterministic blueprint, plus the
one-way CDK condensation to a certified diamond — is **fully implemented and tested**. What does **not**
exist yet is the *unifying machine*: the Wish→tensor encoding, the WishCube→SystemCube materialization
link, the bidirectional two-cube wind/unwind realization, the weaving of the HDAG/phase-space into the
wish/CDK cubes, and the diamond→domain translation. These are the **aspirational integration frontier** —
present as vocabulary and as separate, individually-real subsystems, but not wired into the single
coil-and-mesh engine described.

> Non-goal: this document does not implement any of the missing links. It records the gap.
