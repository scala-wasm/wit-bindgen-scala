package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import wit_component.exports.my.strings.cat.MyString

object TestComponent {
  @WitExport(WitScope.inline("cat"), "foo")
  def foo(@WitName("x") x: MyString): Unit =
    if (x != "hello")
      throw new RuntimeException(s"expected hello, got $x")

  @WitExport(WitScope.inline("cat"), "bar")
  def bar(): MyString =
    "world"
}
