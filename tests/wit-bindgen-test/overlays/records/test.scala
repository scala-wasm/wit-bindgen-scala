package witbindgentest

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.records.ToTest
import wit_component.exports.test.records.to_test._

@WitImplementation
object TestComponent extends ToTest {
  override def multipleResults(): wit.Tuple2[Byte, Short] =
    wit.Tuple2(4.toByte, 5.toShort)

  override def swapTuple(a: wit.Tuple2[Byte, Int]): wit.Tuple2[Int, Byte] =
    wit.Tuple2(a._2, a._1)

  override def roundtripFlags1(a: F1): F1 = a
  override def roundtripFlags2(a: F2): F2 = a

  override def roundtripFlags3(a: Flag8, b: Flag16, c: Flag32): wit.Tuple3[Flag8, Flag16, Flag32] =
    wit.Tuple3(a, b, c)

  override def roundtripRecord1(a: R1): R1 = a

  override def tuple1(a: wit.Tuple1[Byte]): wit.Tuple1[Byte] =
    wit.Tuple1(a._1)
}
