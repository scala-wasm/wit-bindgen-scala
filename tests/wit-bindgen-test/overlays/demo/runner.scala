package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.a.b.the_test.x

@WitImplementation
object Runner extends Root {
  override def run(): Unit =
    x()
}
