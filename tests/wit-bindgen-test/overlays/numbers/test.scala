package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import scala.scalajs.wit.unsigned.{UByte, UInt, ULong, UShort}

object TestComponent {
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-u8")
  def roundtripU8(@WitName("a") a: UByte): UByte = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-s8")
  def roundtripS8(@WitName("a") a: Byte): Byte = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-u16")
  def roundtripU16(@WitName("a") a: UShort): UShort = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-s16")
  def roundtripS16(@WitName("a") a: Short): Short = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-u32")
  def roundtripU32(@WitName("a") a: UInt): UInt = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-s32")
  def roundtripS32(@WitName("a") a: Int): Int = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-u64")
  def roundtripU64(@WitName("a") a: ULong): ULong = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-s64")
  def roundtripS64(@WitName("a") a: Long): Long = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-f32")
  def roundtripF32(@WitName("a") a: Float): Float = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-f64")
  def roundtripF64(@WitName("a") a: Double): Double = a
  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "roundtrip-char")
  def roundtripChar(@WitName("a") a: Char): Char = a

  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "set-scalar")
  def setScalar(@WitName("a") a: UInt): Unit =
    State.scalar = a

  @WitExport(WitScope.unversioned("test", "numbers", "numbers"), "get-scalar")
  def getScalar(): UInt =
    State.scalar
}

private object State {
  var scalar: UInt = 0
}
