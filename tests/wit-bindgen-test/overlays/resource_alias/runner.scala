package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import wit_component.test.resource_alias.e1.{Foo => Foo1, X, a => a1}
import wit_component.test.resource_alias.e2.{Foo => Foo2, a => a2}

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit = {
    a1(Foo1(X(42))).foreach(_.close())

    val y = X(8)
    val result = a2(Foo2(X(7)), Foo1(X(8)), y)
    result.foreach(_.close())
    y.close()
  }
}
