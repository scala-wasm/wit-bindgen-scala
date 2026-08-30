package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}

object TestComponent {
  @WitExport(WitScope.inline("cat"), "foo")
  def foo(@WitName("x") x: String): Unit =
    if (x != "hello")
      throw new RuntimeException(s"expected hello, got $x")

  @WitExport(WitScope.inline("cat"), "bar")
  def bar(): String =
    "world"
}
