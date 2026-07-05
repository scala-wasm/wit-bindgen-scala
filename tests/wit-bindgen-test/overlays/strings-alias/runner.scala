package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.my.strings.cat._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    foo("hello")
    Assert.equal(bar(), "world")
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
