# wit-bindgen-scala

[![CI](https://github.com/scala-wasm/wit-bindgen-scala/actions/workflows/ci.yml/badge.svg)](https://github.com/scala-wasm/wit-bindgen-scala/actions/workflows/ci.yml)

Scala bindings generator for [WebAssembly Component Model](https://github.com/WebAssembly/component-model) targeting the [scala-wasm](https://github.com/tanishiking/scala-wasm) (a friendly fork of Scala.js).

```bash
$ cargo install wit-bindgen-scala --version 0.1.0-rc.1
```

## Usage

### Basic Command

```bash
wit-bindgen-scala path/to/your/wit --out-dir generated --base-package com.example
```

### Options

- `--base-package <PACKAGE>` - Base package for generated bindings (default: `componentmodel`)
- `--out-dir <DIR>` - Output directory for generated Scala files
- `--world <WORLD>` - Specify which world to generate bindings for (required if multiple worlds exist)

## Testing

```bash
cargo test
```

To run only selected runtime fixtures:

```bash
WIT_BINDGEN_SCALA_FIXTURES=numbers,strings cargo test
```

When cloning this repository, initialize submodules with `git submodule update --init --recursive`.
To refresh upstream fixtures, run `git submodule update --remote --recursive tests/upstream/wit-bindgen`.

Runtime fixture prerequisites:

- JDK 17
- sbt
- Rust with the `wasm32-wasip2` target
- `wit-bindgen-cli` 0.58.0 with Rust backend support
- Wasmtime 44.0.3 with `gc`, `function-references`, and `exceptions` support
- `wasm-tools`
- `wac` CLI (`wac-cli`)

## Generated Code Structure

### Package Organization

Generated code follows this structure:

- **Imports**: `{base-package}.{namespace}.{package-name}` (file: `{interface-name}.scala`)
- **Exports**: `{base-package}.exports.{namespace}.{package-name}` (file: `{interface-name}.scala`)

Example for `wasi:io/streams@0.2.0` with base package `com.example`:
- Import package: `com.example.wasi.io` (file: `streams.scala` containing `package object streams`)
- Export package: `com.example.exports.wasi.io` (file: `streams.scala` containing `trait Streams`)

### Type Mappings

| WIT Type | Scala Type |
|----------|------------|
| `bool` | `Boolean` |
| `s8` | `Byte` |
| `u8` | `scala.scalajs.wit.unsigned.UByte` |
| `s16` | `Short` |
| `u16` | `scala.scalajs.wit.unsigned.UShort` |
| `s32` | `Int` |
| `u32` | `scala.scalajs.wit.unsigned.UInt` |
| `s64` | `Long` |
| `u64` | `scala.scalajs.wit.unsigned.ULong` |
| `f32` | `Float` |
| `f64` | `Double` |
| `char` | `Char` |
| `string` | `String` |
| `list<T>` | `Array[T]` |
| `option<T>` | `java.util.Optional[T]` |
| `result<T, E>` | `scala.scalajs.wit.Result[T, E]` |
| `tuple<T1, T2>` | `scala.scalajs.wit.Tuple2[T1, T2]` |
| `record` | `case class` with `@WitRecord` |
| `variant` | `sealed trait` with `@WitVariant` |
| `enum` | `sealed trait` with case objects |
| `flags` | `case class` with bitwise operators |
| `resource` | imported `final class` with companion object |

## Generated Code Examples

### Records

WIT:
```wit
record point {
  x: s32,
  y: s32,
}
```

Generated Scala:
```scala
@scala.scalajs.wit.annotation.WitRecord
final case class Point(x: Int, y: Int)
```

### Variants

WIT:
```wit
variant result {
  ok(string),
  err(string),
}
```

Generated Scala:
```scala
@scala.scalajs.wit.annotation.WitVariant
sealed trait Result

object Result {
  final case class Ok(value: String) extends Result
  final case class Err(value: String) extends Result
}
```

### Enums

WIT:
```wit
enum color {
  red,
  green,
  blue,
}
```

Generated Scala:
```scala
@scala.scalajs.wit.annotation.WitVariant
sealed trait Color

object Color {
  case object Red extends Color
  case object Green extends Color
  case object Blue extends Color
}
```

### Flags

WIT:
```wit
flags permissions {
  read,
  write,
  execute,
}
```

Generated Scala:
```scala
@scala.scalajs.wit.annotation.WitFlags(8)
final case class Permissions(value: Int) {
  def |(other: Permissions): Permissions = Permissions(value | other.value)
  def &(other: Permissions): Permissions = Permissions(value & other.value)
  def ^(other: Permissions): Permissions = Permissions(value ^ other.value)
  def unary_~ : Permissions = Permissions(~value)
  def contains(other: Permissions): Boolean = (value & other.value) == other.value
}

object Permissions {
  val read = Permissions(1 << 0)
  val write = Permissions(1 << 1)
  val execute = Permissions(1 << 2)
}
```

### Import Functions

WIT:
```wit
import add: func(a: s32, b: s32) -> s32;
```

Generated Scala (within a package object):
```scala
@scala.scalajs.wit.annotation.WitImport("example:math/operations", "add")
def add(a: Int, b: Int): Int = scala.scalajs.wit.native
```

### Export Functions

WIT:
```wit
export multiply: func(a: s32, b: s32) -> s32;
```

Generated Scala (within a trait):
```scala
@scala.scalajs.wit.annotation.WitExport("example:math/operations", "multiply")
def multiply(a: Int, b: Int): Int
```

### Resources (Import)

Imported resources are represented as opaque final classes. Instance methods live on the
class, constructors and static methods live on the companion object, and every resource
class includes a `close()` method for dropping the handle.

WIT:
```wit
resource counter {
  constructor(initial: s32);
  increment: func();
  value: func() -> s32;
}
```

Generated Scala:
```scala
@scala.scalajs.wit.annotation.WitResourceImport("example:state/counter", "counter")
final class Counter private () extends Object {
  @scala.scalajs.wit.annotation.WitResourceMethod("increment")
  def increment(): Unit = scala.scalajs.wit.native

  @scala.scalajs.wit.annotation.WitResourceMethod("value")
  def value(): Int = scala.scalajs.wit.native

  @scala.scalajs.wit.annotation.WitResourceDrop
  def close(): Unit = scala.scalajs.wit.native
}

object Counter {
  @scala.scalajs.wit.annotation.WitResourceConstructor
  def apply(initial: Int): Counter = scala.scalajs.wit.native
}
```

### Resources (Export)

Scala bindings currently do not support exporting resources due to Wasm Component Model limitation with WasmGC. Resources can only be imported.

## Naming Conventions

The generator applies these naming transformations:

- **Types** (records, variants, enums, resources): `kebab-case` → `PascalCase`
  - `my-type` → `MyType`
- **Functions and parameters**: `kebab-case` → `camelCase`
  - `my-function` → `myFunction`
- **Packages**: `kebab-case` → `snake_case`
  - `my-package` → `my_package`
- **Scala keywords**: Wrapped in backticks
  - `type` → `` `type` ``

## Limitations

- Resource exports are not supported (resources can only be imported)
- Futures and streams are not yet supported
