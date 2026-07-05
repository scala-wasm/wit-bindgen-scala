package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.dep.{v0_1_0 => v1, v0_2_0 => v2}

@WitImplementation
object TestV1 extends v1.Test {
  override def x(): Float =
    1.0f

  override def y(a: Float): Float =
    1.0f + a
}

@WitImplementation
object TestV2 extends v2.Test {
  override def x(): Float =
    2.0f

  override def z(a: Float, b: Float): Float =
    2.0f + a + b
}
