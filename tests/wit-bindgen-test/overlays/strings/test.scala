package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}

object TestComponent {
  @WitExport(WitScope.unversioned("test", "strings", "to-test"), "take-basic")
  def takeBasic(@WitName("s") s: String): Unit =
    if (s != "latin utf16")
      throw new RuntimeException(s"expected latin utf16, got $s")

  @WitExport(WitScope.unversioned("test", "strings", "to-test"), "return-unicode")
  def returnUnicode(): String =
    "🚀🚀🚀 𠈄𓀀"

  @WitExport(WitScope.unversioned("test", "strings", "to-test"), "return-empty")
  def returnEmpty(): String =
    ""

  @WitExport(WitScope.unversioned("test", "strings", "to-test"), "roundtrip")
  def roundtrip(@WitName("s") s: String): String =
    s
}
