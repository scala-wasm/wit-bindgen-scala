package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import wit_component.foo.bar.component.foo

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit =
    foo()
}
