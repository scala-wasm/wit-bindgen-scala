package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import wit_component.a.b.the_test.x

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit =
    x()
}
