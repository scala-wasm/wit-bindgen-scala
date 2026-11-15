// wit-bindgen-scala: --export-impl-package=witbindgentest
// The generated `@WitExport` forwarders call this object (no annotations here).
package witbindgentest.test.numbers

import scala.scalajs.wit.unsigned.{UByte, UInt, ULong, UShort}

object Numbers {
  def roundtripU8(a: UByte): UByte = a
  def roundtripS8(a: Byte): Byte = a
  def roundtripU16(a: UShort): UShort = a
  def roundtripS16(a: Short): Short = a
  def roundtripU32(a: UInt): UInt = a
  def roundtripS32(a: Int): Int = a
  def roundtripU64(a: ULong): ULong = a
  def roundtripS64(a: Long): Long = a
  def roundtripF32(a: Float): Float = a
  def roundtripF64(a: Double): Double = a
  def roundtripChar(a: Char): Char = a
  def setScalar(a: UInt): Unit =
    State.scalar = a
  def getScalar(): UInt =
    State.scalar
}

private object State {
  var scalar: UInt = 0
}
