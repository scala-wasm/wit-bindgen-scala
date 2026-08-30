package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import java.nio.charset.StandardCharsets.UTF_8

import wit_component.test.resource_with_lists.test.Thing

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit = {
    val thing = Thing(Bytes("Hi"))

    Assert.arrayEqual(
      thing.foo(),
      Bytes("Hi Thing HostThing HostThing.foo Thing.foo")
    )

    thing.bar(Bytes("Hola"))

    Assert.arrayEqual(
      thing.foo(),
      Bytes("Hola Thing.bar HostThing.bar HostThing.foo Thing.foo")
    )

    Assert.arrayEqual(
      Thing.baz(Bytes("Ohayo Gozaimas")),
      Bytes("Ohayo Gozaimas Thing.baz HostThing.baz Thing.baz again")
    )

    thing.close()
  }
}

private object Bytes {
  def apply(value: String): Array[Byte] =
    value.getBytes(UTF_8)
}

private object Assert {
  def arrayEqual[A](actual: Array[A], expected: Array[A]): Unit =
    if (!actual.sameElements(expected))
      throw new RuntimeException(s"expected ${expected.mkString("[", ", ", "]")}, got ${actual.mkString("[", ", ", "]")}")
}
