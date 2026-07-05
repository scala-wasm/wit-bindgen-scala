package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.my.strings.Cat

@WitImplementation
object TestComponent extends Cat {
  override def foo(x: String): Unit =
    if (x != "hello")
      throw new RuntimeException(s"expected hello, got $x")

  override def bar(): String =
    "world"
}
