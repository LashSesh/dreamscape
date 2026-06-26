//! # lawish — the Wish-to-System engine, as a stable public API
//!
//! `lawish` is an isolated, **repo-agnostic** information-technological core: an
//! infrastructural exoskeleton that any AI or agentic framework can integrate
//! with, so a *wish* can be driven from its starting point to **100%
//! Definition-of-Done fulfillment** without prompt-drift or context-loss across
//! that distance.
//!
//! The machine is a closed loop:
//!
//! ```text
//!   parse a wish ──▶ observe a workspace ──▶ render the wish as a layered
//!   hypercube ──▶ measure realization ──▶ report missing facets + closure
//!   hints ──▶ detect regressions / counterfeit progress ──▶ (policy-gated)
//!   scaffold & apply ──▶ surface architecture defects ──▶ build SystemCube
//!   diagnostics ──▶ (optional) condense to a certified diamond core.
//! ```
//!
//! ## Invariants (preserved exactly from the source machine)
//!
//! * **Scores may rank, but gates decide.** A failed gate is a hard reject that
//!   no score can override.
//! * **Fail-closed.** Unobserved facets count as *unmet*; the default
//!   [`PolicyProfile`] is report-only and never writes host files.
//! * **Evidence / trace / replay.** Durable artifacts are content-addressed and
//!   carry an evidence bundle and replay status; identity is deterministic (no
//!   wall-clock, local path, or randomness in artifact IDs).
//! * **Residue is visible.** Anything non-integrable is surfaced, never
//!   swallowed.
//!
//! ## Layout
//!
//! The engine crates retain their original `kosmo-*` names (extraction
//! fidelity). They are re-exported here under stable module aliases plus a
//! [`prelude`]. The lawish-branded CLI lives in the `lawish-cli` package
//! (binary `lawish`).

// ── Engine crates, re-exported under stable module aliases ──────────────────
pub use kosmo_agent as agent;
pub use kosmo_core as core;
pub use kosmo_hyphae as observe_backends;
pub use kosmo_intent as intent;
pub use kosmo_materialize as materialize;
pub use kosmo_pipeline as pipeline;
pub use kosmo_pse_bridge as bridge;
pub use kosmo_synthesizer as synthesizer;
pub use kosmo_systemcube as systemcube;

#[cfg(feature = "cdk")]
pub use kosmo_cdk_core as cdk_core;
#[cfg(feature = "cdk")]
pub use kosmo_coffindragger as coffindragger;
#[cfg(feature = "cdk")]
pub use kosmo_wings as wings;

// ── Stable verbs of the machine (thin, faithful wrappers) ───────────────────

/// Parse plain prose into a content-addressed [`Wish`](kosmo_core::Wish).
///
/// Deterministic and offline. The returned wish is identical for identical
/// `(prose, policy_id, evidence_bundle_id)` inputs.
pub fn parse_wish(
    prose: &str,
    policy_id: kosmo_core::Digest,
    evidence_bundle_id: kosmo_core::Digest,
) -> kosmo_core::Wish {
    kosmo_intent::compile_wish(prose, policy_id, evidence_bundle_id)
}

/// Observe a workspace into an [`ObservedTopology`](kosmo_core::ObservedTopology)
/// (read-only; runs `cargo metadata` once). For deeper, polyglot, validated, or
/// runtime observation use [`intent::observe_workspace_deep`](kosmo_intent::observe_workspace_deep)
/// and friends, re-exported here.
pub use kosmo_intent::{
    observe_workspace, observe_workspace_deep, observe_workspace_runtime,
    observe_workspace_validated,
};

/// Measure a wish against an observed topology (the flat realization distance).
pub use kosmo_core::assess_wish;

/// Render a wish as a layered [`WishCube`](kosmo_core::WishCube) hypercube —
/// existence / shape / wiring / verified / live strata with per-layer opacity
/// and structural solidity.
pub fn render_wish_cube(
    wish: &kosmo_core::Wish,
    observed: &kosmo_core::ObservedTopology,
    evidence_bundle_id: kosmo_core::Digest,
) -> kosmo_core::WishCube {
    kosmo_core::assess_wish_layered(wish, observed, evidence_bundle_id)
}

/// Fold a descent film (a sequence of [`WishCube`](kosmo_core::WishCube)s plus a
/// [`LayeredConvergenceTrace`](kosmo_core::LayeredConvergenceTrace)) into a
/// per-stratum [`StagedClosureReport`](kosmo_core::StagedClosureReport): the
/// Solve→Gate→Coagula closure plan toward Definition-of-Done. Fail-closed:
/// non-contractive or anomalous strata are *Fractured*, never silently solid.
pub fn plan_closure(
    cubes: &[kosmo_core::WishCube],
    trace: &kosmo_core::LayeredConvergenceTrace,
    evidence_bundle_id: kosmo_core::Digest,
) -> kosmo_core::StagedClosureReport {
    kosmo_core::StagedClosureReport::from_descent(cubes, trace, evidence_bundle_id)
}

/// Build the topological [`SystemCube`](kosmo_systemcube::SystemCube) blueprint
/// (D-density, contradiction energy, compatibility profile). High density never
/// authorizes materialization — gates still decide (CROSS-010).
pub fn build_system_cube(
    host_snapshot_id: kosmo_core::Digest,
    run: &kosmo_core::RunDescriptor,
    policy: &kosmo_core::PolicyProfile,
    units: Vec<kosmo_systemcube::BlueprintUnit>,
) -> kosmo_systemcube::SystemCube {
    kosmo_systemcube::SystemCube::new(host_snapshot_id, run, policy, units)
}

/// Surface architecture defects between two facets: `depends_on(g, f)` is the
/// same foundations-first relation the blueprint draws.
pub use kosmo_core::depends_on;

/// Deterministic, write-free scaffolding of a missing facet into a set of
/// [`FileChange`](kosmo_synthesizer::FileChange)s. Guided repair steers toward
/// renaming near-misses instead of scaffolding naming-drift duplicates.
pub use kosmo_synthesizer::FacetScaffolder;

/// Policy-gated host application. [`Materializer::materialize`](kosmo_materialize::Materializer::materialize)
/// is the only path that can write to the host, and only under an
/// operator-approved [`PolicyProfile`](kosmo_core::PolicyProfile); the default
/// report-only profile yields a *skipped* report with zero bytes written.
pub use kosmo_materialize::Materializer;

/// Condense an observed system into a certified, irreducible diamond core
/// (CDK / CoffinDragger). The pipeline is `bind_systemcube` → `close_stack` →
/// `press_diamond`; every step is report-only and fail-closed (a diamond
/// without a QSR certificate, or a reducible/non-replayable core, is refused).
/// Available under the `cdk` feature.
#[cfg(feature = "cdk")]
pub use kosmo_coffindragger::{bind_systemcube, close_stack, press_diamond, DiamondCubeCandidate};

/// The most-used items, for `use lawish::prelude::*;`.
pub mod prelude {
    pub use kosmo_core::{
        assess_wish, assess_wish_layered, depends_on, Digest, EvidenceBundle, GateResult,
        ImplementationMode, LayeredConvergenceTrace, ObservedTopology, PolicyProfile,
        RenderAnomaly, ReplayStatus, RunDescriptor, StagedClosureReport, StratumClosure, Wish,
        WishAssessment, WishClosureStatus, WishConvergenceTrace, WishCube, WishFacet,
        WishFacetKind, WishLayer, Q16,
    };
    pub use kosmo_intent::{
        compile_wish, observe_workspace, observe_workspace_deep, observe_workspace_validated,
    };
    pub use kosmo_materialize::Materializer;
    pub use kosmo_synthesizer::FacetScaffolder;
    pub use kosmo_systemcube::{BlueprintUnit, SystemCube};

    pub use crate::{build_system_cube, parse_wish, plan_closure, render_wish_cube};
}

#[cfg(test)]
mod tests {
    use super::*;
    use kosmo_core::Digest;

    /// End-to-end smoke: a wish parses, and measuring it against an empty
    /// topology fails closed (every facet unmet), never silently realized.
    #[test]
    fn wish_against_empty_topology_fails_closed() {
        let wish = parse_wish("crate api", Digest::ZERO, Digest::ZERO);
        let observed = kosmo_core::ObservedTopology::default();
        let assessment = assess_wish(&wish, &observed, Digest::ZERO);
        assert!(
            !matches!(assessment.status, kosmo_core::WishClosureStatus::Realized),
            "an unobserved wish must not be reported realized"
        );
    }

    /// The layered render produces a cube; closure of an empty descent is
    /// fail-closed (not fully coagulated).
    #[test]
    fn layered_render_and_empty_closure_are_failclosed() {
        let wish = parse_wish("crate api", Digest::ZERO, Digest::ZERO);
        let observed = kosmo_core::ObservedTopology::default();
        let cube = render_wish_cube(&wish, &observed, Digest::ZERO);
        let cubes = std::slice::from_ref(&cube);
        let trace = kosmo_core::LayeredConvergenceTrace::from_cubes(cubes, Digest::ZERO);
        let report = plan_closure(cubes, &trace, Digest::ZERO);
        assert!(
            !report.fully_coagulated,
            "an empty/unmet descent must not be fully coagulated"
        );
    }
}
