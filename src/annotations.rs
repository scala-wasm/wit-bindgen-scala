/// Helper functions for generating Scala Component Model annotations.
///
/// This module provides utilities for creating annotations that bridge
/// Scala code with the WebAssembly Component Model via scala-wasm runtime.
use std::fmt::Write as _;

const ANNOTATION_PKG: &str = "scala.scalajs.wit.annotation";

/// Parse a WIT interface id into a Scala `WitScope` expression.
///
/// Accepted forms:
/// - `` / `$root` → `WitScope.root` (world body)
/// - `name` → `WitScope.inline("name")` (`import name: interface { ... }`)
/// - `ns:pkg/iface[@version]` → package interface scope
pub fn wit_scope_expr(namespace: &str) -> String {
    if namespace.is_empty() || namespace == "$root" {
        return format!("{}.WitScope.root", ANNOTATION_PKG);
    }

    // Expected forms:
    //   ns:pkg/iface@version
    //   ns:pkg/iface
    //   name  (inline world interface)
    let (ns_pkg, iface_and_version) = match namespace.split_once('/') {
        Some(parts) => parts,
        None => {
            return format!("{}.WitScope.inline(\"{}\")", ANNOTATION_PKG, namespace);
        }
    };

    let (ns, pkg) = match ns_pkg.split_once(':') {
        Some(parts) => parts,
        // `foo/bar` without a package namespace — treat as inline world key.
        None => return format!("{}.WitScope.inline(\"{}\")", ANNOTATION_PKG, namespace),
    };

    let (iface, version) = match iface_and_version.split_once('@') {
        Some((iface, version)) => (iface, Some(version)),
        None => (iface_and_version, None),
    };

    match version {
        Some(version) => format!(
            "{}.WitScope(\"{}\", \"{}\", \"{}\", \"{}\")",
            ANNOTATION_PKG, ns, pkg, iface, version
        ),
        None => format!(
            "{}.WitScope.unversioned(\"{}\", \"{}\", \"{}\")",
            ANNOTATION_PKG, ns, pkg, iface
        ),
    }
}

/// Generate `@WitName("...")` for fields, cases, and parameters.
pub fn wit_name(name: &str) -> String {
    format!("@{}.WitName(\"{}\")", ANNOTATION_PKG, name)
}

/// Generate `@WitImport(scope, name)`.
pub fn component_import(namespace: &str, name: &str) -> String {
    format!(
        "@{}.WitImport({}, \"{}\")",
        ANNOTATION_PKG,
        wit_scope_expr(namespace),
        name
    )
}

/// Generate `@WitExport(scope, name)`.
pub fn component_export(namespace: &str, name: &str) -> String {
    format!(
        "@{}.WitExport({}, \"{}\")",
        ANNOTATION_PKG,
        wit_scope_expr(namespace),
        name
    )
}

/// Generate `@WitAlias(scope, name)`.
pub fn component_alias(namespace: &str, wit_name: &str) -> String {
    format!(
        "@{}.WitAlias({}, \"{}\")",
        ANNOTATION_PKG,
        wit_scope_expr(namespace),
        wit_name
    )
}

/// Generate `@WitRecord(scope, name)`.
pub fn component_record(namespace: &str, wit_name: &str) -> String {
    format!(
        "@{}.WitRecord({}, \"{}\")",
        ANNOTATION_PKG,
        wit_scope_expr(namespace),
        wit_name
    )
}

/// Generate `@WitVariant(scope, name)`.
pub fn component_variant(namespace: &str, wit_name: &str) -> String {
    format!(
        "@{}.WitVariant({}, \"{}\")",
        ANNOTATION_PKG,
        wit_scope_expr(namespace),
        wit_name
    )
}

/// Generate `@WitEnum(scope, name)`.
pub fn component_enum(namespace: &str, wit_name: &str) -> String {
    format!(
        "@{}.WitEnum({}, \"{}\")",
        ANNOTATION_PKG,
        wit_scope_expr(namespace),
        wit_name
    )
}

/// Generate `@WitFlags(scope, name, Array(...))`.
pub fn component_flags(namespace: &str, wit_name: &str, flag_names: &[String]) -> String {
    let mut flags = String::new();
    for (i, flag) in flag_names.iter().enumerate() {
        if i > 0 {
            flags.push_str(", ");
        }
        write!(&mut flags, "\"{}\"", flag).unwrap();
    }
    format!(
        "@{}.WitFlags({}, \"{}\", Array({}))",
        ANNOTATION_PKG,
        wit_scope_expr(namespace),
        wit_name,
        flags
    )
}

/// Generate `@WitResourceImport(scope, name)`.
pub fn component_resource_import(namespace: &str, name: &str) -> String {
    format!(
        "@{}.WitResourceImport({}, \"{}\")",
        ANNOTATION_PKG,
        wit_scope_expr(namespace),
        name
    )
}

/// Generate `@WitResourceConstructor`.
pub fn component_resource_constructor() -> &'static str {
    "@scala.scalajs.wit.annotation.WitResourceConstructor"
}

/// Generate `@WitResourceMethod("...")`.
pub fn component_resource_method(name: &str) -> String {
    format!("@{}.WitResourceMethod(\"{}\")", ANNOTATION_PKG, name)
}

/// Generate `@WitResourceStaticMethod("...")`.
pub fn component_resource_static_method(name: &str) -> String {
    format!("@{}.WitResourceStaticMethod(\"{}\")", ANNOTATION_PKG, name)
}

/// Generate `@WitResourceDrop`.
pub fn component_resource_drop() -> &'static str {
    "@scala.scalajs.wit.annotation.WitResourceDrop"
}

/// Generate the `= scala.scalajs.wit.native` marker for imported functions.
pub fn native_marker() -> &'static str {
    "scala.scalajs.wit.native"
}

fn write_params(output: &mut String, params: &[(String, String, String)]) {
    for (i, (wit_param_name, scala_param_name, param_type)) in params.iter().enumerate() {
        if i > 0 {
            write!(output, ", ").unwrap();
        }
        write!(
            output,
            "{} {}: {}",
            wit_name(wit_param_name),
            scala_param_name,
            param_type
        )
        .unwrap();
    }
}

/// Generate a complete import function signature with annotation.
///
/// `params` is `(wit_name, scala_name, scala_type)`.
pub fn import_function(
    namespace: &str,
    wit_func_name: &str,
    scala_name: &str,
    params: &[(String, String, String)],
    return_type: Option<&str>,
    docs: &str,
) -> String {
    let mut output = String::new();

    if !docs.is_empty() {
        write!(&mut output, "{}", docs).unwrap();
    }

    writeln!(
        &mut output,
        "{}",
        component_import(namespace, wit_func_name)
    )
    .unwrap();
    write!(&mut output, "def {}(", scala_name).unwrap();
    write_params(&mut output, params);
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
/// Used for stubs printed for the user to implement.
/// `params` is `(wit_name, scala_name, scala_type)`.
pub fn export_function(
    namespace: &str,
    wit_func_name: &str,
    scala_name: &str,
    params: &[(String, String, String)],
    return_type: Option<&str>,
    docs: &str,
) -> String {
    let mut output = String::new();

    if !docs.is_empty() {
        write!(&mut output, "{}", docs).unwrap();
    }

    writeln!(
        &mut output,
        "{}",
        component_export(namespace, wit_func_name)
    )
    .unwrap();
    write!(&mut output, "def {}(", scala_name).unwrap();
    write_params(&mut output, params);
    write!(&mut output, ")").unwrap();

    if let Some(ret) = return_type {
        write!(&mut output, ": {}", ret).unwrap();
    } else {
        write!(&mut output, ": Unit").unwrap();
    }

    writeln!(&mut output, " = ???").unwrap();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wit_scope_root() {
        assert_eq!(
            wit_scope_expr("$root"),
            "scala.scalajs.wit.annotation.WitScope.root"
        );
        assert_eq!(
            wit_scope_expr(""),
            "scala.scalajs.wit.annotation.WitScope.root"
        );
    }

    #[test]
    fn wit_scope_versioned() {
        assert_eq!(
            wit_scope_expr("wasi:io/streams@0.2.0"),
            "scala.scalajs.wit.annotation.WitScope(\"wasi\", \"io\", \"streams\", \"0.2.0\")"
        );
    }

    #[test]
    fn wit_scope_unversioned() {
        assert_eq!(
            wit_scope_expr("test:records/to-test"),
            "scala.scalajs.wit.annotation.WitScope.unversioned(\"test\", \"records\", \"to-test\")"
        );
    }

    #[test]
    fn wit_scope_inline() {
        assert_eq!(
            wit_scope_expr("exports"),
            "scala.scalajs.wit.annotation.WitScope.inline(\"exports\")"
        );
    }
}
