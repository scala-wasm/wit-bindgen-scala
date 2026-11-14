//! Code generation checks for WIT `async func`.
//!
//! scala-wasm lowers and lifts these functions with the synchronous Canonical
//! ABI. The bindings only add `@WitAsync`, so that the linker writes
//! `async func` in the component type.

use wit_bindgen_core::Files;
use wit_bindgen_core::wit_parser::Resolve;
use wit_bindgen_scala::Opts;

const WIT: &str = r#"
package test:async-gen;

interface api {
  resource conn {
    constructor();
    send: async func(data: list<u8>);
    id: func() -> u32;
    open: static async func(url: string) -> conn;
    count: static func() -> u32;
  }

  get: async func(key: string) -> option<string>;
  put: func(key: string);
}

world w {
  import api;

  export run: async func() -> u32;
  export ping: func();
}
"#;

fn generate() -> String {
    let mut resolve = Resolve::default();
    let pkg = resolve.push_str("async-gen.wit", WIT).unwrap();
    let world = resolve.select_world(&[pkg], Some("w")).unwrap();
    let mut files = Files::default();
    Opts {
        base_package: "componentmodel".to_string(),
        ..Opts::default()
    }
    .build()
    .generate(&mut resolve, world, &mut files)
    .unwrap();
    files
        .iter()
        .map(|(_, bytes)| String::from_utf8(bytes.to_vec()).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The annotation line that comes directly before `line`, if any.
fn line_before<'a>(src: &'a str, line: &str) -> Option<&'a str> {
    let lines: Vec<&str> = src.lines().map(str::trim).collect();
    let i = lines
        .iter()
        .position(|l| l.starts_with(line))
        .unwrap_or_else(|| panic!("`{line}` not found in:\n{src}"));
    i.checked_sub(1).map(|j| lines[j])
}

const WIT_ASYNC: &str = "@scala.scalajs.wit.annotation.WitAsync";

#[test]
fn async_import_function() {
    let src = generate();
    assert_eq!(
        line_before(&src, "def get("),
        Some(
            "@scala.scalajs.wit.annotation.WitImport(\
             scala.scalajs.wit.annotation.WitScope.unversioned(\"test\", \"async-gen\", \"api\"), \"get\")"
        )
    );
    assert_eq!(
        line_before(
            &src,
            "@scala.scalajs.wit.annotation.WitImport(\
             scala.scalajs.wit.annotation.WitScope.unversioned(\"test\", \"async-gen\", \"api\"), \"get\")"
        ),
        Some(WIT_ASYNC)
    );
    assert_ne!(
        line_before(
            &src,
            "@scala.scalajs.wit.annotation.WitImport(\
             scala.scalajs.wit.annotation.WitScope.unversioned(\"test\", \"async-gen\", \"api\"), \"put\")"
        ),
        Some(WIT_ASYNC)
    );
}

#[test]
fn async_resource_methods_stay_in_the_resource() {
    let src = generate();
    assert_eq!(
        line_before(
            &src,
            "@scala.scalajs.wit.annotation.WitResourceMethod(\"send\")"
        ),
        Some(WIT_ASYNC)
    );
    assert_eq!(
        line_before(
            &src,
            "@scala.scalajs.wit.annotation.WitResourceStaticMethod(\"open\")"
        ),
        Some(WIT_ASYNC)
    );
    assert_ne!(
        line_before(
            &src,
            "@scala.scalajs.wit.annotation.WitResourceMethod(\"id\")"
        ),
        Some(WIT_ASYNC)
    );
    assert_ne!(
        line_before(
            &src,
            "@scala.scalajs.wit.annotation.WitResourceStaticMethod(\"count\")"
        ),
        Some(WIT_ASYNC)
    );
    // Async methods are not also generated as freestanding imports.
    assert!(!src.contains("\"[method]conn.send\""), "{src}");
    assert!(!src.contains("\"[static]conn.open\""), "{src}");
    assert_eq!(src.matches("def send(").count(), 1, "{src}");
    assert_eq!(src.matches("def `open`(").count(), 1, "{src}");
}
