package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.resource_borrow_in_record.to_test._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    val thing1 = Thing("Bonjour")
    val thing2 = Thing("mon cher")
    val result = test(Array(Foo(thing1), Foo(thing2)))
    val strings = result.map(_.get()).toList

    result.foreach(_.close())
    thing1.close()
    thing2.close()

    Assert.equal(strings, List("Bonjour new test get", "mon cher new test get"))
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
