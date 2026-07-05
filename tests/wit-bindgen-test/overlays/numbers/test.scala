package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.numbers.Numbers

@WitImplementation
object TestComponent extends Numbers {
  override def roundtripU8(a: Byte): Byte = a
  override def roundtripS8(a: Byte): Byte = a
  override def roundtripU16(a: Short): Short = a
  override def roundtripS16(a: Short): Short = a
  override def roundtripU32(a: Int): Int = a
  override def roundtripS32(a: Int): Int = a
  override def roundtripU64(a: Long): Long = a
  override def roundtripS64(a: Long): Long = a
  override def roundtripF32(a: Float): Float = a
  override def roundtripF64(a: Double): Double = a
  override def roundtripChar(a: Char): Char = a

  override def setScalar(a: Int): Unit =
    State.scalar = a

  override def getScalar(): Int =
    State.scalar
}

private object State {
  var scalar: Int = 0
}
