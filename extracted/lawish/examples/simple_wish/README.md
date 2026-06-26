# Example: simple_wish

A one-crate workspace with module `parser` and function `add`.

```sh
cargo run --bin lawish -- --wish "a module parser" --no-color examples/simple_wish
cargo run --bin lawish -- --wish "a function add" --layers --no-color examples/simple_wish
```
Expect `status REALIZED` — both facets are present.
