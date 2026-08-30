package witbindgentest

import scala.scalajs.wit.annotation.{WitExport, WitScope}
import wit_component.test.many_arguments.to_test.manyArguments

object Runner {
  @WitExport(WitScope.root, "run")
  def run(): Unit =
    manyArguments(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L, 9L, 10L, 11L, 12L, 13L, 14L, 15L, 16L)
}
