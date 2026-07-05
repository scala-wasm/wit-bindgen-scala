package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.numbers.numbers._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    Assert.equal(roundtripU8(1.toByte), 1.toByte)
    Assert.equal(roundtripS8(1.toByte), 1.toByte)
    Assert.equal(roundtripU16(2.toShort), 2.toShort)
    Assert.equal(roundtripS16(2.toShort), 2.toShort)
    Assert.equal(roundtripU32(3), 3)
    Assert.equal(roundtripS32(3), 3)
    Assert.equal(roundtripU64(4L), 4L)
    Assert.equal(roundtripS64(4L), 4L)
    Assert.equal(roundtripF32(5.0f), 5.0f)
    Assert.equal(roundtripF64(6.0), 6.0)
    Assert.equal(roundtripChar('a'), 'a')

    setScalar(42)
    Assert.equal(getScalar(), 42)
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
