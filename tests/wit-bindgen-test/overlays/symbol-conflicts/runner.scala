package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    wit_component.my.inline.foo1.foo()
    wit_component.my.inline.foo2.foo()
    wit_component.my.inline.bar1.bar()
    wit_component.my.inline.bar2.bar()
  }
}
