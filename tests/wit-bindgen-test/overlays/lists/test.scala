package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import scala.scalajs.wit
import scala.scalajs.wit.unsigned.{UByte, UInt, ULong, UShort}

object TestComponent {
  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "allocated-bytes")
  def allocatedBytes(): UInt = 0

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "empty-list-param")
  def emptyListParam(@WitName("a") a: Array[UByte]): Unit =
    Assert.arrayEqual(a, Array.emptyByteArray)

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "empty-string-param")
  def emptyStringParam(@WitName("a") a: String): Unit =
    Assert.equal(a, "")

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "empty-list-result")
  def emptyListResult(): Array[UByte] =
    Array.emptyByteArray

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "empty-string-result")
  def emptyStringResult(): String =
    ""

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-param")
  def listParam(@WitName("a") a: Array[UByte]): Unit =
    Assert.arrayEqual(a, Array[Byte](1, 2, 3, 4))

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-param2")
  def listParam2(@WitName("a") a: String): Unit =
    Assert.equal(a, "foo")

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-param3")
  def listParam3(@WitName("a") a: Array[String]): Unit =
    Assert.arrayEqual(a, Array("foo", "bar", "baz"))

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-param4")
  def listParam4(@WitName("a") a: Array[Array[String]]): Unit = {
    Assert.equal(a.length, 2)
    Assert.arrayEqual(a(0), Array("foo", "bar"))
    Assert.arrayEqual(a(1), Array("baz"))
  }

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-param5")
  def listParam5(@WitName("a") a: Array[wit.Tuple3[UByte, UInt, UByte]]): Unit = {
    Assert.equal(a.length, 2)
    Assert.equal((a(0)._1, a(0)._2, a(0)._3), (1.toByte, 2, 3.toByte))
    Assert.equal((a(1)._1, a(1)._2, a(1)._3), (4.toByte, 5, 6.toByte))
  }

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-param-large")
  def listParamLarge(@WitName("a") a: Array[String]): Unit =
    Assert.equal(a.length, 1000)

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-result")
  def listResult(): Array[UByte] =
    Array[Byte](1, 2, 3, 4, 5)

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-result2")
  def listResult2(): String =
    "hello!"

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-result3")
  def listResult3(): Array[String] =
    Array("hello,", "world!")

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-roundtrip")
  def listRoundtrip(@WitName("a") a: Array[UByte]): Array[UByte] =
    a

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "string-roundtrip")
  def stringRoundtrip(@WitName("a") a: String): String =
    a

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "wasi-http-headers-roundtrip")
  def wasiHttpHeadersRoundtrip(
      @WitName("a") a: Array[wit.Tuple2[String, Array[UByte]]]
  ): Array[wit.Tuple2[String, Array[UByte]]] =
    a

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-minmax8")
  def listMinmax8(@WitName("a") a: Array[UByte], @WitName("b") b: Array[Byte]): wit.Tuple2[Array[UByte], Array[Byte]] =
    wit.Tuple2(a, b)

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-minmax16")
  def listMinmax16(@WitName("a") a: Array[UShort], @WitName("b") b: Array[Short]): wit.Tuple2[Array[UShort], Array[Short]] =
    wit.Tuple2(a, b)

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-minmax32")
  def listMinmax32(@WitName("a") a: Array[UInt], @WitName("b") b: Array[Int]): wit.Tuple2[Array[UInt], Array[Int]] =
    wit.Tuple2(a, b)

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-minmax64")
  def listMinmax64(@WitName("a") a: Array[ULong], @WitName("b") b: Array[Long]): wit.Tuple2[Array[ULong], Array[Long]] =
    wit.Tuple2(a, b)

  @WitExport(WitScope.unversioned("test", "lists", "to-test"), "list-minmax-float")
  def listMinmaxFloat(@WitName("a") a: Array[Float], @WitName("b") b: Array[Double]): wit.Tuple2[Array[Float], Array[Double]] =
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
