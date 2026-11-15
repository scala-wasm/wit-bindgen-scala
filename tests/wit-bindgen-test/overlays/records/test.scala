// wit-bindgen-scala: --export-impl-package=witbindgentest
// The generated `@WitExport` forwarders call this object (no annotations here).
package witbindgentest.test.records

import scala.scalajs.wit
import scala.scalajs.wit.unsigned.{UByte, UInt, UShort}

import wit_component.exports.test.records.to_test._

object ToTest {
  def multipleResults(): wit.Tuple2[UByte, UShort] =
    wit.Tuple2(4.toByte, 5.toShort)
  def swapTuple(a: wit.Tuple2[UByte, UInt]): wit.Tuple2[UInt, UByte] =
    wit.Tuple2(a._2, a._1)
  def roundtripFlags1(a: F1): F1 = a
  def roundtripFlags2(a: F2): F2 = a
  def roundtripFlags3(a: Flag8, b: Flag16, c: Flag32): wit.Tuple3[Flag8, Flag16, Flag32] =
    wit.Tuple3(a, b, c)
  def roundtripRecord1(a: R1): R1 = a
  def tuple1(a: wit.Tuple1[UByte]): wit.Tuple1[UByte] =
    wit.Tuple1(a._1)
}
