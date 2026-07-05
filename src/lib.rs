use anyhow::Result;
use std::collections::{HashMap, HashSet};
use wit_bindgen_core::{Files, WorldGenerator, wit_parser::*};

pub mod annotations;
pub mod context;
pub mod interface;
pub mod resource;
pub mod world;

pub use context::ScalaContext;

/// Specifies how a `--with` mapping should be handled.
#[derive(Debug, Clone)]
pub enum WithOption {
    /// Remap the interface to an external Scala package path.
    Path(String),
    /// Generate the interface normally (explicit opt-in).
    Generate,
}

/// Internal enum controlling whether an interface is generated or remapped.
#[derive(Debug, Clone)]
enum TypeGeneration {
    /// Skip generation; types are provided by an external Scala package (path stored in `with_map`).
    Remap,
    /// Generate the interface normally.
    Generate,
}

/// Parse a `--with` CLI argument of the form `key=value`.
///
/// - `key=generate` → `WithOption::Generate`
/// - `key=scala.package.path` → `WithOption::Path("scala.package.path")`
#[cfg(feature = "clap")]
fn parse_with(s: &str) -> std::result::Result<(String, WithOption), String> {
    let (key, value) = s
        .split_once('=')
        .ok_or_else(|| format!("expected `key=value` or `key=generate`, got `{}`", s))?;
    let key = key.trim().to_string();
    let value = value.trim();
    if key.is_empty() {
        return Err("key must not be empty".to_string());
    }
    if value == "generate" {
        Ok((key, WithOption::Generate))
    } else {
        Ok((key, WithOption::Path(value.to_string())))
    }
}

/// Strip the `@version` suffix from a WIT key.
///
/// `"wasi:cli/environment@0.2.0"` → `Some("wasi:cli/environment")`
/// `"wasi:cli/environment"` → `None`
pub(crate) fn strip_version(key: &str) -> Option<&str> {
    key.rfind('@').map(|i| &key[..i])
}

/// Strip both `/interface` and `@version` from a WIT key, leaving just the package.
///
/// `"wasi:cli/environment@0.2.0"` → `Some("wasi:cli")`
/// `"wasi:cli/environment"` → `Some("wasi:cli")`
/// `"wasi:cli"` → `None`
pub(crate) fn strip_interface(key: &str) -> Option<&str> {
    let without_version = strip_version(key).unwrap_or(key);
    without_version.rfind('/').map(|i| &without_version[..i])
}

/// Configuration options for the Scala bindings generator.
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {
    /// Base package for generated bindings (e.g., "com.example.wasm")
    #[cfg_attr(feature = "clap", arg(long, default_value = "componentmodel"))]
    pub base_package: String,

    /// Output directory for bindings
    #[cfg_attr(feature = "clap", arg(long))]
    pub binding_root: Option<String>,

    /// Generate unapply methods for pattern matching
    #[cfg_attr(feature = "clap", arg(
        long,
        default_value = "true",
        default_missing_value = "true",
        num_args = 0..=1,
        require_equals = true,
    ))]
    pub generate_unapply: bool,

    /// Remap WIT interfaces to pre-existing Scala packages or force generation.
    ///
    /// Each entry is `key=value` where key is a WIT interface name and value is
    /// either a Scala package path or the literal `generate`.
    ///
    /// Key formats (matched in order of specificity):
    /// - `wasi:cli/environment@0.2.0` — exact interface + version
    /// - `wasi:cli/environment` — any version of this interface
    /// - `wasi:cli` — all interfaces in the package (interface name auto-appended)
    #[cfg_attr(feature = "clap", arg(long = "with", value_parser = parse_with, value_name = "key=value"))]
    pub with: Vec<(String, WithOption)>,
}

impl Opts {
    pub fn build(&self) -> Box<dyn WorldGenerator> {
        Box::new(Scala::new(self.clone()))
    }
}

/// Main Scala bindings generator.
pub struct Scala {
    context: ScalaContext,
    imports: HashSet<InterfaceId>,
    exports: HashSet<InterfaceId>,
    world_import_funcs: Vec<(String, Function)>,
    world_export_funcs: Vec<(String, Function)>,
    with: HashMap<String, TypeGeneration>,
}

impl Scala {
    fn new(opts: Opts) -> Self {
        // Build internal with map from CLI options
        let mut with = HashMap::new();
        let mut with_map = HashMap::new();
        for (key, option) in &opts.with {
            match option {
                WithOption::Path(path) => {
                    with.insert(key.clone(), TypeGeneration::Remap);
                    with_map.insert(key.clone(), path.clone());
                }
                WithOption::Generate => {
                    with.insert(key.clone(), TypeGeneration::Generate);
                }
            }
        }

        let mut context = ScalaContext::new(&opts);
        context.set_with_map(with_map);

        Self {
            context,
            imports: HashSet::new(),
            exports: HashSet::new(),
            world_import_funcs: Vec::new(),
            world_export_funcs: Vec::new(),
            with,
        }
    }

    /// Look up how an interface should be generated. Defaults to `Generate`.
    ///
    /// Tries matching in order: exact → version-stripped → package-level.
    fn get_generation(&self, with_name: &str) -> &TypeGeneration {
        // Exact match (e.g., "wasi:cli/environment@0.2.0")
        if let Some(g) = self.with.get(with_name) {
            return g;
        }
        // Version-stripped match (e.g., "wasi:cli/environment")
        if let Some(stripped) = strip_version(with_name) {
            if let Some(g) = self.with.get(stripped) {
                return g;
            }
        }
        // Package-level match (e.g., "wasi:cli")
        if let Some(pkg) = strip_interface(with_name) {
            if let Some(g) = self.with.get(pkg) {
                return g;
            }
        }
        static DEFAULT: TypeGeneration = TypeGeneration::Generate;
        &DEFAULT
    }

    fn interface_name(resolve: &Resolve, name: &WorldKey, id: InterfaceId) -> String {
        resolve.interfaces[id]
            .name
            .clone()
            .unwrap_or_else(|| resolve.name_world_key(name))
    }

    /// Build the package namespace used for generated Scala file layout.
    fn build_package_namespace(
        &self,
        resolve: &Resolve,
        name: &WorldKey,
        id: InterfaceId,
    ) -> String {
        let interface = &resolve.interfaces[id];
        let interface_name = Self::interface_name(resolve, name, id);

        if let Some(package_id) = interface.package {
            let package = &resolve.packages[package_id];
            let pkg_name = &package.name;
            let version_segment = self
                .context
                .version_segment_if_needed(resolve, package_id)
                .map(|segment| format!("{segment}/"))
                .unwrap_or_default();
            if let Some(version) = &pkg_name.version {
                format!(
                    "{}:{}/{}{}@{}",
                    pkg_name.namespace, pkg_name.name, version_segment, interface_name, version
                )
            } else {
                format!(
                    "{}:{}/{}{}",
                    pkg_name.namespace, pkg_name.name, version_segment, &interface_name
                )
            }
        } else {
            resolve.name_world_key(name)
        }
    }

    /// Build the namespace used in WitImport/WitExport annotations.
    fn build_annotation_namespace(
        &self,
        resolve: &Resolve,
        name: &WorldKey,
        id: InterfaceId,
    ) -> String {
        if resolve.interfaces[id].name.is_none() {
            resolve.name_world_key(name)
        } else {
            let interface = &resolve.interfaces[id];
            let interface_name = Self::interface_name(resolve, name, id);
            if let Some(package_id) = interface.package {
                let package = &resolve.packages[package_id];
                let pkg_name = &package.name;
                if let Some(version) = &pkg_name.version {
                    format!(
                        "{}:{}/{}@{}",
                        pkg_name.namespace, pkg_name.name, interface_name, version
                    )
                } else {
                    format!(
                        "{}:{}/{}",
                        pkg_name.namespace, pkg_name.name, interface_name
                    )
                }
            } else {
                resolve.name_world_key(name)
            }
        }
    }
}

impl WorldGenerator for Scala {
    fn preprocess(&mut self, _resolve: &Resolve, _world: WorldId) {
        // No preprocessing needed
    }

    fn import_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        id: InterfaceId,
        files: &mut Files,
    ) -> Result<()> {
        self.imports.insert(id);

        let package_namespace = self.build_package_namespace(resolve, name, id);
        let annotation_namespace = self.build_annotation_namespace(resolve, name, id);
        let with_name = resolve.name_world_key(name);

        // Check if this interface is remapped to an external package
        match self.get_generation(&with_name) {
            TypeGeneration::Remap => {
                // Skip generation entirely — types and functions live in external lib
                return Ok(());
            }
            TypeGeneration::Generate => {
                // Fall through to normal generation
            }
        }

        let interface_name = Self::interface_name(resolve, name, id);

        // Generate interface content
        let content = interface::render_interface(
            &mut self.context,
            resolve,
            id,
            &interface_name,
            &package_namespace,
            &annotation_namespace,
            true, // is_import
        );

        // Get file path
        let file_path = interface::get_interface_file_path(
            &self.context,
            &package_namespace,
            &interface_name,
            true, // is_import
        );

        files.push(&file_path, content.as_bytes());

        Ok(())
    }

    fn import_funcs(
        &mut self,
        _resolve: &Resolve,
        _world: WorldId,
        funcs: &[(&str, &Function)],
        _files: &mut Files,
    ) {
        for (name, func) in funcs {
            self.world_import_funcs
                .push((name.to_string(), (*func).clone()));
        }
    }

    fn import_types(
        &mut self,
        _resolve: &Resolve,
        _world: WorldId,
        _types: &[(&str, TypeId)],
        _files: &mut Files,
    ) {
        // World-level types are handled in finish()
    }

    fn export_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        id: InterfaceId,
        files: &mut Files,
    ) -> Result<()> {
        self.exports.insert(id);

        let package_namespace = self.build_package_namespace(resolve, name, id);
        let annotation_namespace = self.build_annotation_namespace(resolve, name, id);
        let with_name = resolve.name_world_key(name);

        let interface_name = Self::interface_name(resolve, name, id);

        // Check if this interface is remapped to an external package
        let content = match self.get_generation(&with_name) {
            TypeGeneration::Remap => {
                // Generate trait only (no types) — type refs resolve to external path
                interface::render_export_trait_only(
                    &mut self.context,
                    resolve,
                    id,
                    &interface_name,
                    &package_namespace,
                    &annotation_namespace,
                )
            }
            TypeGeneration::Generate => {
                // Generate normally
                interface::render_interface(
                    &mut self.context,
                    resolve,
                    id,
                    &interface_name,
                    &package_namespace,
                    &annotation_namespace,
                    false, // is_import = false for exports
                )
            }
        };

        // Get file path
        let file_path = interface::get_interface_file_path(
            &self.context,
            &package_namespace,
            &interface_name,
            false, // is_import = false for exports
        );

        files.push(&file_path, content.as_bytes());

        Ok(())
    }

    fn export_funcs(
        &mut self,
        _resolve: &Resolve,
        _world: WorldId,
        funcs: &[(&str, &Function)],
        _files: &mut Files,
    ) -> Result<()> {
        for (name, func) in funcs {
            self.world_export_funcs
                .push((name.to_string(), (*func).clone()));
        }
        Ok(())
    }

    fn finish(&mut self, resolve: &Resolve, world_id: WorldId, files: &mut Files) -> Result<()> {
        let world = &resolve.worlds[world_id];
        let mut generated_count = self.imports.len() + self.exports.len();

        let has_world_imports = !self.world_import_funcs.is_empty()
            || world
                .imports
                .values()
                .any(|item| matches!(item, WorldItem::Type { .. }));

        if has_world_imports {
            if let Some(content) = world::render_world(
                &mut self.context,
                resolve,
                world_id,
                true, // is_import
                &self.world_import_funcs,
            ) {
                let file_path = world::get_world_file_path(&self.context, true);
                files.push(&file_path, content.as_bytes());
                generated_count += 1;
            }
        }

        let has_world_exports = !self.world_export_funcs.is_empty()
            || world
                .exports
                .values()
                .any(|item| matches!(item, WorldItem::Type { .. }));

        if has_world_exports {
            if let Some(content) = world::render_world(
                &mut self.context,
                resolve,
                world_id,
                false, // is_import = false for exports
                &self.world_export_funcs,
            ) {
                let file_path = world::get_world_file_path(&self.context, false);
                files.push(&file_path, content.as_bytes());
                generated_count += 1;
            }
        }

        eprintln!(
            "Generated {} Scala files ({} imports, {} exports)",
            generated_count,
            self.imports.len(),
            self.exports.len()
        );

        Ok(())
    }
}
