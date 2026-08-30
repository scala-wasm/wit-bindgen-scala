package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import java.util.Optional
import scala.scalajs.wit
import scala.scalajs.wit.unsigned.{UByte, UInt}

import wit_component.exports.test.variants.to_test._

object TestComponent {
  @WitExport(WitScope.unversioned("test", "variants", "to-test"), "roundtrip-option")
  def roundtripOption(@WitName("a") a: Optional[Float]): Optional[UByte] =
    if (a.isPresent) Optional.of(a.get().toByte) else Optional.empty[UByte]()

  @WitExport(WitScope.unversioned("test", "variants", "to-test"), "roundtrip-result")
  def roundtripResult(@WitName("a") a: wit.Result[UInt, Float]): wit.Result[Double, UByte] =
    a match {
      case ok: wit.Ok[UInt] => wit.Ok(ok.value.toDouble)
      case err: wit.Err[Float] => wit.Err(err.value.toByte)
    }

  @WitExport(WitScope.unversioned("test", "variants", "to-test"), "roundtrip-enum")
  def roundtripEnum(@WitName("a") a: E1): E1 = a

  @WitExport(WitScope.unversioned("test", "variants", "to-test"), "invert-bool")
  def invertBool(@WitName("a") a: Boolean): Boolean = !a

  @WitExport(WitScope.unversioned("test", "variants", "to-test"), "variant-casts")
  def variantCasts(@WitName("a") a: Casts): Casts =
    a

  @WitExport(WitScope.unversioned("test", "variants", "to-test"), "variant-zeros")
  def variantZeros(@WitName("a") a: Zeros): Zeros =
    a

  @WitExport(WitScope.unversioned("test", "variants", "to-test"), "variant-typedefs")
  def variantTypedefs(@WitName("a") a: OptionTypedef, @WitName("b") b: BoolTypedef, @WitName("c") c: ResultTypedef): Unit =
    ()

  @WitExport(WitScope.unversioned("test", "variants", "to-test"), "variant-enums")
  def variantEnums(@WitName("a") a: Boolean, @WitName("b") b: wit.Result[Unit, Unit], @WitName("c") c: MyErrno): wit.Tuple3[Boolean, wit.Result[Unit, Unit], MyErrno] =
    wit.Tuple3(a, b, c)
}
