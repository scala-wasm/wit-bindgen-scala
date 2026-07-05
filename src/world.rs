/// World-level code generation for top-level functions and types.
///
/// Worlds can have top-level imports and exports that are not part of
/// any interface. These are generated in separate world files.
use crate::{ScalaContext, annotations};
use std::fmt::Write as _;
use wit_bindgen_core::wit_parser::*;

/// Generate a world file for top-level imports or exports.
pub fn render_world(
    ctx: &mut ScalaContext,
    resolve: &Resolve,
    world_id: WorldId,
    is_import: bool,
    funcs: &[(String, Function)],
) -> Option<String> {
    ctx.set_current_interface(None);

    let world = &resolve.worlds[world_id];
    let _world_name = &world.name;

    let mut has_content = false;
    let mut output = String::new();

    // Determine package path
    let package_path = get_world_package_path(ctx, is_import);
    writeln!(&mut output, "package {}", package_path).unwrap();
    writeln!(&mut output).unwrap();

    if is_import {
        // For imports, use package object root
        writeln!(&mut output, "package object root {{").unwrap();
        writeln!(&mut output).unwrap();
    } else {
        // For exports, use trait Root with @WitExportInterface annotation
        writeln!(&mut output, "{}", annotations::component_export_interface()).unwrap();
        writeln!(&mut output, "trait Root {{").unwrap();
        writeln!(&mut output).unwrap();
    }

    // Generate top-level types
    if is_import {
        for (_name, item) in &world.imports {
            if let WorldItem::Type { id, .. } = item {
                let typedef = ctx.render_typedef(resolve, *id);
                if !typedef.is_empty() && !typedef.starts_with("//") {
                    has_content = true;
                    writeln!(&mut output, "  // Type definitions").unwrap();
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
        }
    } else {
        for (_name, item) in &world.exports {
            if let WorldItem::Type { id, .. } = item {
                let typedef = ctx.render_typedef(resolve, *id);
                if !typedef.is_empty() && !typedef.starts_with("//") {
                    has_content = true;
                    writeln!(&mut output, "  // Type definitions").unwrap();
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
        }
    }

    // Generate world-level functions
    // World-level imports use "$root" as the module name
    // World-level exports use empty string "" for bare exports like (export "function-name")
    if !funcs.is_empty() {
        has_content = true;
        writeln!(&mut output, "  // World-level functions").unwrap();
        let namespace = if is_import { "$root" } else { "" };
        for (_func_name, func) in funcs {
            let func_code = ctx.render_function(resolve, func, is_import, namespace);
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

    if has_content { Some(output) } else { None }
}

/// Get the package path for a world.
pub fn get_world_package_path(ctx: &ScalaContext, is_import: bool) -> String {
    let mut segments = ctx.base_package_segments_translated();

    if !is_import {
        segments.push("exports".to_string());
    }

    segments.join(".")
}

/// Get the file path for a world file.
pub fn get_world_file_path(ctx: &ScalaContext, is_import: bool) -> String {
    let mut segments = ctx.base_package_segments_translated();

    if !is_import {
        segments.push("exports".to_string());
    }

    let path = segments.join("/");
    if is_import {
        format!("{}/package.scala", path)
    } else {
        format!("{}/Root.scala", path)
    }
}
