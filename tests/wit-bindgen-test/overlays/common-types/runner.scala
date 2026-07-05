package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.common.test_types._
import wit_component.test.common.to_test._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    val res = wrap(F1.a)
    Assert.equal(res.b, F1.a)
    Assert.equal(res.a, 1)

    val res2 = wrap(F1.b)
    Assert.equal(res2.b, F1.b)
    Assert.equal(res2.a, 2)

    varF() match {
      case value: V1.B => Assert.equal(value.value, 42)
      case other => throw new RuntimeException(s"expected V1.B(42), got $other")
    }
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
