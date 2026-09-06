package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import scala.scalajs.wit

import wit_component.test.options.to_test._

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit = {
    optionNoneParam(wit.None)
    optionSomeParam(wit.Some("foo"))

    Assert.equal(optionNoneResult(), wit.None)
    Assert.equal(optionSomeResult(), wit.Some("foo"))
    Assert.equal(optionRoundtrip(wit.Some("foo")), wit.Some("foo"))

    Assert.equal(doubleOptionRoundtrip(wit.Some(wit.Some(42))), wit.Some(wit.Some(42)))
    Assert.equal(doubleOptionRoundtrip(wit.Some(wit.None)), wit.Some(wit.None))
    Assert.equal(doubleOptionRoundtrip(wit.None), wit.None)
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
