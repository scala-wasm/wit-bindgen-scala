package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit = {
    wit_component.my.inline.foo1.foo()
    wit_component.my.inline.foo2.foo()
    wit_component.my.inline.bar1.bar()
    wit_component.my.inline.bar2.bar()
  }
}
