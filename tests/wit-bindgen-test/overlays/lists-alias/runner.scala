package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.my.lists.cat._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    foo(Bytes("hello"))
    Assert.arrayEqual(bar(), Bytes("world"))
  }
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
