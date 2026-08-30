package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}

object TestComponent {
  @WitExport(WitScope.unversioned("foo", "bar", "component"), "foo")
  def foo(): Unit = ()
}
