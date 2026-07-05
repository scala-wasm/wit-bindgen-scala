package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.strings.ToTest

@WitImplementation
object TestComponent extends ToTest {
  override def takeBasic(s: String): Unit =
    if (s != "latin utf16")
      throw new RuntimeException(s"expected latin utf16, got $s")

  override def returnUnicode(): String =
    "🚀🚀🚀 𠈄𓀀"

  override def returnEmpty(): String =
    ""

  override def roundtrip(s: String): String =
    s
}
