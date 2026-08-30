package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import wit_component.test.common.test_types._

object Leaf {
  @WitExport(WitScope.unversioned("test", "common", "to-test"), "wrap")
  def wrap(@WitName("flag") flag: F1): R1 =
    if (flag == F1.a) R1(1, flag) else R1(2, flag)

  @WitExport(WitScope.unversioned("test", "common", "to-test"), "var-f")
  def varF(): V1 =
    V1.B(42)
}
