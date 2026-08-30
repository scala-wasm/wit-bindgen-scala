package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import wit_component.my.inline.foo.Bar

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit =
    Bar().close()
}
