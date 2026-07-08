/// Resource generation for WebAssembly Component Model resources.
///
/// Resources represent opaque handles to objects that can have methods,
/// constructors, and destructors. This module generates Scala final-class
/// representations for imported resources.
use crate::{
    ScalaContext, annotations,
    context::{format_docs, format_docs_with_indent},
};
use std::fmt::Write as _;
use wit_bindgen_core::wit_parser::*;

/// Generate an imported resource as a Scala final class with companion object.
///
/// Imported resources are defined by the host and accessed from guest code.
/// They have methods marked with `= scala.scalajs.component.native`.
pub fn render_imported_resource(
    ctx: &mut ScalaContext,
    resolve: &Resolve,
    resource_id: TypeId,
    namespace: &str,
) -> String {
    let resource = &resolve.types[resource_id];
    let resource_name = resource.name.as_ref().expect("Resource must have a name");
    let scala_name = ctx.to_pascal_case(resource_name);

    let mut output = String::new();

    // Generate scaladoc if docs exist
    let docs = format_docs(&resource.docs);
    if !docs.is_empty() {
        write!(&mut output, "{}", docs).unwrap();
    }

    // Generate the class with annotation
    writeln!(
        &mut output,
        "{}",
        annotations::component_resource_import(namespace, resource_name)
    )
    .unwrap();
    // Resources model an opaque host-owned handle whose lifetime the guest must
    // release. Extending `AutoCloseable` lets callers use try-with-resources
    // style management; the generated `close()` (the resource drop) satisfies it.
    writeln!(
        &mut output,
        "final class {} private () extends Object with AutoCloseable {{",
        scala_name
    )
    .unwrap();

    // Collect instance methods
    if let TypeOwner::Interface(iface_id) = resource.owner {
        let iface = &resolve.interfaces[iface_id];

        for (_func_key, func) in &iface.functions {
            if let FunctionKind::Method(method_resource_id) = func.kind {
                if method_resource_id == resource_id {
                    let method = render_resource_method(ctx, resolve, func);
                    write!(&mut output, "{}", method).unwrap();
                }
            }
        }
    }

    // Add drop method
    let drop_method = render_resource_drop_method();
    write!(&mut output, "{}", drop_method).unwrap();

    writeln!(&mut output, "}}").unwrap();

    // Generate companion object for static methods and constructor
    writeln!(&mut output, "object {} {{", scala_name).unwrap();

    // Check for constructor and static methods
    if let TypeOwner::Interface(iface_id) = resource.owner {
        let iface = &resolve.interfaces[iface_id];

        for (_func_key, func) in &iface.functions {
            match func.kind {
                FunctionKind::Constructor(ctor_resource_id) if ctor_resource_id == resource_id => {
                    let ctor = render_resource_constructor(ctx, resolve, &scala_name, func);
                    write!(&mut output, "{}", ctor).unwrap();
                }
                FunctionKind::Static(static_resource_id) if static_resource_id == resource_id => {
                    let static_method = render_resource_static_method(ctx, resolve, func);
                    write!(&mut output, "{}", static_method).unwrap();
                }
                _ => {}
            }
        }
    }

    writeln!(&mut output, "}}").unwrap();

    output
}

/// Render an imported resource instance method.
pub fn render_resource_method(
    ctx: &mut ScalaContext,
    resolve: &Resolve,
    func: &Function,
) -> String {
    let method_name = ctx.to_camel_case(func.item_name());
    let mut output = String::new();

    // Generate scaladoc if docs exist (with 2-space indentation for class body)
    let docs = format_docs_with_indent(&func.docs, 2);
    if !docs.is_empty() {
        write!(&mut output, "{}", docs).unwrap();
    }

    writeln!(
        &mut output,
        "  {}",
        annotations::component_resource_method(func.item_name())
    )
    .unwrap();
    write!(&mut output, "  def {}(", method_name).unwrap();

    // Render parameters (skip first parameter which is 'self' for instance methods)
    let params_to_render = if func.params.is_empty() {
        &func.params[..]
    } else {
        &func.params[1..]
    };

    for (i, param) in params_to_render.iter().enumerate() {
        if i > 0 {
            write!(&mut output, ", ").unwrap();
        }
        let scala_param = ctx.to_camel_case(&param.name);
        let scala_type = ctx.render_type(resolve, &param.ty);
        write!(&mut output, "{}: {}", scala_param, scala_type).unwrap();
    }

    write!(&mut output, ")").unwrap();

    // Render return type
    if let Some(ret_ty) = &func.result {
        let scala_ret = ctx.render_type(resolve, ret_ty);
        write!(&mut output, ": {}", scala_ret).unwrap();
    } else {
        write!(&mut output, ": Unit").unwrap();
    }

    writeln!(&mut output, " = {}", annotations::native_marker()).unwrap();

    output
}

/// Render an imported resource constructor.
pub fn render_resource_constructor(
    ctx: &mut ScalaContext,
    resolve: &Resolve,
    scala_name: &str,
    func: &Function,
) -> String {
    let mut output = String::new();

    // Generate scaladoc if docs exist (with 2-space indentation for companion object body)
    let docs = format_docs_with_indent(&func.docs, 2);
    if !docs.is_empty() {
        write!(&mut output, "{}", docs).unwrap();
    }

    writeln!(
        &mut output,
        "  {}",
        annotations::component_resource_constructor()
    )
    .unwrap();
    write!(&mut output, "  def apply(").unwrap();

    // Render parameters
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            write!(&mut output, ", ").unwrap();
        }
        let scala_param = ctx.to_camel_case(&param.name);
        let scala_type = ctx.render_type(resolve, &param.ty);
        write!(&mut output, "{}: {}", scala_param, scala_type).unwrap();
    }

    write!(&mut output, ")").unwrap();
    write!(&mut output, ": {}", scala_name).unwrap();
    writeln!(&mut output, " = {}", annotations::native_marker()).unwrap();

    output
}

/// Render an imported resource static method.
fn render_resource_static_method(
    ctx: &mut ScalaContext,
    resolve: &Resolve,
    func: &Function,
) -> String {
    let method_name = ctx.to_camel_case(func.item_name());
    let mut output = String::new();

    // Generate scaladoc if docs exist (with 2-space indentation for companion object body)
    let docs = format_docs_with_indent(&func.docs, 2);
    if !docs.is_empty() {
        write!(&mut output, "{}", docs).unwrap();
    }

    writeln!(
        &mut output,
        "  {}",
        annotations::component_resource_static_method(func.item_name())
    )
    .unwrap();
    write!(&mut output, "  def {}(", method_name).unwrap();

    // Render parameters
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            write!(&mut output, ", ").unwrap();
        }
        let scala_param = ctx.to_camel_case(&param.name);
        let scala_type = ctx.render_type(resolve, &param.ty);
        write!(&mut output, "{}: {}", scala_param, scala_type).unwrap();
    }

    write!(&mut output, ")").unwrap();

    // Render return type
    if let Some(ret_ty) = &func.result {
        let scala_ret = ctx.render_type(resolve, ret_ty);
        write!(&mut output, ": {}", scala_ret).unwrap();
    } else {
        write!(&mut output, ": Unit").unwrap();
    }

    writeln!(&mut output, " = {}", annotations::native_marker()).unwrap();

    output
}

/// Render the resource drop method.
pub fn render_resource_drop_method() -> String {
    let mut output = String::new();
    writeln!(&mut output, "  {}", annotations::component_resource_drop()).unwrap();
    writeln!(
        &mut output,
        "  def close(): Unit = {}",
        annotations::native_marker()
    )
    .unwrap();
    output
}
