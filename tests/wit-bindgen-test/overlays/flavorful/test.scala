package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}
import java.util.Optional

import scala.scalajs.wit
import wit_component.exports.test.flavorful.to_test._

object TestComponent {
  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "f-list-in-record1")
  def fListInRecord1(@WitName("a") a: ListInRecord1): Unit =
    Assert.equal(a.a, "list_in_record1")

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "f-list-in-record2")
  def fListInRecord2(): ListInRecord2 =
    ListInRecord2("list_in_record2")

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "f-list-in-record3")
  def fListInRecord3(@WitName("a") a: ListInRecord3): ListInRecord3 = {
    Assert.equal(a.a, "list_in_record3 input")
    ListInRecord3("list_in_record3 output")
  }

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "f-list-in-record4")
  def fListInRecord4(@WitName("a") a: ListInAlias): ListInAlias = {
    Assert.equal(a.a, "input4")
    ListInRecord4("result4")
  }

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "f-list-in-variant1")
  def fListInVariant1(@WitName("a") a: ListInVariant1V1, @WitName("b") b: ListInVariant1V2): Unit = {
    Assert.equal(a, Optional.of("foo"))
    Assert.equal(b, wit.Err("bar"))
  }

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "f-list-in-variant2")
  def fListInVariant2(): ListInVariant2 =
    Optional.of("list_in_variant2")

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "f-list-in-variant3")
  def fListInVariant3(@WitName("a") a: ListInVariant3): ListInVariant3 = {
    Assert.equal(a, Optional.of("input3"))
    Optional.of("output3")
  }

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "errno-result")
  def errnoResult(): wit.Result[Unit, MyErrno] =
    if (State.firstErrnoResult) {
      State.firstErrnoResult = false
      wit.Err(MyErrno.B)
    } else {
      wit.Ok(())
    }

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "list-typedefs")
  def listTypedefs(@WitName("a") a: ListTypedef, @WitName("c") c: ListTypedef3): wit.Tuple2[ListTypedef2, ListTypedef3] = {
    Assert.equal(a, "typedef1")
    Assert.arrayEqual(c, Array("typedef2"))
    wit.Tuple2("typedef3".getBytes("UTF-8"), Array("typedef4"))
  }

  @WitExport(WitScope.unversioned("test", "flavorful", "to-test"), "list-of-variants")
  def listOfVariants(
      @WitName("a") a: Array[Boolean],
      @WitName("b") b: Array[wit.Result[Unit, Unit]],
      @WitName("c") c: Array[MyErrno]
  ): wit.Tuple3[Array[Boolean], Array[wit.Result[Unit, Unit]], Array[MyErrno]] = {
    Assert.arrayEqual(a, Array(true, false))
    Assert.arrayEqual(b, Array(wit.Ok(()), wit.Err(())))
    Assert.arrayEqual(c, Array[MyErrno](MyErrno.Success, MyErrno.A))
    wit.Tuple3(
      Array(false, true),
      Array(wit.Err(()), wit.Ok(())),
      Array[MyErrno](MyErrno.A, MyErrno.B)
    )
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
