package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}

object Foo1Impl {
  @WitExport(WitScope.unversioned("my", "inline", "foo1"), "foo")
  def foo(): Unit = ()
}

object Foo2Impl {
  @WitExport(WitScope.unversioned("my", "inline", "foo2"), "foo")
  def foo(): Unit = ()
}

object Bar1Impl {
  @WitExport(WitScope.unversioned("my", "inline", "bar1"), "bar")
  def bar(): String = ""
}

object Bar2Impl {
  @WitExport(WitScope.unversioned("my", "inline", "bar2"), "bar")
  def bar(): String = ""
}
