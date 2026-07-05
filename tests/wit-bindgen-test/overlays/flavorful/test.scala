package witbindgentest

import java.util.Optional

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.test.flavorful.ToTest
import wit_component.exports.test.flavorful.to_test._

@WitImplementation
object TestComponent extends ToTest {
  override def fListInRecord1(a: ListInRecord1): Unit =
    Assert.equal(a.a, "list_in_record1")

  override def fListInRecord2(): ListInRecord2 =
    ListInRecord2("list_in_record2")

  override def fListInRecord3(a: ListInRecord3): ListInRecord3 = {
    Assert.equal(a.a, "list_in_record3 input")
    ListInRecord3("list_in_record3 output")
  }

  override def fListInRecord4(a: ListInRecord4): ListInRecord4 = {
    Assert.equal(a.a, "input4")
    ListInRecord4("result4")
  }

  override def fListInVariant1(a: Optional[String], b: wit.Result[Unit, String]): Unit = {
    Assert.equal(a, Optional.of("foo"))
    Assert.equal(b, wit.Err("bar"))
  }

  override def fListInVariant2(): Optional[String] =
    Optional.of("list_in_variant2")

  override def fListInVariant3(a: Optional[String]): Optional[String] = {
    Assert.equal(a, Optional.of("input3"))
    Optional.of("output3")
  }

  override def errnoResult(): wit.Result[Unit, MyErrno] =
    if (State.firstErrnoResult) {
      State.firstErrnoResult = false
      wit.Err(MyErrno.B)
    } else {
      wit.Ok(())
    }

  override def listTypedefs(a: String, c: Array[String]): wit.Tuple2[Array[Byte], Array[String]] = {
    Assert.equal(a, "typedef1")
    Assert.arrayEqual(c, Array("typedef2"))
    wit.Tuple2("typedef3".getBytes("UTF-8"), Array("typedef4"))
  }

  override def listOfVariants(
      a: Array[Boolean],
      b: Array[wit.Result[Unit, Unit]],
      c: Array[MyErrno]): wit.Tuple3[Array[Boolean], Array[wit.Result[Unit, Unit]], Array[MyErrno]] = {
    Assert.arrayEqual(a, Array(true, false))
    Assert.arrayEqual(b, Array(wit.Ok(()), wit.Err(())))
    Assert.arrayEqual(c, Array(MyErrno.Success, MyErrno.A))
    wit.Tuple3(Array(false, true), Array(wit.Err(()), wit.Ok(())), Array(MyErrno.A, MyErrno.B))
  }
}

private object State {
  var firstErrnoResult: Boolean = true
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")

  def arrayEqual[A](actual: Array[A], expected: Array[A]): Unit =
    if (!actual.sameElements(expected))
      throw new RuntimeException(s"expected ${expected.toList}, got ${actual.toList}")
}
