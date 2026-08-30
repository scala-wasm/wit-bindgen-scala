package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitName, WitScope}

object TestV1 {
  @WitExport(WitScope("test", "dep", "test", "0.1.0"), "x")
  def x(): Float =
    1.0f

  @WitExport(WitScope("test", "dep", "test", "0.1.0"), "y")
  def y(@WitName("a") a: Float): Float =
    1.0f + a
}

object TestV2 {
  @WitExport(WitScope("test", "dep", "test", "0.2.0"), "x")
  def x(): Float =
    2.0f

  @WitExport(WitScope("test", "dep", "test", "0.2.0"), "z")
  def z(@WitName("a") a: Float, @WitName("b") b: Float): Float =
    2.0f + a + b
}
