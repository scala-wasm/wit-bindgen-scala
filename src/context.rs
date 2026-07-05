use crate::{Opts, annotations};
use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use wit_bindgen_core::wit_parser::*;

/// Format WIT documentation as Scaladoc comments.
///
/// Converts WIT documentation strings into properly formatted Scaladoc with
/// the correct indentation and continuation markers.
pub fn format_docs(docs: &Docs) -> String {
    format_docs_with_indent(docs, 0)
}

/// Format WIT documentation as Scaladoc comments with custom indentation.
///
/// Converts WIT documentation strings into properly formatted Scaladoc with
/// the specified indentation level (number of spaces) and continuation markers.
pub fn format_docs_with_indent(docs: &Docs, indent: usize) -> String {
    let content = docs.contents.as_deref().unwrap_or("").trim();
    if content.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() {
        return String::new();
    }

    let indent_str = " ".repeat(indent);

    // First line with opening /**
    writeln!(&mut output, "{}/** {}", indent_str, lines[0]).unwrap();

    // Subsequent lines with continuation marker
    for line in &lines[1..] {
        if line.trim().is_empty() {
            writeln!(&mut output, "{} *", indent_str).unwrap();
        } else {
            writeln!(&mut output, "{} *  {}", indent_str, line).unwrap();
        }
    }

    // Closing */
    writeln!(&mut output, "{} */", indent_str).unwrap();

    output
}

/// Context for Scala code generation, containing shared utilities and state.
pub struct ScalaContext {
    opts: Opts,
    keywords: ScalaKeywords,
    /// Current interface being rendered (for cross-interface type references)
    current_interface: Option<InterfaceId>,
    /// WIT interface name → external Scala package path (for `--with` remapped interfaces)
    with_map: HashMap<String, String>,
    /// When true, even same-interface types resolve through `with_map`.
    /// Used when rendering export-trait-only for a remapped interface.
    force_external_for_current: bool,
}

impl ScalaContext {
    pub fn new(opts: &Opts) -> Self {
        Self {
            opts: opts.clone(),
            keywords: ScalaKeywords::new(),
            current_interface: None,
            with_map: HashMap::new(),
            force_external_for_current: false,
        }
    }

    /// Set the with_map for external interface remapping.
    pub fn set_with_map(&mut self, with_map: HashMap<String, String>) {
        self.with_map = with_map;
    }

    /// Set whether the current interface's own types should resolve through `with_map`.
    pub fn set_force_external_for_current(&mut self, force: bool) {
        self.force_external_for_current = force;
    }

    /// Set the current interface being rendered (for cross-interface type references).
    pub fn set_current_interface(&mut self, interface_id: Option<InterfaceId>) {
        self.current_interface = interface_id;
    }

    /// Build the WIT interface key (e.g., `ns:pkg/iface@ver`) for an InterfaceId.
    fn build_wit_interface_key(
        &self,
        resolve: &Resolve,
        interface_id: InterfaceId,
    ) -> Option<String> {
        let interface = &resolve.interfaces[interface_id];
        let interface_name = interface.name.as_ref()?;
        let package_id = interface.package?;
        let package = &resolve.packages[package_id];
        let pkg_name = &package.name;
        if let Some(version) = &pkg_name.version {
            Some(format!(
                "{}:{}/{}@{}",
                pkg_name.namespace, pkg_name.name, interface_name, version
            ))
        } else {
            Some(format!(
                "{}:{}/{}",
                pkg_name.namespace, pkg_name.name, interface_name
            ))
        }
    }

    pub fn version_segment_if_needed(
        &self,
        resolve: &Resolve,
        package_id: PackageId,
    ) -> Option<String> {
        let package = &resolve.packages[package_id];
        let package_name = &package.name;
        let has_multiple_versions = resolve.packages.iter().any(|(_, other)| {
            other.name.namespace == package_name.namespace
                && other.name.name == package_name.name
                && other.name.version != package_name.version
        });
        if !has_multiple_versions {
            return None;
        }

        package_name
            .version
            .as_ref()
            .map(|version| format!("v{}", version.to_string().replace(['.', '-', '+'], "_")))
    }

    /// Look up whether an interface has been remapped to an external Scala package.
    ///
    /// Tries matching in order: exact → version-stripped → package-level.
    /// For package-level matches, the interface name is appended as a snake_case segment.
    fn get_remapped_path(&self, resolve: &Resolve, interface_id: InterfaceId) -> Option<String> {
        let key = self.build_wit_interface_key(resolve, interface_id)?;
        let interface = &resolve.interfaces[interface_id];
        let interface_name = interface.name.as_ref()?;

        // Exact match (e.g., "wasi:cli/environment@0.2.0")
        if let Some(path) = self.with_map.get(&key) {
            return Some(path.clone());
        }
        // Version-stripped match (e.g., "wasi:cli/environment")
        if let Some(stripped) = crate::strip_version(&key) {
            if let Some(path) = self.with_map.get(stripped) {
                return Some(path.clone());
            }
        }
        // Package-level match (e.g., "wasi:cli") — append interface name
        if let Some(pkg) = crate::strip_interface(&key) {
            if let Some(path) = self.with_map.get(pkg) {
                return Some(format!("{}.{}", path, self.to_snake_case(interface_name)));
            }
        }
        None
    }

    /// Generate fully qualified package path for a type from another interface.
    ///
    /// When a type comes from a remapped interface (via `--with`), resolves to
    /// `external.package.path.TypeName` instead of the generated path.
    fn get_qualified_type_name(
        &self,
        resolve: &Resolve,
        type_id: TypeId,
        type_name: &str,
    ) -> String {
        let ty = &resolve.types[type_id];

        // Check if this type is from a different interface (or force_external_for_current)
        if let TypeOwner::Interface(type_interface_id) = ty.owner {
            let is_cross_interface = self
                .current_interface
                .map_or(true, |cid| cid != type_interface_id);

            if is_cross_interface || self.force_external_for_current {
                // Check if the type's interface has been remapped to an external package
                if let Some(external_path) = self.get_remapped_path(resolve, type_interface_id) {
                    return format!("{}.{}", external_path, self.to_pascal_case(type_name));
                }

                // Not remapped — use existing generated path logic for cross-interface refs
                if is_cross_interface {
                    let type_interface = &resolve.interfaces[type_interface_id];
                    let interface_name = type_interface
                        .name
                        .as_ref()
                        .expect("Interface must have a name");

                    if let Some(package_id) = type_interface.package {
                        let package = &resolve.packages[package_id];
                        let pkg_name = &package.name;

                        let mut segments = self.base_package_segments_translated();
                        segments.push(self.to_snake_case(&pkg_name.namespace));
                        segments.push(self.to_snake_case(&pkg_name.name));
                        if let Some(version) = self.version_segment_if_needed(resolve, package_id) {
                            segments.push(version);
                        }
                        segments.push(self.to_snake_case(interface_name));
                        segments.push(self.to_pascal_case(type_name));

                        return segments.join(".");
                    }
                }
            }
        }

        // Same interface or no interface context - use simple name
        self.to_pascal_case(type_name)
    }

    /// Render a WIT type to its Scala equivalent with fully qualified names.
    pub fn render_type(&mut self, resolve: &Resolve, ty: &Type) -> String {
        match ty {
            // Primitive types - delegate to render_primitive_type
            Type::Bool
            | Type::S8
            | Type::U8
            | Type::S16
            | Type::U16
            | Type::S32
            | Type::U32
            | Type::S64
            | Type::U64
            | Type::F32
            | Type::F64
            | Type::Char
            | Type::String => self.render_primitive_type(ty).to_string(),
            Type::Id(id) => self.render_type_id(resolve, *id),
            Type::ErrorContext => panic!("ErrorContext type is not supported"),
        }
    }

    /// Render a type ID reference with fully qualified name.
    fn render_type_id(&mut self, resolve: &Resolve, id: TypeId) -> String {
        let ty = &resolve.types[id];

        // Check what kind of type this is
        match &ty.kind {
            TypeDefKind::List(inner) => {
                // list<T> maps to Array[T]
                format!("Array[{}]", self.render_type(resolve, inner))
            }
            TypeDefKind::Map(key, value) => {
                format!(
                    "scala.collection.immutable.Map[{}, {}]",
                    self.render_type(resolve, key),
                    self.render_type(resolve, value)
                )
            }
            TypeDefKind::Option(inner) => {
                // option<T> maps to java.util.Optional[T]
                format!("java.util.Optional[{}]", self.render_type(resolve, inner))
            }
            TypeDefKind::Result(result) => {
                // result<T, E> maps to scala.scalajs.wit.Result[T, E]
                let ok_type = result
                    .ok
                    .as_ref()
                    .map(|t| self.render_type(resolve, t))
                    .unwrap_or_else(|| "Unit".to_string());
                let err_type = result
                    .err
                    .as_ref()
                    .map(|t| self.render_type(resolve, t))
                    .unwrap_or_else(|| "Unit".to_string());
                format!("scala.scalajs.wit.Result[{}, {}]", ok_type, err_type)
            }
            TypeDefKind::Tuple(tuple) => {
                // tuple<T1, T2, ...> maps to scala.scalajs.wit.TupleN[...]
                let type_params: Vec<String> = tuple
                    .types
                    .iter()
                    .map(|t| self.render_type(resolve, t))
                    .collect();
                format!(
                    "scala.scalajs.wit.Tuple{}[{}]",
                    type_params.len(),
                    type_params.join(", ")
                )
            }
            TypeDefKind::Record(_)
            | TypeDefKind::Variant(_)
            | TypeDefKind::Enum(_)
            | TypeDefKind::Flags(_) => {
                // Named types - use qualified name if from different interface
                let type_name = ty.name.as_ref().expect("Named types must have a name");
                self.get_qualified_type_name(resolve, id, type_name)
            }
            TypeDefKind::Type(inner) => {
                // Type alias - render the underlying type
                self.render_type(resolve, inner)
            }
            TypeDefKind::Handle(handle) => {
                // Handle to a resource - follow the reference to get the resource name
                use wit_bindgen_core::wit_parser::Handle;
                let resource_id = match handle {
                    Handle::Own(id) | Handle::Borrow(id) => *id,
                };
                let resource_ty = &resolve.types[resource_id];
                let type_name = resource_ty
                    .name
                    .as_ref()
                    .expect("Resources must have a name");
                self.get_qualified_type_name(resolve, resource_id, type_name)
            }
            TypeDefKind::Resource => {
                // Resource definition - use qualified name if from different interface
                let type_name = ty.name.as_ref().expect("Resources must have a name");
                self.get_qualified_type_name(resolve, id, type_name)
            }
            TypeDefKind::FixedLengthList(inner, _size) => {
                // Fixed-size list also maps to Array[T]
                format!("Array[{}]", self.render_type(resolve, inner))
            }
            TypeDefKind::Future(_) | TypeDefKind::Stream(_) | TypeDefKind::Unknown => {
                "Unknown".to_string()
            }
        }
    }

    /// Render a WIT primitive type to its Scala equivalent.
    ///
    /// This returns non-fully qualified names for primitive types and fully qualified names
    /// for unsigned types from the scala.scalajs.component.unsigned package.
    pub fn render_primitive_type(&mut self, ty: &Type) -> &'static str {
        match ty {
            Type::Bool => "Boolean",
            Type::S8 => "Byte",
            Type::U8 => "scala.scalajs.wit.unsigned.UByte",
            Type::S16 => "Short",
            Type::U16 => "scala.scalajs.wit.unsigned.UShort",
            Type::S32 => "Int",
            Type::U32 => "scala.scalajs.wit.unsigned.UInt",
            Type::S64 => "Long",
            Type::U64 => "scala.scalajs.wit.unsigned.ULong",
            Type::F32 => "Float",
            Type::F64 => "Double",
            Type::Char => "Char",
            Type::String => "String",
            _ => unreachable!("Not a primitive type: {:?}", ty),
        }
    }

    /// Generate equals method for a class with the given fields.
    /// fields: Vec<(field_name, field_type)>
    fn render_equals_method(
        &self,
        class_name: &str,
        fields: &[(String, String)],
        indent: usize,
    ) -> String {
        let indent_str = "  ".repeat(indent);
        let mut output = String::new();

        writeln!(
            &mut output,
            "{}override def equals(other: Any): Boolean = other match {{",
            indent_str
        )
        .unwrap();
        if fields.is_empty() {
            writeln!(
                &mut output,
                "{}  case _: {} => true",
                indent_str, class_name
            )
            .unwrap();
        } else {
            let field_comparisons: Vec<String> = fields
                .iter()
                .map(|(name, _)| format!("this.{} == that.{}", name, name))
                .collect();
            writeln!(
                &mut output,
                "{}  case that: {} => {}",
                indent_str,
                class_name,
                field_comparisons.join(" && ")
            )
            .unwrap();
        }
        writeln!(&mut output, "{}  case _ => false", indent_str).unwrap();
        writeln!(&mut output, "{}}}", indent_str).unwrap();

        output
    }

    /// Generate hashCode method for a class with the given fields.
    fn render_hash_code_method(&self, fields: &[(String, String)], indent: usize) -> String {
        let indent_str = "  ".repeat(indent);
        let mut output = String::new();

        writeln!(
            &mut output,
            "{}override def hashCode(): Int = {{",
            indent_str
        )
        .unwrap();
        if fields.is_empty() {
            writeln!(&mut output, "{}  1", indent_str).unwrap();
        } else if fields.len() == 1 {
            writeln!(&mut output, "{}  {}.hashCode()", indent_str, fields[0].0).unwrap();
        } else {
            writeln!(&mut output, "{}  var result = 1", indent_str).unwrap();
            for (name, _) in fields {
                writeln!(
                    &mut output,
                    "{}  result = 31 * result + {}.hashCode()",
                    indent_str, name
                )
                .unwrap();
            }
            writeln!(&mut output, "{}  result", indent_str).unwrap();
        }
        writeln!(&mut output, "{}}}", indent_str).unwrap();

        output
    }

    /// Generate toString method for a class.
    fn render_to_string_method(
        &self,
        class_name: &str,
        field_names: &[String],
        indent: usize,
    ) -> String {
        let indent_str = "  ".repeat(indent);
        let mut output = String::new();

        write!(
            &mut output,
            "{}override def toString(): String = \"{}(\"",
            indent_str, class_name
        )
        .unwrap();
        for (i, name) in field_names.iter().enumerate() {
            if i > 0 {
                write!(&mut output, " + \", \"").unwrap();
            }
            write!(&mut output, " + {}", name).unwrap();
        }
        writeln!(&mut output, " + \")\"").unwrap();

        output
    }

    /// Generate apply method for a companion object.
    fn render_apply_method(
        &self,
        class_name: &str,
        fields: &[(String, String)],
        indent: usize,
    ) -> String {
        let indent_str = "  ".repeat(indent);
        let params: Vec<String> = fields
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .collect();
        let args: Vec<String> = fields.iter().map(|(name, _)| name.clone()).collect();

        format!(
            "{}def apply({}): {} = new {}({})\n",
            indent_str,
            params.join(", "),
            class_name,
            class_name,
            args.join(", ")
        )
    }

    /// Generate unapply method for a companion object.
    fn render_unapply_method(
        &self,
        class_name: &str,
        fields: &[(String, String)],
        indent: usize,
    ) -> String {
        let indent_str = "  ".repeat(indent);

        if fields.is_empty() {
            // For empty class, unapply returns Boolean
            format!(
                "{}def unapply(arg: {}): Boolean = true\n",
                indent_str, class_name
            )
        } else if fields.len() == 1 {
            // For single field, unapply returns Some[T]
            let (name, ty) = &fields[0];
            format!(
                "{}def unapply(arg: {}): Some[{}] = Some(arg.{})\n",
                indent_str, class_name, ty, name
            )
        } else {
            // For multiple fields, unapply returns Some[(T1, T2, ...)]
            let types: Vec<String> = fields.iter().map(|(_, ty)| ty.clone()).collect();
            let args: Vec<String> = fields
                .iter()
                .map(|(name, _)| format!("arg.{}", name))
                .collect();
            format!(
                "{}def unapply(arg: {}): Some[({})] = Some(({}))\n",
                indent_str,
                class_name,
                types.join(", "),
                args.join(", ")
            )
        }
    }

    /// Render a typedef (record, variant, enum, flags, etc.) to Scala code.
    pub fn render_typedef(&mut self, resolve: &Resolve, id: TypeId) -> String {
        let ty = &resolve.types[id];
        let name = ty.name.as_ref().expect("Type must have a name");
        let type_name = self.to_pascal_case(name);

        match &ty.kind {
            TypeDefKind::Record(record) => {
                self.render_record(&type_name, record, resolve, &ty.docs)
            }
            TypeDefKind::Variant(variant) => {
                let scope_name = match ty.owner {
                    TypeOwner::Interface(interface_id) => resolve.interfaces[interface_id]
                        .name
                        .as_ref()
                        .map(|name| self.to_snake_case(name)),
                    _ => None,
                };
                self.render_variant(
                    &type_name,
                    variant,
                    resolve,
                    &ty.docs,
                    scope_name.as_deref(),
                )
            }
            TypeDefKind::Enum(enum_) => self.render_enum(&type_name, enum_, &ty.docs),
            TypeDefKind::Flags(flags) => self.render_flags(&type_name, flags, &ty.docs),
            TypeDefKind::Tuple(tuple) => self.render_tuple_typedef(&type_name, tuple, resolve),
            TypeDefKind::Option(inner) => self.render_option_typedef(&type_name, inner, resolve),
            TypeDefKind::Result(result) => self.render_result_typedef(&type_name, result, resolve),
            TypeDefKind::List(inner) => self.render_list_typedef(&type_name, inner, resolve),
            TypeDefKind::Map(key, value) => format!(
                "type {} = scala.collection.immutable.Map[{}, {}]",
                type_name,
                self.render_type(resolve, key),
                self.render_type(resolve, value)
            ),
            TypeDefKind::Type(inner) => {
                // Type alias
                format!("type {} = {}", type_name, self.render_type(resolve, inner))
            }
            TypeDefKind::Handle(_handle) => {
                // Resources are handled separately
                format!("// Resource: {}", type_name)
            }
            TypeDefKind::Resource => {
                // Resources are handled separately
                format!("// Resource: {}", type_name)
            }
            TypeDefKind::FixedLengthList(inner, size) => {
                // Fixed-size lists map to Array
                format!(
                    "type {} = Array[{}] // Fixed size: {}",
                    type_name,
                    self.render_type(resolve, inner),
                    size
                )
            }
            TypeDefKind::Future(_) | TypeDefKind::Stream(_) | TypeDefKind::Unknown => {
                panic!("Unsupported type: {:?}", ty.kind)
            }
        }
    }

    /// Render a record type as a Scala class with companion object.
    fn render_record(
        &mut self,
        name: &str,
        record: &Record,
        resolve: &Resolve,
        type_docs: &Docs,
    ) -> String {
        let mut output = String::new();

        // Generate scaladoc if docs exist
        let docs = format_docs(type_docs);
        if !docs.is_empty() {
            write!(&mut output, "{}", docs).unwrap();
        }

        // Collect field information
        let fields: Vec<(String, String)> = record
            .fields
            .iter()
            .map(|field| {
                let field_name = self.to_camel_case(&field.name);
                let field_type = self.render_type(resolve, &field.ty);
                (field_name, field_type)
            })
            .collect();

        // Generate class declaration
        writeln!(&mut output, "{}", annotations::component_record()).unwrap();
        write!(&mut output, "final class {}(", name).unwrap();
        for (i, (field_name, field_type)) in fields.iter().enumerate() {
            if i > 0 {
                write!(&mut output, ", ").unwrap();
            }
            write!(&mut output, "val {}: {}", field_name, field_type).unwrap();
        }
        writeln!(&mut output, ") {{").unwrap();

        // Generate helper methods
        let field_names: Vec<String> = fields.iter().map(|(name, _)| name.clone()).collect();
        write!(
            &mut output,
            "{}",
            self.render_equals_method(name, &fields, 1)
        )
        .unwrap();
        write!(&mut output, "{}", self.render_hash_code_method(&fields, 1)).unwrap();
        write!(
            &mut output,
            "{}",
            self.render_to_string_method(name, &field_names, 1)
        )
        .unwrap();

        writeln!(&mut output, "}}").unwrap();

        // Generate companion object
        writeln!(&mut output, "object {} {{", name).unwrap();
        write!(
            &mut output,
            "{}",
            self.render_apply_method(name, &fields, 1)
        )
        .unwrap();

        if self.opts.generate_unapply {
            write!(
                &mut output,
                "{}",
                self.render_unapply_method(name, &fields, 1)
            )
            .unwrap();
        }

        writeln!(&mut output, "}}").unwrap();

        output
    }

    /// Render a variant type as a Scala sealed trait with classes/objects.
    fn render_variant(
        &mut self,
        name: &str,
        variant: &Variant,
        resolve: &Resolve,
        type_docs: &Docs,
        scope_name: Option<&str>,
    ) -> String {
        let mut output = String::new();

        // Generate scaladoc if docs exist
        let docs = format_docs(type_docs);
        if !docs.is_empty() {
            write!(&mut output, "{}", docs).unwrap();
        }

        writeln!(&mut output, "{}", annotations::component_variant()).unwrap();
        writeln!(&mut output, "sealed trait {}", name).unwrap();
        writeln!(&mut output, "object {} {{", name).unwrap();

        for case in &variant.cases {
            let case_name = self.to_pascal_case(&case.name);
            match &case.ty {
                Some(ty) => {
                    let mut case_type = self.render_type(resolve, ty);
                    if case_type == case_name {
                        if let Some(scope_name) = scope_name {
                            case_type = format!("{}.{}", scope_name, case_type);
                        }
                    }
                    let fields = vec![("value".to_string(), case_type.clone())];

                    // Generate class with payload
                    writeln!(
                        &mut output,
                        "  final class {}(val value: {}) extends {} {{",
                        case_name, case_type, name
                    )
                    .unwrap();
                    write!(
                        &mut output,
                        "{}",
                        self.render_equals_method(&case_name, &fields, 2)
                    )
                    .unwrap();
                    write!(&mut output, "{}", self.render_hash_code_method(&fields, 2)).unwrap();
                    write!(
                        &mut output,
                        "{}",
                        self.render_to_string_method(&case_name, &["value".to_string()], 2)
                    )
                    .unwrap();
                    writeln!(&mut output, "  }}").unwrap();

                    // Generate companion object
                    writeln!(&mut output, "  object {} {{", case_name).unwrap();
                    write!(
                        &mut output,
                        "{}",
                        self.render_apply_method(&case_name, &fields, 2)
                    )
                    .unwrap();

                    if self.opts.generate_unapply {
                        write!(
                            &mut output,
                            "{}",
                            self.render_unapply_method(&case_name, &fields, 2)
                        )
                        .unwrap();
                    }

                    writeln!(&mut output, "  }}").unwrap();
                }
                None => {
                    // Generate object without payload
                    writeln!(&mut output, "  object {} extends {} {{", case_name, name).unwrap();
                    writeln!(
                        &mut output,
                        "    override def toString(): String = \"{}\"",
                        case_name
                    )
                    .unwrap();
                    writeln!(&mut output, "  }}").unwrap();
                }
            }
        }

        writeln!(&mut output, "}}").unwrap();
        output
    }

    /// Render an enum type as a Scala sealed trait with objects.
    fn render_enum(&mut self, name: &str, enum_: &Enum, type_docs: &Docs) -> String {
        let mut output = String::new();

        // Generate scaladoc if docs exist
        let docs = format_docs(type_docs);
        if !docs.is_empty() {
            write!(&mut output, "{}", docs).unwrap();
        }

        writeln!(&mut output, "{}", annotations::component_variant()).unwrap();
        writeln!(&mut output, "sealed trait {}", name).unwrap();
        writeln!(&mut output, "object {} {{", name).unwrap();

        for case in &enum_.cases {
            let case_name = self.to_pascal_case(&case.name);
            writeln!(&mut output, "  object {} extends {} {{", case_name, name).unwrap();
            writeln!(
                &mut output,
                "    override def toString(): String = \"{}\"",
                case_name
            )
            .unwrap();
            writeln!(&mut output, "  }}").unwrap();
        }

        writeln!(&mut output, "}}").unwrap();
        output
    }

    /// Render a flags type as a Scala class with bitwise operators.
    fn render_flags(&mut self, name: &str, flags: &Flags, type_docs: &Docs) -> String {
        let mut output = String::new();

        // Generate scaladoc if docs exist
        let docs = format_docs(type_docs);
        if !docs.is_empty() {
            write!(&mut output, "{}", docs).unwrap();
        }

        writeln!(
            &mut output,
            "{}",
            annotations::component_flags(flags.flags.len())
        )
        .unwrap();
        writeln!(&mut output, "final class {}(val value: Int) {{", name).unwrap();

        // Bitwise operators
        writeln!(
            &mut output,
            "  def |(other: {}): {} = new {}(value | other.value)",
            name, name, name
        )
        .unwrap();
        writeln!(
            &mut output,
            "  def &(other: {}): {} = new {}(value & other.value)",
            name, name, name
        )
        .unwrap();
        writeln!(
            &mut output,
            "  def ^(other: {}): {} = new {}(value ^ other.value)",
            name, name, name
        )
        .unwrap();
        writeln!(
            &mut output,
            "  def unary_~ : {} = new {}(~value)",
            name, name
        )
        .unwrap();
        writeln!(
            &mut output,
            "  def contains(other: {}): Boolean = (value & other.value) == other.value",
            name
        )
        .unwrap();

        // Helper methods
        let fields = vec![("value".to_string(), "Int".to_string())];
        write!(
            &mut output,
            "{}",
            self.render_equals_method(name, &fields, 1)
        )
        .unwrap();
        write!(&mut output, "{}", self.render_hash_code_method(&fields, 1)).unwrap();
        write!(
            &mut output,
            "{}",
            self.render_to_string_method(name, &["value".to_string()], 1)
        )
        .unwrap();

        writeln!(&mut output, "}}").unwrap();

        // Companion object
        writeln!(&mut output, "object {} {{", name).unwrap();
        write!(
            &mut output,
            "{}",
            self.render_apply_method(name, &fields, 1)
        )
        .unwrap();

        if self.opts.generate_unapply {
            write!(
                &mut output,
                "{}",
                self.render_unapply_method(name, &fields, 1)
            )
            .unwrap();
        }

        for (i, flag) in flags.flags.iter().enumerate() {
            let flag_name = self.to_camel_case(&flag.name);
            writeln!(
                &mut output,
                "  val {} = new {}(1 << {})",
                flag_name, name, i
            )
            .unwrap();
        }
        writeln!(&mut output, "}}").unwrap();

        output
    }

    /// Render a tuple type reference.
    fn render_tuple_typedef(&mut self, name: &str, tuple: &Tuple, resolve: &Resolve) -> String {
        let mut type_params = String::new();
        for (i, ty) in tuple.types.iter().enumerate() {
            if i > 0 {
                type_params.push_str(", ");
            }
            type_params.push_str(&self.render_type(resolve, ty));
        }
        format!(
            "type {} = scala.scalajs.wit.Tuple{}[{}]",
            name,
            tuple.types.len(),
            type_params
        )
    }

    /// Render an option type reference.
    fn render_option_typedef(&mut self, name: &str, inner: &Type, resolve: &Resolve) -> String {
        format!(
            "type {} = java.util.Optional[{}]",
            name,
            self.render_type(resolve, inner)
        )
    }

    /// Render a result type reference.
    fn render_result_typedef(&mut self, name: &str, result: &Result_, resolve: &Resolve) -> String {
        let ok_type = result
            .ok
            .as_ref()
            .map(|t| self.render_type(resolve, t))
            .unwrap_or_else(|| "Unit".to_string());
        let err_type = result
            .err
            .as_ref()
            .map(|t| self.render_type(resolve, t))
            .unwrap_or_else(|| "Unit".to_string());
        format!(
            "type {} = scala.scalajs.wit.Result[{}, {}]",
            name, ok_type, err_type
        )
    }

    /// Render a list type reference.
    fn render_list_typedef(&mut self, name: &str, inner: &Type, resolve: &Resolve) -> String {
        format!(
            "type {} = Array[{}]",
            name,
            self.render_type(resolve, inner)
        )
    }

    /// Escape Scala keywords and java.lang.Object methods.
    /// - Scala keywords/reserved words: wrap in backticks
    /// - java.lang.Object methods: append underscore suffix (backticks don't prevent overriding)
    pub fn escape_keyword(&self, name: &str) -> String {
        if self.keywords.is_object_method(name) {
            format!("{}_", name)
        } else if self.keywords.is_keyword(name) {
            format!("`{}`", name)
        } else {
            name.to_string()
        }
    }

    /// Convert a kebab-case name to camelCase (for method names, variables).
    pub fn to_camel_case(&self, name: &str) -> String {
        self.escape_keyword(&name.to_lower_camel_case())
    }

    /// Convert a kebab-case name to PascalCase (for type names, constructors).
    pub fn to_pascal_case(&self, name: &str) -> String {
        self.escape_keyword(&name.to_pascal_case())
    }

    /// Convert a kebab-case name to snake_case (for package names, file names).
    pub fn to_snake_case(&self, name: &str) -> String {
        name.to_snake_case()
    }

    /// Get the base package segments.
    pub fn base_package_segments_translated(&self) -> Vec<String> {
        self.opts
            .base_package
            .split('.')
            .map(|s| self.to_snake_case(s))
            .collect()
    }

    /// Render a function signature with annotation (import or export).
    pub fn render_function(
        &mut self,
        resolve: &Resolve,
        func: &Function,
        is_import: bool,
        namespace: &str,
    ) -> String {
        let func_name = self.to_camel_case(&func.name);
        let wit_name = &func.name;

        // Generate scaladoc if docs exist
        let docs = format_docs(&func.docs);

        // Collect parameters
        let mut params = Vec::new();
        for param in &func.params {
            let scala_param_name = self.to_camel_case(&param.name);
            let scala_param_type = self.render_type(resolve, &param.ty);
            params.push((scala_param_name, scala_param_type));
        }

        // Render return type
        let return_type = func.result.as_ref().map(|ty| self.render_type(resolve, ty));

        if is_import {
            annotations::import_function(
                namespace,
                wit_name,
                &func_name,
                &params,
                return_type.as_deref(),
                &docs,
            )
        } else {
            annotations::export_function(
                namespace,
                wit_name,
                &func_name,
                &params,
                return_type.as_deref(),
                &docs,
            )
        }
    }
}

/// Scala keywords that need to be escaped.
struct ScalaKeywords {
    keywords: HashSet<&'static str>,
    object_methods: HashSet<&'static str>,
}

impl ScalaKeywords {
    fn new() -> Self {
        let mut keywords = HashSet::new();

        // Scala keywords (Scala 2 + future-proofing with Scala 3 keywords)
        keywords.extend([
            // Keywords
            "abstract",
            "case",
            "catch",
            "class",
            "def",
            "do",
            "else",
            "extends",
            "false",
            "final",
            "finally",
            "for",
            "forSome",
            "if",
            "implicit",
            "import",
            "lazy",
            "match",
            "new",
            "null",
            "object",
            "override",
            "package",
            "private",
            "protected",
            "return",
            "sealed",
            "super",
            "this",
            "throw",
            "trait",
            "true",
            "try",
            "type",
            "val",
            "var",
            "while",
            "with",
            "yield",
            // Scala 3 keywords (future-proofing)
            "enum",
            "export",
            "given",
            "then",
            // Soft keywords
            "as",
            "derives",
            "end",
            "extension",
            "infix",
            "inline",
            "opaque",
            "open",
            "transparent",
            "using",
            // Reserved words
            "_",
            ":",
            "=",
            "=>",
            "<-",
            "<:",
            "<%",
            ">:",
            "#",
            "@",
        ]);

        let mut object_methods = HashSet::new();
        // java.lang.Object methods that conflict even with backticks
        // These need underscore suffix like other languages binding (Rust, C/C++, and Moonbit)
        // because backticks don't prevent overriding.
        object_methods.extend([
            "equals",
            "hashCode",
            "toString",
            "wait",
            "notify",
            "notifyAll",
            "clone",
            "finalize",
            "getClass",
        ]);

        Self {
            keywords,
            object_methods,
        }
    }

    fn is_keyword(&self, name: &str) -> bool {
        self.keywords.contains(name)
    }

    fn is_object_method(&self, name: &str) -> bool {
        self.object_methods.contains(name)
    }
}
