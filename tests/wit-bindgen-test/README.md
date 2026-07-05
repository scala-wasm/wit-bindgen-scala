# wit-bindgen-test coverage

Runtime tests are based on upstream fixtures in
`tests/upstream/wit-bindgen/tests/runtime`. A fixture is executed here only when
there is a matching overlay directory in `tests/wit-bindgen-test/overlays`.

For most overlays, both cross-language directions run:

- `rust-to-scala`: upstream `runner.rs` with overlay `test.scala`
- `scala-to-rust`: overlay `runner.scala` with upstream `test.rs`

Resource overlays may run only `scala-to-rust`. Scala resource exports are not
supported yet, so those fixtures intentionally omit `test.scala`.

## Upstream Fixtures Not Run

| Upstream fixture | Reason |
| --- | --- |
| `gated-features` | TODO: custom hook needs a clean way to receive `//@ args` such as `--features y` from `wit-bindgen-test`. |
| `resource_aggregates` | Blocked after codegen. Scala runner compiles, but `wasm-tools component new` rejects the component with `type mismatch: expected i32 but nothing on stack`. |

## Partially Run Fixtures

These fixtures run only `scala-to-rust`:

- `common-types`: upstream runner is C++ only.
- `resource-borrow`: requires resource exports.
- `package-with-version`: requires versioned resource exports.
- `resource-import-and-export`: requires resource exports across intermediate/leaf composition.
- `resource_alias`: requires aliased resource exports.
- `resource_alias_redux`: requires aliased resources and list-of-resource exports.
- `resource_borrow_in_record`: requires resource exports in borrowed record fields.
- `resource_floats`: requires resource exports across leaf/intermediate composition.
- `resource_with_lists`: requires resource exports with list-taking methods.
- `resources`: requires multiple resource exports.
