// wit-bindgen-scala: --export-impl-package=witbindgentest
// The generated `@WitExport` forwarders call this object and class.
package witbindgentest.test.resource_borrow_in_record

import scala.scalajs.wit.annotation.{WitResourceExport, WitScope}

import wit_component.exports.test.resource_borrow_in_record.to_test.Foo

object ToTest {
  @WitResourceExport(WitScope.unversioned("test", "resource-borrow-in-record", "to-test"), "thing")
  final class Thing(s: String) {
    var contents: String = s"$s new"
    def get(): String = s"$contents get"
  }

  def test(a: Array[Foo]): Array[Thing] =
    a.map { foo =>
      val thing = new Thing("")
      thing.contents = s"${foo.thing.contents} test"
      thing
    }
}
