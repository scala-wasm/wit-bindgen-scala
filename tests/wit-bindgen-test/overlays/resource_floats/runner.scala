package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.root.add
import wit_component.test.resource_floats.exports.{Float => ExportFloat}
import wit_component.test.resource_floats.test.{Float => TestFloat}

@WitImplementation
object Runner extends Root {
  override def run(): Unit = {
    val float3 = add(TestFloat(42.0), TestFloat(55.0))
    Assert.equal(float3.get(), 114.0)
    float3.close()

    val float = ExportFloat(22.0)
    Assert.equal(float.get(), 22.0 + 1.0 + 2.0 + 4.0 + 3.0)

    val result = ExportFloat.add(float, 7.0)
    Assert.equal(result.get(), 59.0)
    result.close()
  }
}

private object Assert {
  def equal[A](actual: A, expected: A): Unit =
    if (actual != expected)
      throw new RuntimeException(s"expected $expected, got $actual")
}
