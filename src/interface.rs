/// Interface generation for WebAssembly Component Model interfaces.
///
/// Each WIT interface generates one Scala package object containing:
/// - Type definitions (records, variants, enums, flags)
/// - Function declarations (imports/exports)
/// - Resource definitions (imports/exports)
use crate::{ScalaContext, annotations, resource};
use std::fmt::Write as _;
use wit_bindgen_core::wit_parser::*;

/// Generate an interface file (import or export).
pub fn render_interface(
    ctx: &mut ScalaContext,
    resolve: &Resolve,
    interface_id: InterfaceId,
    interface_name: &str,
    package_namespace: &str,
    annotation_namespace: &str,
    is_import: bool,
) -> String {
    let interface = &resolve.interfaces[interface_id];

    // Set current interface context for type qualification
    ctx.set_current_interface(Some(interface_id));

    let package_name = ctx.to_snake_case(interface_name);
    let type_name = ctx.to_pascal_case(interface_name);
    let mut output = String::new();

    // Generate package declaration
    let package_path = get_package_path(ctx, package_namespace, is_import);
    writeln!(&mut output, "package {}", package_path).unwrap();
    writeln!(&mut output).unwrap();

    // Use package object for both imports and exports
    writeln!(&mut output, "package object {} {{", package_name).unwrap();
    writeln!(&mut output).unwrap();

    // Generate type definitions
    let mut generated_types = Vec::new();
    for (type_name, type_id) in &interface.types {
        let typedef = ctx.render_typedef(resolve, *type_id);
        if !typedef.is_empty() && !typedef.starts_with("//") {
            generated_types.push((type_name.clone(), typedef));
        }
    }

    if !generated_types.is_empty() {
        writeln!(&mut output, "  // Type definitions").unwrap();
        for (_name, typedef) in &generated_types {
            for line in typedef.lines() {
                if line.is_empty() {
                    writeln!(&mut output).unwrap();
                } else {
                    writeln!(&mut output, "  {}", line).unwrap();
                }
            }
            writeln!(&mut output).unwrap();
        }
    }

    // Generate resources (import only - Scala cannot export resources)
    let mut generated_resources = Vec::new();
    for (resource_name, resource_id) in &interface.types {
        let resource_type = &resolve.types[*resource_id];
        if matches!(resource_type.kind, TypeDefKind::Resource) {
            if is_import {
                let resource_code = resource::render_imported_resource(
                    ctx,
                    resolve,
                    *resource_id,
                    annotation_namespace,
                );
                generated_resources.push((resource_name.clone(), resource_code));
            } else {
                // Scala cannot export resources
                panic!(
                    "Scala bindings do not support exporting resources. Resource '{}' in interface '{}' cannot be exported.",
                    resource_name, interface_name
                );
            }
        }
    }

    if !generated_resources.is_empty() {
        writeln!(&mut output, "  // Resources").unwrap();
        for (_name, resource_code) in &generated_resources {
            for line in resource_code.lines() {
                if line.is_empty() {
                    writeln!(&mut output).unwrap();
                } else {
                    writeln!(&mut output, "  {}", line).unwrap();
                }
            }
            writeln!(&mut output).unwrap();
        }
    }

    // Generate functions (excluding resource methods which are handled above)
    let mut generated_functions = Vec::new();
    for (func_name, func) in &interface.functions {
        // Skip resource-related functions (they're handled in resource generation)
        if matches!(
            func.kind,
            FunctionKind::Method(_) | FunctionKind::Constructor(_) | FunctionKind::Static(_)
        ) {
            continue;
        }

        let func_code = ctx.render_function(resolve, func, is_import, annotation_namespace);
        generated_functions.push((func_name.clone(), func_code));
    }

    // For imports: generate functions inside package object, then close it
    if is_import && !generated_functions.is_empty() {
        writeln!(&mut output, "  // Functions").unwrap();
        for (_name, func_code) in &generated_functions {
            for line in func_code.lines() {
                if line.is_empty() {
                    writeln!(&mut output).unwrap();
                } else {
                    writeln!(&mut output, "  {}", line).unwrap();
                }
            }
            writeln!(&mut output).unwrap();
        }
    }

    // Close the package object
    writeln!(&mut output, "}}").unwrap();

    // For exports: create a trait at package level to hold functions
    if !is_import && !generated_functions.is_empty() {
        writeln!(&mut output).unwrap();
        writeln!(&mut output, "// Export interface").unwrap();
        writeln!(&mut output, "{}", annotations::component_export_interface()).unwrap();
        writeln!(&mut output, "trait {} {{", type_name).unwrap();
        writeln!(&mut output).unwrap();

        // Import types from package object if there are any type definitions
        if !generated_types.is_empty() {
            writeln!(&mut output, "  import {}._", package_name).unwrap();
            writeln!(&mut output).unwrap();
        }

        // Generate functions inside trait (2-space indentation)
        writeln!(&mut output, "  // Functions").unwrap();
        for (_name, func_code) in &generated_functions {
            for line in func_code.lines() {
                if line.is_empty() {
                    writeln!(&mut output).unwrap();
                } else {
                    writeln!(&mut output, "  {}", line).unwrap();
                }
            }
            writeln!(&mut output).unwrap();
        }

        writeln!(&mut output, "}}").unwrap(); // Close trait
    }

    output
}

/// Generate an export file with only the trait (no types or package object).
///
/// Used when an interface is remapped via `--with` to an external package.
/// Types resolve to the external path; only the export trait with function
/// signatures is generated.
pub fn render_export_trait_only(
    ctx: &mut ScalaContext,
    resolve: &Resolve,
    interface_id: InterfaceId,
    interface_name: &str,
    package_namespace: &str,
    annotation_namespace: &str,
) -> String {
    let interface = &resolve.interfaces[interface_id];

    // Set current interface and force external resolution so own types use with_map
    ctx.set_current_interface(Some(interface_id));
    ctx.set_force_external_for_current(true);

    let type_name = ctx.to_pascal_case(interface_name);
    let mut output = String::new();

    // Package declaration (export path)
    let package_path = get_package_path(ctx, package_namespace, false);
    writeln!(&mut output, "package {}", package_path).unwrap();
    writeln!(&mut output).unwrap();

    // Export trait only — no package object, no type definitions
    writeln!(&mut output, "// Export interface").unwrap();
    writeln!(&mut output, "{}", annotations::component_export_interface()).unwrap();
    writeln!(&mut output, "trait {} {{", type_name).unwrap();
    writeln!(&mut output).unwrap();

    // Generate export functions
    let mut generated_functions = Vec::new();
    for (func_name, func) in &interface.functions {
        if matches!(
            func.kind,
            FunctionKind::Method(_) | FunctionKind::Constructor(_) | FunctionKind::Static(_)
        ) {
            continue;
        }

        let func_code = ctx.render_function(resolve, func, false, annotation_namespace);
        generated_functions.push((func_name.clone(), func_code));
    }

    if !generated_functions.is_empty() {
        writeln!(&mut output, "  // Functions").unwrap();
        for (_name, func_code) in &generated_functions {
            for line in func_code.lines() {
                if line.is_empty() {
                    writeln!(&mut output).unwrap();
                } else {
                    writeln!(&mut output, "  {}", line).unwrap();
                }
            }
            writeln!(&mut output).unwrap();
        }
    }

    writeln!(&mut output, "}}").unwrap();

    // Reset force flag
    ctx.set_force_external_for_current(false);

    output
}

/// Get the package path for an interface.
///
/// For imports: base.package.namespace.name
/// For exports: base.package.exports.namespace.name
pub fn get_package_path(ctx: &ScalaContext, namespace: &str, is_import: bool) -> String {
    let mut segments = ctx.base_package_segments_translated();

    if !is_import {
        segments.push("exports".to_string());
    }

    // Parse namespace which might be like "wasi:io/streams@0.2.0"
    // or just "wasi:io/streams"
    let parts: Vec<&str> = namespace.split(':').collect();
    if parts.len() >= 2 {
        let package_part = parts[0];
        let rest = parts[1];

        segments.push(ctx.to_snake_case(package_part));

        // Split by / for package/interface separation
        // Strip version from interface name if present (e.g., "streams@0.2.0" -> "streams")
        let path_parts: Vec<&str> = rest.split('/').collect();
        if !path_parts.is_empty() {
            // Add package name (strip any version after @)
            let package_name = path_parts[0].split('@').next().unwrap_or(path_parts[0]);
            segments.push(ctx.to_snake_case(package_name));
            for segment in path_parts
                .iter()
                .skip(1)
                .take(path_parts.len().saturating_sub(2))
            {
                let segment = segment.split('@').next().unwrap_or(segment);
                segments.push(ctx.to_snake_case(segment));
            }
        }
    }

    segments.join(".")
}

/// Get the file path for an interface.
///
/// Returns the relative path where the Scala file should be written.
pub fn get_interface_file_path(
    ctx: &ScalaContext,
    namespace: &str,
    interface_name: &str,
    is_import: bool,
) -> String {
    let mut segments = ctx.base_package_segments_translated();

    if !is_import {
        segments.push("exports".to_string());
    }

    // Parse namespace
    let parts: Vec<&str> = namespace.split(':').collect();
    if parts.len() >= 2 {
        let package_part = parts[0];
        let rest = parts[1];

        segments.push(ctx.to_snake_case(package_part));

        // Split by / for package/interface separation
        // Strip version from package name if present
        let path_parts: Vec<&str> = rest.split('/').collect();
        if !path_parts.is_empty() {
            let package_name = path_parts[0].split('@').next().unwrap_or(path_parts[0]);
            segments.push(ctx.to_snake_case(package_name));
            for segment in path_parts
                .iter()
                .skip(1)
                .take(path_parts.len().saturating_sub(2))
            {
                let segment = segment.split('@').next().unwrap_or(segment);
                segments.push(ctx.to_snake_case(segment));
            }
        }
    }

    // Add interface name as file name
    let file_name = format!("{}.scala", ctx.to_snake_case(interface_name));
    let path = segments.join("/");
    format!("{}/{}", path, file_name)
}
