package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.my.inline.foo.Bar

@WitImplementation
object Runner extends Root {
  override def run(): Unit =
    Bar().close()
}
