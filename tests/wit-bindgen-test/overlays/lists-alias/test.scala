package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import wit_component.exports.my.lists.cat.MyList

object TestComponent {
  @WitExport(WitScope.inline("cat"), "foo")
  def foo(@WitName("x") x: MyList): Unit =
    Assert.arrayEqual(x, Bytes("hello"))

  @WitExport(WitScope.inline("cat"), "bar")
  def bar(): MyList =
    Bytes("world")
}

private object Bytes {
  def apply(value: String): Array[Byte] =
    value.getBytes("UTF-8")
}

private object Assert {
  def arrayEqual[A](actual: Array[A], expected: Array[A]): Unit =
    if (!actual.sameElements(expected))
      throw new RuntimeException(s"expected ${expected.mkString("[", ", ", "]")}, got ${actual.mkString("[", ", ", "]")}")
}
