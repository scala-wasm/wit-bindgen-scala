package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import scala.scalajs.wit
import scala.scalajs.wit.unsigned.UInt

object TestComponent {
  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-none-param")
  def optionNoneParam(@WitName("a") a: wit.Option[String]): Unit =
    a match {
      case wit.None => ()
      case wit.Some(value) =>
        throw new RuntimeException(s"expected empty option, got $value")
    }

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-some-param")
  def optionSomeParam(@WitName("a") a: wit.Option[String]): Unit =
    a match {
      case wit.Some("foo") => ()
      case other =>
        throw new RuntimeException(s"expected foo, got $other")
    }

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-none-result")
  def optionNoneResult(): wit.Option[String] =
    wit.None

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-some-result")
  def optionSomeResult(): wit.Option[String] =
    wit.Some("foo")

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "option-roundtrip")
  def optionRoundtrip(@WitName("a") a: wit.Option[String]): wit.Option[String] =
    a.map(identity)

  @WitExport(WitScope.unversioned("test", "options", "to-test"), "double-option-roundtrip")
  def doubleOptionRoundtrip(@WitName("a") a: wit.Option[wit.Option[UInt]]): wit.Option[wit.Option[UInt]] =
    a.flatMap(inner => wit.Some(inner.map(identity)))
}
