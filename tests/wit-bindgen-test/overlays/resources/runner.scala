package witbindgentest

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.resources.exports._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    testImports() match {
      case _: wit.Ok[Unit] => ()
      case err: wit.Err[String] => throw new RuntimeException(err.value)
    }

    val x = X(5)
    Assert.equal(x.getA(), 5)
    x.setA(10)
    Assert.equal(x.getA(), 10)

    val z1 = Z(10)
    Assert.equal(z1.getA(), 10)
    val z2 = Z(20)
    Assert.equal(z2.getA(), 20)

    val xAdd = X.add(x, 5)
    Assert.equal(xAdd.getA(), 15)

    val zAdd = add(z1, z2)
    Assert.equal(zAdd.getA(), 30)
    zAdd.close()

    val droppedZsStart = Z.numDropped()
    z1.close()
    z2.close()

    consume(xAdd)

    val droppedZsEnd = Z.numDropped()
    if (droppedZsStart != 0)
      Assert.equal(droppedZsEnd, droppedZsStart + 2)
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
