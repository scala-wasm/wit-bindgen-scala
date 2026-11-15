// wit-bindgen-scala: --export-impl-package=witbindgentest
// The generated `@WitExport` forwarders call these objects and the class.
package witbindgentest.test.resource_floats

import scala.scalajs.wit
import scala.scalajs.wit.annotation.{WitResourceExport, WitScope}

import wit_component.test.resource_floats.{imports => imp}
import wit_component.test.resource_floats.test.{Float => TestFloat}

object Intermediate {
  def add(a: wit.Borrow[TestFloat], b: wit.Borrow[TestFloat]): TestFloat =
    TestFloat(a.get() + b.get() + 5.0)
}

package intermediate {
  object Exports {
    @WitResourceExport(WitScope.inline("exports"), "float")
    final class Float(private val value: imp.Float) {
      def this(v: Double) = this(imp.Float(v + 1.0))
      def get(): Double = value.get() + 3.0
    }
    object Float {
      def add(a: Float, b: Double): Float =
        new Float(imp.Float.add(a.value, b).get() + 5.0)
    }
  }
}
