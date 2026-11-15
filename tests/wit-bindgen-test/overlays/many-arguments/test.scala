// wit-bindgen-scala: --export-impl-package=witbindgentest
// The generated `@WitExport` forwarders call this object (no annotations here).
package witbindgentest.test.many_arguments

import scala.scalajs.wit.unsigned.ULong

object ToTest {
  def manyArguments(
      a1: ULong,
      a2: ULong,
      a3: ULong,
      a4: ULong,
      a5: ULong,
      a6: ULong,
      a7: ULong,
      a8: ULong,
      a9: ULong,
      a10: ULong,
      a11: ULong,
      a12: ULong,
      a13: ULong,
      a14: ULong,
      a15: ULong,
      a16: ULong
  ): Unit = {
    val actual = List(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16)
    val expected = (1L to 16L).toList
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
  }
}
