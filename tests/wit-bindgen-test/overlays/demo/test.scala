package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}

object TestComponent {
  @WitExport(WitScope.unversioned("a", "b", "the-test"), "x")
  def x(): Unit = ()
}
