package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.resource_import_and_export.test.Thing

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    val thing1 = Thing(42)
    Assert.equal(thing1.foo(), 48)

    thing1.bar(33)
    Assert.equal(thing1.foo(), 43)

    val thing2 = Thing(81)
    val thing3 = Thing.baz(thing1, thing2)
    Assert.equal(
      thing3.foo(),
      33 + 3 + 3 + 81 + 1 + 1 + 2 + 2 + 4 + 1 + 2 + 4 + 1 + 1 + 2 + 2
    )
    thing3.close()
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
