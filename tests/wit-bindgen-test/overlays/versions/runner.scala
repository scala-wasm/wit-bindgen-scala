package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.dep.{v0_1_0 => v1, v0_2_0 => v2}

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    Assert.equal(v1.test.x(), 1.0f)
    Assert.equal(v1.test.y(1.0f), 2.0f)

    Assert.equal(v2.test.x(), 2.0f)
    Assert.equal(v2.test.z(1.0f, 1.0f), 4.0f)
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
