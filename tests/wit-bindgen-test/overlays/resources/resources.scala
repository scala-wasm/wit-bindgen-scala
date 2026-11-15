// wit-bindgen-scala: --export-impl-package=witbindgentest
// The generated `@WitExport` forwarders call this object and its classes.
package witbindgentest.test.resources.resources

import scala.scalajs.wit
import scala.scalajs.wit.annotation.{WitResourceExport, WitScope}
import scala.scalajs.wit.unsigned.UInt

import wit_component.test.resources.imports.Y

object Exports {
  @WitResourceExport(WitScope.inline("exports"), "x")
  final class X(var a: Int) {
    def getA(): Int = a
    def setA(a: Int): Unit = this.a = a
  }
  object X {
    def add(x: X, a: Int): X = new X(x.a + a)
  }

  @WitResourceExport(WitScope.inline("exports"), "z")
  final class Z(a: Int) {
    def getA(): Int = a
  }
  object Z {
    // The instances belong to the garbage collector, not to the handles:
    // there is no destructor callback to count drops with.
    def numDropped(): UInt = 0
  }

  @WitResourceExport(WitScope.inline("exports"), "kebab-case")
  final class KebabCase(a: UInt) {
    def getA(): UInt = a
  }
  object KebabCase {
    def takeOwned(k: KebabCase): UInt = k.getA()
  }

  def add(a: wit.Borrow[Z], b: wit.Borrow[Z]): Z = new Z(a.getA() + b.getA())

  def consume(x: X): Unit = ()

  def testImports(): wit.Result[Unit, String] = {
    val y = Y(10)
    if (y.getA() != 10) return new wit.Err("y.getA() != 10")
    y.setA(20)
    if (y.getA() != 20) return new wit.Err("y.getA() != 20")
    val y2 = Y.add(y, 20)
    if (y2.getA() != 40) return new wit.Err("Y.add != 40")
    y2.close()
    new wit.Ok(())
  }
}
