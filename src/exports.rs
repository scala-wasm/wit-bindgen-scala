/// Implementation objects of exported functions (`--export-impl-package`).
///
/// With `--export-impl-package <prefix>`, every exported function gets a
/// generated `@WitExport` forwarder that calls a method of the same name on an
/// implementation object that the user writes. Its fully qualified name follows
/// a fixed convention, which includes the WIT package, so no two exported
/// interfaces of a world map to the same object:
///
/// | Export | Implementation object |
/// |---|---|
/// | interface `ns:pkg/iface` (any version) | `<prefix>.<ns>.<pkg>.<Iface>` |
/// | inline interface `export foo: interface { ... }` of world `w` in `ns:pkg` | `<prefix>.<ns>.<pkg>.<w>.<Foo>` |
/// | world-level function of world `w` in `ns:pkg` | `<prefix>.<ns>.<pkg>.<W>` |
///
/// Package segments are `snake_case`, object names `PascalCase`. WIT does not
/// allow an interface and a world of the same name in one package, nor two
/// world items of the same name, so the objects never clash.
use crate::ScalaContext;
use wit_bindgen_core::wit_parser::*;

/// The fully qualified implementation object of an exported interface, with a
/// leading `_root_.` so that no generated name can shadow it.
pub fn interface_impl_object(
    ctx: &ScalaContext,
    prefix: &str,
    resolve: &Resolve,
    world_id: WorldId,
    name: &WorldKey,
    interface_id: InterfaceId,
) -> String {
    let interface = &resolve.interfaces[interface_id];
    match (&interface.name, interface.package) {
        (Some(interface_name), Some(package_id)) => {
            let mut segments = package_segments(ctx, prefix, &resolve.packages[package_id].name);
            segments.push(ctx.to_pascal_case(interface_name));
            qualified(segments)
        }
        _ => {
            let world = &resolve.worlds[world_id];
            let mut segments = world_package_segments(ctx, prefix, resolve, world);
            segments.push(escape_package_segment(ctx, &ctx.to_snake_case(&world.name)));
            segments.push(ctx.to_pascal_case(&resolve.name_world_key(name)));
            qualified(segments)
        }
    }
}

/// The fully qualified implementation object of the world-level exported functions.
pub fn world_impl_object(
    ctx: &ScalaContext,
    prefix: &str,
    resolve: &Resolve,
    world_id: WorldId,
) -> String {
    let world = &resolve.worlds[world_id];
    let mut segments = world_package_segments(ctx, prefix, resolve, world);
    segments.push(ctx.to_pascal_case(&world.name));
    qualified(segments)
}

/// The object name without the leading `_root_.`, for messages to the user.
pub fn display_name(impl_object: &str) -> &str {
    impl_object.strip_prefix("_root_.").unwrap_or(impl_object)
}

fn world_package_segments(
    ctx: &ScalaContext,
    prefix: &str,
    resolve: &Resolve,
    world: &World,
) -> Vec<String> {
    match world.package {
        Some(package_id) => package_segments(ctx, prefix, &resolve.packages[package_id].name),
        None => prefix_segments(prefix),
    }
}

fn package_segments(ctx: &ScalaContext, prefix: &str, package: &PackageName) -> Vec<String> {
    let mut segments = prefix_segments(prefix);
    segments.push(escape_package_segment(
        ctx,
        &ctx.to_snake_case(&package.namespace),
    ));
    segments.push(escape_package_segment(
        ctx,
        &ctx.to_snake_case(&package.name),
    ));
    segments
}

fn prefix_segments(prefix: &str) -> Vec<String> {
    prefix
        .split('.')
        .filter(|segment| !segment.is_empty())
        .map(str::to_string)
        .collect()
}

/// Package segments that are Scala keywords need backticks (`` `type` ``).
fn escape_package_segment(ctx: &ScalaContext, segment: &str) -> String {
    let escaped = ctx.escape_keyword(segment);
    if escaped.starts_with('`') {
        escaped
    } else {
        segment.to_string()
    }
}

fn qualified(segments: Vec<String>) -> String {
    format!("_root_.{}", segments.join("."))
}

/// Print the implementation object the user must write, with one stub per
/// exported function. `own_types` is the generated Scala path of the exported
/// interface's (or world's) own types: the stubs live in another package.
pub fn print_impl_stubs(
    ctx: &mut ScalaContext,
    resolve: &Resolve,
    wit_name: &str,
    impl_object: &str,
    own_types: &str,
    funcs: &[&Function],
) {
    let name = display_name(impl_object);
    let (package, object) = name.rsplit_once('.').unwrap_or(("", name));
    let wit_name = if wit_name.is_empty() {
        "the world"
    } else {
        wit_name
    };
    eprintln!("Implement the exports of `{}` in `{}`:", wit_name, name);
    eprintln!();
    if !package.is_empty() {
        eprintln!("package {}", package);
        eprintln!();
    }
    eprintln!("object {} {{", object);
    ctx.set_own_type_prefix(Some(own_types.to_string()));
    for func in funcs {
        eprintln!("  {}", ctx.render_impl_stub(resolve, func));
    }
    ctx.set_own_type_prefix(None);
    eprintln!("}}");
    eprintln!();
}
