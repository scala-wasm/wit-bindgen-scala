/// Helper functions for generating Scala Component Model annotations.
///
/// This module provides utilities for creating annotations that bridge
/// Scala code with the WebAssembly Component Model via scala-wasm runtime.
use std::fmt::Write as _;

/// Generate @ComponentImport annotation for importing functions.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitImport("wasi:io/streams@0.2.0", "read")
/// def read(stream: InputStream, len: Long): scala.scalajs.wit.Result[Array[Byte], StreamError] = scala.scalajs.wit.native
/// ```
pub fn component_import(namespace: &str, name: &str) -> String {
    format!(
        "@scala.scalajs.wit.annotation.WitImport(\"{}\", \"{}\")",
        namespace, name
    )
}

/// Generate @ComponentExport annotation for exporting functions.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitExport("wasi:cli/run@0.2.0", "run")
/// def run(): Int
/// ```
pub fn component_export(namespace: &str, name: &str) -> String {
    format!(
        "@scala.scalajs.wit.annotation.WitExport(\"{}\", \"{}\")",
        namespace, name
    )
}

/// Generate @ComponentRecord annotation for record types.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitRecord
/// final case class Point(x: Int, y: Int)
/// ```
pub fn component_record() -> &'static str {
    "@scala.scalajs.wit.annotation.WitRecord"
}

/// Generate @ComponentVariant annotation for variant and enum types.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitVariant
/// sealed trait Result
/// object Result {
///   final case class Ok(value: String) extends Result
///   final case class Err(value: String) extends Result
/// }
/// ```
pub fn component_variant() -> &'static str {
    "@scala.scalajs.wit.annotation.WitVariant"
}

/// Generate @ComponentFlags annotation for flags types.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitFlags(8)
/// final case class Permissions(value: Int) { ... }
/// ```
pub fn component_flags(num_flags: usize) -> String {
    format!("@scala.scalajs.wit.annotation.WitFlags({})", num_flags)
}

/// Generate @ComponentResourceImport annotation for importing resource types.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitResourceImport("wasi:io/streams@0.2.0", "input-stream")
/// final class InputStream private () extends Object with AutoCloseable {
///   @WitResourceMethod("read")
///   def read(len: Long): scala.scalajs.wit.Result[Array[Byte], StreamError] = scala.scalajs.wit.native
/// }
/// ```
pub fn component_resource_import(namespace: &str, name: &str) -> String {
    format!(
        "@scala.scalajs.wit.annotation.WitResourceImport(\"{}\", \"{}\")",
        namespace, name
    )
}

/// Generate @ComponentResourceConstructor annotation for resource constructors.
///
/// # Example
/// ```scala
/// object InputStream {
///   @scala.scalajs.wit.annotation.WitResourceConstructor
///   def apply(): InputStream = scala.scalajs.wit.native
/// }
/// ```
pub fn component_resource_constructor() -> &'static str {
    "@scala.scalajs.wit.annotation.WitResourceConstructor"
}

/// Generate @ComponentResourceMethod annotation for resource instance methods.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitResourceMethod("read")
/// def read(len: Long): scala.scalajs.wit.Result[Array[Byte], StreamError] = scala.scalajs.wit.native
/// ```
pub fn component_resource_method(name: &str) -> String {
    format!(
        "@scala.scalajs.wit.annotation.WitResourceMethod(\"{}\")",
        name
    )
}

/// Generate @ComponentResourceStaticMethod annotation for resource static methods.
///
/// # Example
/// ```scala
/// object InputStream {
///   @scala.scalajs.wit.annotation.WitResourceStaticMethod("merge")
///   def merge(a: InputStream, b: InputStream): InputStream = scala.scalajs.wit.native
/// }
/// ```
pub fn component_resource_static_method(name: &str) -> String {
    format!(
        "@scala.scalajs.wit.annotation.WitResourceStaticMethod(\"{}\")",
        name
    )
}

/// Generate @ComponentResourceDrop annotation for resource destructors.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitResourceDrop
/// def close(): Unit = scala.scalajs.wit.native
/// ```
pub fn component_resource_drop() -> &'static str {
    "@scala.scalajs.wit.annotation.WitResourceDrop"
}

/// Generate @ComponentExportInterface annotation for export traits.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitExportInterface
/// trait Handler {
///   @WitExport("namespace", "function")
///   def method(): Unit
/// }
/// ```
pub fn component_export_interface() -> &'static str {
    "@scala.scalajs.wit.annotation.WitExportInterface"
}

/// Generate the `= scala.scalajs.wit.native` marker for imported functions.
///
/// This indicates that the function implementation is provided by the runtime.
pub fn native_marker() -> &'static str {
    "scala.scalajs.wit.native"
}

/// Generate a complete import function signature with annotation.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitImport("wasi:io/streams@0.2.0", "read")
/// def read(stream: InputStream, len: Long): scala.scalajs.wit.Result[Array[Byte], StreamError] = scala.scalajs.wit.native
/// ```
pub fn import_function(
    namespace: &str,
    wit_name: &str,
    scala_name: &str,
    params: &[(String, String)], // (name, type)
    return_type: Option<&str>,
    docs: &str,
) -> String {
    let mut output = String::new();

    // Add scaladoc if present
    if !docs.is_empty() {
        write!(&mut output, "{}", docs).unwrap();
    }

    writeln!(&mut output, "{}", component_import(namespace, wit_name)).unwrap();
    write!(&mut output, "def {}(", scala_name).unwrap();

    for (i, (param_name, param_type)) in params.iter().enumerate() {
        if i > 0 {
            write!(&mut output, ", ").unwrap();
        }
        write!(&mut output, "{}: {}", param_name, param_type).unwrap();
    }

    write!(&mut output, ")").unwrap();

    if let Some(ret) = return_type {
        write!(&mut output, ": {}", ret).unwrap();
    } else {
        write!(&mut output, ": Unit").unwrap();
    }

    writeln!(&mut output, " = {}", native_marker()).unwrap();

    output
}

/// Generate a complete export function signature with annotation.
///
/// # Example
/// ```scala
/// @scala.scalajs.wit.annotation.WitExport("wasi:cli/run@0.2.0", "run")
/// def run(): Int
/// ```
pub fn export_function(
    namespace: &str,
    wit_name: &str,
    scala_name: &str,
    params: &[(String, String)], // (name, type)
    return_type: Option<&str>,
    docs: &str,
) -> String {
    let mut output = String::new();

    // Add scaladoc if present
    if !docs.is_empty() {
        write!(&mut output, "{}", docs).unwrap();
    }

    writeln!(&mut output, "{}", component_export(namespace, wit_name)).unwrap();
    write!(&mut output, "def {}(", scala_name).unwrap();

    for (i, (param_name, param_type)) in params.iter().enumerate() {
        if i > 0 {
            write!(&mut output, ", ").unwrap();
        }
        write!(&mut output, "{}: {}", param_name, param_type).unwrap();
    }

    write!(&mut output, ")").unwrap();

    if let Some(ret) = return_type {
        write!(&mut output, ": {}", ret).unwrap();
    } else {
        write!(&mut output, ": Unit").unwrap();
    }

    writeln!(&mut output).unwrap();

    output
}
