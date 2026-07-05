package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.common.ToTest
import wit_component.test.common.test_types._

@WitImplementation
object Leaf extends ToTest {
  override def wrap(flag: F1): R1 =
    if (flag == F1.a) R1(1, flag) else R1(2, flag)

  override def varF(): V1 =
    V1.B(42)
}
