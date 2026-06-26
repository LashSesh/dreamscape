# Example: wishlist_project

A project DoD with three wishes; `beta` is intentionally missing.

```sh
cargo run --bin lawish -- --wishlist examples/wishlist_project/dod.wishes --no-color examples/wishlist_project
```
Expect `realized 2/3` (alpha + f met, `a module beta` unmet) and exit code 1 —
the gauge fails closed until every wish is realized.
