package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import scala.scalajs.wit
import scala.scalajs.wit.unsigned.{UByte, UInt, UShort}

import wit_component.exports.test.records.to_test._

object TestComponent {
  @WitExport(WitScope.unversioned("test", "records", "to-test"), "multiple-results")
  def multipleResults(): wit.Tuple2[UByte, UShort] =
    wit.Tuple2(4.toByte, 5.toShort)

  @WitExport(WitScope.unversioned("test", "records", "to-test"), "swap-tuple")
  def swapTuple(@WitName("a") a: wit.Tuple2[UByte, UInt]): wit.Tuple2[UInt, UByte] =
    wit.Tuple2(a._2, a._1)

  @WitExport(WitScope.unversioned("test", "records", "to-test"), "roundtrip-flags1")
  def roundtripFlags1(@WitName("a") a: F1): F1 = a
  @WitExport(WitScope.unversioned("test", "records", "to-test"), "roundtrip-flags2")
  def roundtripFlags2(@WitName("a") a: F2): F2 = a

  @WitExport(WitScope.unversioned("test", "records", "to-test"), "roundtrip-flags3")
  def roundtripFlags3(@WitName("a") a: Flag8, @WitName("b") b: Flag16, @WitName("c") c: Flag32): wit.Tuple3[Flag8, Flag16, Flag32] =
    wit.Tuple3(a, b, c)

  @WitExport(WitScope.unversioned("test", "records", "to-test"), "roundtrip-record1")
  def roundtripRecord1(@WitName("a") a: R1): R1 = a

  @WitExport(WitScope.unversioned("test", "records", "to-test"), "tuple1")
  def tuple1(@WitName("a") a: wit.Tuple1[UByte]): wit.Tuple1[UByte] =
    wit.Tuple1(a._1)
}
