package witbindgentest

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.lists.to_test._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    emptyListParam(Array.emptyByteArray)
    emptyStringParam("")
    Assert.arrayEqual(emptyListResult(), Array.emptyByteArray)
    Assert.equal(emptyStringResult(), "")

    listParam(Array[Byte](1, 2, 3, 4))
    listParam2("foo")
    listParam3(Array("foo", "bar", "baz"))
    listParam4(Array(Array("foo", "bar"), Array("baz")))
    listParam5(Array(wit.Tuple3(1.toByte, 2, 3.toByte), wit.Tuple3(4.toByte, 5, 6.toByte)))
    listParamLarge(Array.fill(1000)("string"))

    Assert.arrayEqual(listResult(), Array[Byte](1, 2, 3, 4, 5))
    Assert.equal(listResult2(), "hello!")
    Assert.arrayEqual(listResult3(), Array("hello,", "world!"))

    Assert.arrayEqual(listRoundtrip(Array[Byte](1, 2, 3)), Array[Byte](1, 2, 3))
    Assert.equal(stringRoundtrip("hello ⚑ world"), "hello ⚑ world")

    val minmax = listMinmax32(Array(Int.MinValue, Int.MaxValue), Array(Int.MinValue, Int.MaxValue))
    Assert.arrayEqual(minmax._1, Array(Int.MinValue, Int.MaxValue))
    Assert.arrayEqual(minmax._2, Array(Int.MinValue, Int.MaxValue))
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")

  def arrayEqual[A](actual: Array[A], expected: Array[A]): Unit =
    if (!actual.sameElements(expected))
      throw new RuntimeException(s"expected ${expected.toList}, got ${actual.toList}")
}
