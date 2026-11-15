# wit-bindgen-test coverage

Runtime tests are based on upstream fixtures in
`tests/upstream/wit-bindgen/tests/runtime`. A fixture is executed here only when
there is a matching overlay directory in `tests/wit-bindgen-test/overlays`.

For most overlays, both cross-language directions run:

- `rust-to-scala`: upstream `runner.rs` with the overlay's Scala components
  (`test.scala`, or e.g. `resources.scala`), each one replacing the upstream
  Rust component of the same name
- `scala-to-rust`: overlay `runner.scala` with upstream `test.rs`

The overlays that export resources (`resource-borrow`, `resources`,
`resource_floats`, `resource_borrow_in_record`) need a scala-wasm with
`@WitResourceExport` (scala-wasm/scala-wasm#274).

## Generator Arguments

A Scala overlay can pass extra arguments to wit-bindgen-scala with a line such as:

```scala
// wit-bindgen-scala: --export-impl-package=witbindgentest
```

The hook then regenerates the bindings with them before it compiles the
overlay. The `numbers`, `records` and `many-arguments` overlays (and the
`numbers` runner) use it: they implement their exports as plain objects, and the
generated `@WitExport` forwarders call them. The other overlays write the
`@WitExport` methods by hand.

## Upstream Fixtures Not Run

| Upstream fixture | Reason |
| --- | --- |
| `gated-features` | TODO: custom hook needs a clean way to receive `//@ args` such as `--features y` from `wit-bindgen-test`. |
| `resource_aggregates` | Blocked after codegen. Scala runner compiles, but `wasm-tools component new` rejects the component with `type mismatch: expected i32 but nothing on stack`. |

## Partially Run Fixtures

These fixtures run only `scala-to-rust`:

- `common-types`: upstream runner is C++ only.
- `package-with-version`, `resource_alias`, `resource_alias_redux`: their
  resource exports have no Scala overlay yet.
- `resource-import-and-export`, `resource_with_lists`: the component imports and
  exports the same interface. scala-wasm identifies a resource type by its WIT
  scope and name, so the imported and the exported resource collide.
