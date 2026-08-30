package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import java.util.Optional
import scala.scalajs.wit.unsigned.UInt

object TestComponent {
  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-none-param")
  def optionNoneParam(@WitName("a") a: Optional[String]): Unit =
    if (a.isPresent)
      throw new RuntimeException(s"expected empty option, got $a")

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-some-param")
  def optionSomeParam(@WitName("a") a: Optional[String]): Unit =
    if (a != Optional.of("foo"))
      throw new RuntimeException(s"expected foo, got $a")

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-none-result")
  def optionNoneResult(): Optional[String] =
    Optional.empty[String]()

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-some-result")
  def optionSomeResult(): Optional[String] =
    Optional.of("foo")

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-roundtrip")
  def optionRoundtrip(@WitName("a") a: Optional[String]): Optional[String] =
    a

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "double-option-roundtrip")
  def doubleOptionRoundtrip(@WitName("a") a: Optional[Optional[UInt]]): Optional[Optional[UInt]] =
    a
}
