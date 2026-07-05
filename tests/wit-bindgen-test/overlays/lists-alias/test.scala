package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.my.lists.Cat

@WitImplementation
object TestComponent extends Cat {
  override def foo(x: Array[Byte]): Unit =
    Assert.arrayEqual(x, Bytes("hello"))

  override def bar(): Array[Byte] =
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
