# Example: cdk_condense_if_supported

The L9 condensation machine (CDK / CoffinDragger / Diamond) is report-only and
fail-closed. It builds under the `cdk` feature; the `kosmo-cdk` CLI demonstrates
a full fold over a sample SystemCube.

```sh
cargo run -p kosmo-cdk -- explain          # gate-by-gate fold, report-only
cargo run -p kosmo-cdk -- bind --json      # KBL binding of a real SystemCube
cargo run -p kosmo-cdk -- diamond          # press to a DiamondCubeCandidate (if irreducible)
```
A diamond without a QSR certificate, or a reducible/non-replayable core, is
refused — scores rank, gates decide.
