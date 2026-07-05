package witbindgentest

import java.util.Optional

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.options.ToTest

@WitImplementation
object TestComponent extends ToTest {
  override def optionNoneParam(a: Optional[String]): Unit =
    if (a.isPresent)
      throw new RuntimeException(s"expected empty option, got $a")

  override def optionSomeParam(a: Optional[String]): Unit =
    if (a != Optional.of("foo"))
      throw new RuntimeException(s"expected foo, got $a")

  override def optionNoneResult(): Optional[String] =
    Optional.empty[String]()

  override def optionSomeResult(): Optional[String] =
    Optional.of("foo")

  override def optionRoundtrip(a: Optional[String]): Optional[String] =
    a

  override def doubleOptionRoundtrip(a: Optional[Optional[Int]]): Optional[Optional[Int]] =
    a
}
