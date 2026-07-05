package witbindgentest

import java.util.Optional

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.variants.to_test._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    Assert.equal(roundtripOption(Optional.of(1.0f)), Optional.of(1.toByte))
    Assert.equal(roundtripOption(Optional.empty[Float]()), Optional.empty[Byte]())

    Assert.equal(roundtripResult(wit.Ok(2)), wit.Ok(2.0))
    Assert.equal(roundtripResult(wit.Err(5.3f)), wit.Err(5.toByte))

    Assert.equal(roundtripEnum(E1.A), E1.A)
    Assert.equal(invertBool(true), false)

    val casts = variantCasts(wit.Tuple6(
      C1.A(1),
      C2.A(2),
      C3.A(3),
      C4.A(4L),
      C5.A(5L),
      C6.A(6.0f)
    ))
    Assert.equal(casts._1, C1.A(1))
    Assert.equal(casts._6, C6.A(6.0f))

    val zeros = variantZeros(wit.Tuple4(Z1.B, Z2.B, Z3.B, Z4.B))
    Assert.equal(zeros._1, Z1.B)
    Assert.equal(zeros._4, Z4.B)

    variantTypedefs(Optional.empty[Int](), false, wit.Err(()))
    val enums = variantEnums(true, wit.Ok(()), MyErrno.Success)
    Assert.equal(enums._1, true)
    Assert.equal(enums._3, MyErrno.Success)
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
