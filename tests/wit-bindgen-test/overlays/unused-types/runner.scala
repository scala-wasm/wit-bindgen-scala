package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.foo.bar.component.foo

@WitImplementation
object Runner extends Root {
  override def run(): Unit =
    foo()
}
