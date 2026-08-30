package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import java.util.Optional

import wit_component.test.options.to_test._

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit = {
    optionNoneParam(Optional.empty[String]())
    optionSomeParam(Optional.of("foo"))

    Assert.equal(optionNoneResult(), Optional.empty[String]())
    Assert.equal(optionSomeResult(), Optional.of("foo"))
    Assert.equal(optionRoundtrip(Optional.of("foo")), Optional.of("foo"))

    Assert.equal(doubleOptionRoundtrip(Optional.of(Optional.of(42))), Optional.of(Optional.of(42)))
    Assert.equal(doubleOptionRoundtrip(Optional.of(Optional.empty[Int]())), Optional.of(Optional.empty[Int]()))
    Assert.equal(doubleOptionRoundtrip(Optional.empty[Optional[Int]]()), Optional.empty[Optional[Int]]())
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
