package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.many_arguments.ToTest

@WitImplementation
object TestComponent extends ToTest {
  override def manyArguments(
      a1: Long,
      a2: Long,
      a3: Long,
      a4: Long,
      a5: Long,
      a6: Long,
      a7: Long,
      a8: Long,
      a9: Long,
      a10: Long,
      a11: Long,
      a12: Long,
      a13: Long,
      a14: Long,
      a15: Long,
      a16: Long): Unit = {
    val actual = List(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16)
    val expected = (1L to 16L).toList
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
  }
}
