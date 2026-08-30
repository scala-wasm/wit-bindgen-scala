package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import wit_component.my.strings.cat._

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit = {
    foo("hello")
    Assert.equal(bar(), "world")
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
