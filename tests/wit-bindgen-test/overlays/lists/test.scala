package witbindgentest

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.lists.ToTest

@WitImplementation
object TestComponent extends ToTest {
  override def allocatedBytes(): Int = 0

  override def emptyListParam(a: Array[Byte]): Unit =
    Assert.arrayEqual(a, Array.emptyByteArray)

  override def emptyStringParam(a: String): Unit =
    Assert.equal(a, "")

  override def emptyListResult(): Array[Byte] =
    Array.emptyByteArray

  override def emptyStringResult(): String =
    ""

  override def listParam(a: Array[Byte]): Unit =
    Assert.arrayEqual(a, Array[Byte](1, 2, 3, 4))

  override def listParam2(a: String): Unit =
    Assert.equal(a, "foo")

  override def listParam3(a: Array[String]): Unit =
    Assert.arrayEqual(a, Array("foo", "bar", "baz"))

  override def listParam4(a: Array[Array[String]]): Unit = {
    Assert.equal(a.length, 2)
    Assert.arrayEqual(a(0), Array("foo", "bar"))
    Assert.arrayEqual(a(1), Array("baz"))
  }

  override def listParam5(a: Array[wit.Tuple3[Byte, Int, Byte]]): Unit = {
    Assert.equal(a.length, 2)
    Assert.equal((a(0)._1, a(0)._2, a(0)._3), (1.toByte, 2, 3.toByte))
    Assert.equal((a(1)._1, a(1)._2, a(1)._3), (4.toByte, 5, 6.toByte))
  }

  override def listParamLarge(a: Array[String]): Unit =
    Assert.equal(a.length, 1000)

  override def listResult(): Array[Byte] =
    Array[Byte](1, 2, 3, 4, 5)

  override def listResult2(): String =
    "hello!"

  override def listResult3(): Array[String] =
    Array("hello,", "world!")

  override def listRoundtrip(a: Array[Byte]): Array[Byte] =
    a

  override def stringRoundtrip(a: String): String =
    a

  override def wasiHttpHeadersRoundtrip(
      a: Array[wit.Tuple2[String, Array[Byte]]]
  ): Array[wit.Tuple2[String, Array[Byte]]] =
    a

  override def listMinmax8(a: Array[Byte], b: Array[Byte]): wit.Tuple2[Array[Byte], Array[Byte]] =
    wit.Tuple2(a, b)

  override def listMinmax16(a: Array[Short], b: Array[Short]): wit.Tuple2[Array[Short], Array[Short]] =
    wit.Tuple2(a, b)

  override def listMinmax32(a: Array[Int], b: Array[Int]): wit.Tuple2[Array[Int], Array[Int]] =
    wit.Tuple2(a, b)

  override def listMinmax64(a: Array[Long], b: Array[Long]): wit.Tuple2[Array[Long], Array[Long]] =
    wit.Tuple2(a, b)

  override def listMinmaxFloat(a: Array[Float], b: Array[Double]): wit.Tuple2[Array[Float], Array[Double]] =
    wit.Tuple2(a, b)
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")

  def arrayEqual[A](actual: Array[A], expected: Array[A]): Unit =
    if (!actual.sameElements(expected))
      throw new RuntimeException(s"expected ${expected.toList}, got ${actual.toList}")
}
