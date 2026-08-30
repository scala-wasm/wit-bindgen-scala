package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import wit_component.test.strings.to_test._

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit = {
    takeBasic("latin utf16")
    Assert.equal(returnUnicode(), "🚀🚀🚀 𠈄𓀀")
    Assert.equal(returnEmpty(), "")
    Assert.equal(roundtrip("🚀🚀🚀 𠈄𓀀"), "🚀🚀🚀 𠈄𓀀")
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
