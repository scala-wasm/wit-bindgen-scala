package witbindgentest

import java.util.Optional

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.flavorful.to_test._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    fListInRecord1(ListInRecord1("list_in_record1"))
    Assert.equal(fListInRecord2().a, "list_in_record2")
    Assert.equal(fListInRecord3(ListInRecord3("list_in_record3 input")).a, "list_in_record3 output")
    Assert.equal(fListInRecord4(ListInRecord4("input4")).a, "result4")

    fListInVariant1(Optional.of("foo"), wit.Err("bar"))
    Assert.equal(fListInVariant2(), Optional.of("list_in_variant2"))
    Assert.equal(fListInVariant3(Optional.of("input3")), Optional.of("output3"))

    errnoResult()
    errnoResult()

    val typedefs = listTypedefs("typedef1", Array("typedef2"))
    Assert.arrayEqual(typedefs._1, "typedef3".getBytes("UTF-8"))
    Assert.arrayEqual(typedefs._2, Array("typedef4"))

    val variants = listOfVariants(
      Array(true, false),
      Array(wit.Ok(()), wit.Err(())),
      Array(MyErrno.Success, MyErrno.A)
    )
    Assert.arrayEqual(variants._1, Array(false, true))
    Assert.arrayEqual(variants._2, Array(wit.Err(()), wit.Ok(())))
    Assert.arrayEqual(variants._3, Array(MyErrno.A, MyErrno.B))
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")

  def arrayEqual[A](actual: Array[A], expected: Array[A]): Unit =
    if (!actual.sameElements(expected))
      throw new RuntimeException(s"expected ${expected.toList}, got ${actual.toList}")
}
