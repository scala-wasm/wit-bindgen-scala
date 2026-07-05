package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.a.b.TheTest

@WitImplementation
object TestComponent extends TheTest {
  override def x(): Unit = ()
}
