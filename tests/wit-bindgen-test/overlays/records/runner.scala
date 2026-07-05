package witbindgentest

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.records.to_test._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    val multiple = multipleResults()
    Assert.equal(multiple._1, 4.toByte)
    Assert.equal(multiple._2, 5.toShort)

    val swapped = swapTuple(wit.Tuple2(1.toByte, 2))
    Assert.equal(swapped._1, 2)
    Assert.equal(swapped._2, 1.toByte)

    Assert.equal(roundtripFlags1(F1.a | F1.b), F1.a | F1.b)
    Assert.equal(roundtripFlags2(F2.c | F2.e), F2.c | F2.e)

    val flags = roundtripFlags3(Flag8.b0, Flag16.b1, Flag32.b2)
    Assert.equal(flags._1, Flag8.b0)
    Assert.equal(flags._2, Flag16.b1)
    Assert.equal(flags._3, Flag32.b2)

    val record = roundtripRecord1(R1(8.toByte, F1.a))
    Assert.equal(record.a, 8.toByte)
    Assert.equal(record.b, F1.a)

    Assert.equal(tuple1(wit.Tuple1(1.toByte))._1, 1.toByte)
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
