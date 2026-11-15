// wit-bindgen-scala: --export-impl-package=witbindgentest
// The generated `@WitExport` forwarders call this object and class.
package witbindgentest.test.resource_borrow

import scala.scalajs.wit
import scala.scalajs.wit.annotation.{WitResourceExport, WitScope}
import scala.scalajs.wit.unsigned.UInt

object ToTest {
  @WitResourceExport(WitScope.unversioned("test", "resource-borrow", "to-test"), "thing")
  final class Thing(v: UInt) {
    val value: UInt = v + 1
  }

  def foo(v: wit.Borrow[Thing]): UInt = v.value + 2
}
