package witbindgentest

import java.util.Optional

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.variants.ToTest
import wit_component.exports.test.variants.to_test._

@WitImplementation
object TestComponent extends ToTest {
  override def roundtripOption(a: Optional[Float]): Optional[Byte] =
    if (a.isPresent) Optional.of(a.get().toByte) else Optional.empty[Byte]()

  override def roundtripResult(a: wit.Result[Int, Float]): wit.Result[Double, Byte] =
    a match {
      case ok: wit.Ok[Int] => wit.Ok(ok.value.toDouble)
      case err: wit.Err[Float] => wit.Err(err.value.toByte)
    }

  override def roundtripEnum(a: E1): E1 = a

  override def invertBool(a: Boolean): Boolean = !a

  override def variantCasts(a: wit.Tuple6[C1, C2, C3, C4, C5, C6]): wit.Tuple6[C1, C2, C3, C4, C5, C6] =
    a

  override def variantZeros(a: wit.Tuple4[Z1, Z2, Z3, Z4]): wit.Tuple4[Z1, Z2, Z3, Z4] =
    a

  override def variantTypedefs(a: Optional[Int], b: Boolean, c: wit.Result[Int, Unit]): Unit =
    ()

  override def variantEnums(a: Boolean, b: wit.Result[Unit, Unit], c: MyErrno): wit.Tuple3[Boolean, wit.Result[Unit, Unit], MyErrno] =
    wit.Tuple3(a, b, c)
}
