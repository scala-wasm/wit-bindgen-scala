package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import wit_component.test.resource_borrow.to_test._

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit = {
    val thing = Thing(42)
    try Assert.equal(foo(thing), 45)
    finally thing.close()
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
