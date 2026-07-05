package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.resource_alias_redux.{resource_alias1 => a1, resource_alias2 => a2}
import wit_component.test.resource_alias_redux.the_test.test

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    val thing1 = a1.Thing("Ni Hao")
    val result1 = test(Array(thing1))
    Assert.equal(result1.length, 1)
    Assert.equal(result1(0).get(), "Ni Hao GuestThing GuestThing.get")
    result1.foreach(_.close())

    val thing2 = a1.Thing("Ciao")
    val result2 = a1.a(a1.Foo(thing2))
    Assert.equal(result2.length, 1)
    Assert.equal(result2(0).get(), "Ciao GuestThing GuestThing.get")
    result2.foreach(_.close())

    val thing3 = a1.Thing("Ciao")
    val thing4 = a1.Thing("Aloha")
    val result3 = a2.b(a2.Foo(thing3), a1.Foo(thing4))
    Assert.equal(result3.length, 2)
    Assert.equal(result3(0).get(), "Ciao GuestThing GuestThing.get")
    Assert.equal(result3(1).get(), "Aloha GuestThing GuestThing.get")
    result3.foreach(_.close())
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
