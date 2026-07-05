package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.my.inline.{Bar1, Bar2, Foo1, Foo2}

@WitImplementation
object Foo1Impl extends Foo1 {
  override def foo(): Unit = ()
}

@WitImplementation
object Foo2Impl extends Foo2 {
  override def foo(): Unit = ()
}

@WitImplementation
object Bar1Impl extends Bar1 {
  override def bar(): String = ""
}

@WitImplementation
object Bar2Impl extends Bar2 {
  override def bar(): String = ""
}
