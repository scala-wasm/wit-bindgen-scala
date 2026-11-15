//! Code generation checks for `--export-impl-package`: one `@WitExport`
//! forwarder per exported function, which calls the implementation object
//! that the naming convention gives.

use std::collections::BTreeMap;
use wit_bindgen_core::Files;
use wit_bindgen_core::wit_parser::Resolve;
use wit_bindgen_scala::Opts;

const WIT: &str = r#"
package my:pkg@1.0.0;

interface api {
  record point { x: s32, y: s32 }

  move: func(p: point, dx: s32) -> point;
  ping: func();
  get: async func(key: string) -> option<string>;
}

interface helper {
  noop: func();
}

world w {
  import helper;
  export api;
  export inline-api: interface {
    %type: func(a: u8) -> u8;
  }
  export run: func() -> s32;
}
"#;

const API_SCOPE: &str = r#"scala.scalajs.wit.annotation.WitScope("my", "pkg", "api", "1.0.0")"#;

/// Generated files by path.
fn generate(export_impl_package: Option<&str>) -> BTreeMap<String, String> {
    let mut resolve = Resolve::default();
    let pkg = resolve.push_str("w.wit", WIT).unwrap();
    let world = resolve.select_world(&[pkg], Some("w")).unwrap();
    let mut files = Files::default();
    Opts {
        base_package: "componentmodel".to_string(),
        export_impl_package: export_impl_package.map(str::to_string),
        ..Opts::default()
    }
    .build()
    .generate(&mut resolve, world, &mut files)
    .unwrap();
    files
        .iter()
        .map(|(path, bytes)| (path.to_string(), String::from_utf8(bytes.to_vec()).unwrap()))
        .collect()
}

fn file<'a>(files: &'a BTreeMap<String, String>, path: &str) -> &'a str {
    files
        .get(path)
        .unwrap_or_else(|| panic!("`{path}` not generated, only {:?}", files.keys()))
}

#[test]
fn interface_forwarders_call_the_interface_object() {
    let files = generate(Some("com.example"));
    let api = file(&files, "componentmodel/exports/my/pkg/api.scala");
    assert!(
        api.contains(&format!(
            "  @scala.scalajs.wit.annotation.WitExport({API_SCOPE}, \"move\")\n  \
             def move(@scala.scalajs.wit.annotation.WitName(\"p\") p: Point, \
             @scala.scalajs.wit.annotation.WitName(\"dx\") dx: Int): Point = \
             _root_.com.example.my.pkg.Api.move(p, dx)\n"
        )),
        "{api}"
    );
    assert!(
        api.contains("def ping(): Unit = _root_.com.example.my.pkg.Api.ping()\n"),
        "{api}"
    );
}

#[test]
fn async_forwarders_keep_witasync() {
    let files = generate(Some("com.example"));
    let api = file(&files, "componentmodel/exports/my/pkg/api.scala");
    assert!(
        api.contains(&format!(
            "  @scala.scalajs.wit.annotation.WitAsync\n  \
             @scala.scalajs.wit.annotation.WitExport({API_SCOPE}, \"get\")\n  \
             def get(@scala.scalajs.wit.annotation.WitName(\"key\") key: String): \
             scala.scalajs.wit.Option[String] = _root_.com.example.my.pkg.Api.get(key)\n"
        )),
        "{api}"
    );
}

#[test]
fn inline_interface_forwarders_call_an_object_under_the_world() {
    let files = generate(Some("com.example"));
    let inline = file(&files, "componentmodel/exports/my/pkg/inline_api.scala");
    assert!(
        inline.contains("= _root_.com.example.my.pkg.w.InlineApi.`type`(a)\n"),
        "{inline}"
    );
}

#[test]
fn world_forwarders_call_the_world_object() {
    let files = generate(Some("com.example"));
    let world = file(&files, "componentmodel/exports/package.scala");
    assert!(
        world.contains(
            "  @scala.scalajs.wit.annotation.WitExport(scala.scalajs.wit.annotation.WitScope.root, \"run\")\n  \
             def run(): Int = _root_.com.example.my.pkg.W.run()\n"
        ),
        "{world}"
    );
}

#[test]
fn imports_have_no_forwarders() {
    let files = generate(Some("com.example"));
    let helper = file(&files, "componentmodel/my/pkg/helper.scala");
    assert!(!helper.contains("WitExport"), "{helper}");
    assert!(!helper.contains("_root_.com.example"), "{helper}");
}

#[test]
fn without_the_option_nothing_is_generated_for_exported_functions() {
    let files = generate(None);
    assert!(
        !files.contains_key("componentmodel/exports/my/pkg/inline_api.scala"),
        "{:?}",
        files.keys()
    );
    assert!(
        !files.contains_key("componentmodel/exports/package.scala"),
        "{:?}",
        files.keys()
    );
    let api = file(&files, "componentmodel/exports/my/pkg/api.scala");
    assert!(!api.contains("WitExport"), "{api}");
    assert!(api.contains("final class Point"), "{api}");
}
