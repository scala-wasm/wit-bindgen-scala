//! Code generation checks for exported resources with
//! `--export-impl-package`: the user implements an exported resource with an
//! annotated class, and the generated `@WitExport` forwarders call it.

use std::collections::BTreeMap;
use wit_bindgen_core::Files;
use wit_bindgen_core::wit_parser::Resolve;
use wit_bindgen_scala::Opts;

const WIT: &str = r#"
package test:counters;

interface counters {
  resource counter {
    constructor(init: s32);
    get: func() -> s32;
    add: func(n: s32);
    merge: static func(a: borrow<counter>, b: counter) -> counter;
  }

  record counter-ref { c: borrow<counter> }

  make-counter: func(init: s32) -> counter;
  sum: func(refs: list<counter-ref>) -> s32;
}

world w {
  export counters;
}
"#;

const SCOPE: &str =
    r#"scala.scalajs.wit.annotation.WitScope.unversioned("test", "counters", "counters")"#;
const CLASS: &str = "_root_.com.example.test.counters.Counters.Counter";

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

fn counters() -> String {
    generate(Some("com.example"))
        .remove("componentmodel/exports/test/counters/counters.scala")
        .expect("counters.scala")
}

#[test]
fn constructor_creates_an_instance_of_the_class() {
    let src = counters();
    assert!(
        src.contains(&format!(
            "  @scala.scalajs.wit.annotation.WitExport({SCOPE}, \"[constructor]counter\")\n  \
             def counter$constructor(@scala.scalajs.wit.annotation.WitName(\"init\") init: Int): \
             {CLASS} = new {CLASS}(init)\n"
        )),
        "{src}"
    );
}

#[test]
fn methods_call_the_instance() {
    let src = counters();
    assert!(
        src.contains(&format!(
            "  @scala.scalajs.wit.annotation.WitExport({SCOPE}, \"[method]counter.add\")\n  \
             def counter$method$add(@scala.scalajs.wit.annotation.WitName(\"self\") self: \
             scala.scalajs.wit.Borrow[{CLASS}], @scala.scalajs.wit.annotation.WitName(\"n\") n: Int): \
             Unit = self.add(n)\n"
        )),
        "{src}"
    );
}

#[test]
fn static_functions_call_the_companion() {
    let src = counters();
    assert!(
        src.contains(&format!(
            "  @scala.scalajs.wit.annotation.WitExport({SCOPE}, \"[static]counter.merge\")\n  \
             def counter$static$merge(@scala.scalajs.wit.annotation.WitName(\"a\") a: \
             scala.scalajs.wit.Borrow[{CLASS}], @scala.scalajs.wit.annotation.WitName(\"b\") b: \
             {CLASS}): {CLASS} = {CLASS}.merge(a, b)\n"
        )),
        "{src}"
    );
}

#[test]
fn every_reference_is_the_user_class() {
    let src = counters();
    // A record field and a freestanding function use the class too.
    assert!(
        src.contains(&format!("val c: scala.scalajs.wit.Borrow[{CLASS}]")),
        "{src}"
    );
    assert!(
        src.contains(&format!(
            "def makeCounter(@scala.scalajs.wit.annotation.WitName(\"init\") init: Int): {CLASS} = \
             _root_.com.example.test.counters.Counters.makeCounter(init)\n"
        )),
        "{src}"
    );
    // No generated resource class: the user writes it.
    assert!(!src.contains("WitResourceImport"), "{src}");
    assert!(!src.contains("final class Counter "), "{src}");
    assert!(!src.contains("final class Counter("), "{src}");
}

#[test]
fn without_the_option_the_resource_is_skipped() {
    let files = generate(None);
    let src = files
        .get("componentmodel/exports/test/counters/counters.scala")
        .expect("counters.scala (the record)");
    assert!(!src.contains("[constructor]counter"), "{src}");
}
