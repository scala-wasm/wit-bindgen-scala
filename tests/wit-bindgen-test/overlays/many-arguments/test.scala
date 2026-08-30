package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import scala.scalajs.wit.unsigned.ULong

object TestComponent {
  @WitExport(WitScope.unversioned("test", "many-arguments", "to-test"), "many-arguments")
  def manyArguments(
      @WitName("a1") a1: ULong,
      @WitName("a2") a2: ULong,
      @WitName("a3") a3: ULong,
      @WitName("a4") a4: ULong,
      @WitName("a5") a5: ULong,
      @WitName("a6") a6: ULong,
      @WitName("a7") a7: ULong,
      @WitName("a8") a8: ULong,
      @WitName("a9") a9: ULong,
      @WitName("a10") a10: ULong,
      @WitName("a11") a11: ULong,
      @WitName("a12") a12: ULong,
      @WitName("a13") a13: ULong,
      @WitName("a14") a14: ULong,
      @WitName("a15") a15: ULong,
      @WitName("a16") a16: ULong
  ): Unit = {
    val actual = List(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16)
    val expected = (1L to 16L).toList
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
  }
}
