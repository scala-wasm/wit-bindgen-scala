package witbindgentest

import scala.scalajs.wit
import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.results.test._

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    Assert.equal(stringError(0.0f), wit.Err("zero"))
    Assert.equal(stringError(1.0f), wit.Ok(1.0f))

    Assert.equal(enumError(0.0f), wit.Err(E.A))
    Assert.equal(enumError(1.0f), wit.Ok(1.0f))

    ResultAssertions.recordErr(recordError(0.0f), 420, 0)
    ResultAssertions.recordErr(recordError(1.0f), 77, 2)
    Assert.equal(recordError(2.0f), wit.Ok(2.0f))

    ResultAssertions.variantErrE2(variantError(0.0f))
    ResultAssertions.variantErrE1(variantError(1.0f), E.B)
    ResultAssertions.variantErrE1(variantError(2.0f), E.C)

    Assert.equal(emptyError(0), wit.Err(()))
    Assert.equal(emptyError(1), wit.Ok(42))
    Assert.equal(emptyError(2), wit.Ok(2))

    Assert.equal(doubleError(0), wit.Ok(wit.Ok(())))
    Assert.equal(doubleError(1), wit.Ok(wit.Err("one")))
    Assert.equal(doubleError(2), wit.Err("two"))
  }
}

private object ResultAssertions {
  def recordErr(actual: wit.Result[Float, E2], line: Int, column: Int): Unit =
    actual match {
      case err: wit.Err[E2] =>
        Assert.equal(err.value.line, line)
        Assert.equal(err.value.column, column)
      case other => throw new RuntimeException(s"expected Err(E2($line, $column)), got $other")
    }

  def variantErrE1(actual: wit.Result[Float, E3], expected: E): Unit =
    actual match {
      case err: wit.Err[E3] =>
        err.value match {
          case value: E3.E1 => Assert.equal(value.value, expected)
          case other => throw new RuntimeException(s"expected E3.E1($expected), got $other")
        }
      case other => throw new RuntimeException(s"expected Err(E3.E1($expected)), got $other")
    }

  def variantErrE2(actual: wit.Result[Float, E3]): Unit =
    actual match {
      case err: wit.Err[E3] =>
        err.value match {
          case value: E3.E2 =>
            Assert.equal(value.value.line, 420)
            Assert.equal(value.value.column, 0)
          case other => throw new RuntimeException(s"expected E3.E2, got $other")
        }
      case other => throw new RuntimeException(s"expected Err(E3.E2), got $other")
    }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
