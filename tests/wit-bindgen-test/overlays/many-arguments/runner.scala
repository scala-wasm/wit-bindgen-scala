package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.Root
import wit_component.test.many_arguments.to_test.manyArguments

@WitImplementation
object Runner extends Root {
  override def run(): Unit =
    manyArguments(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L, 9L, 10L, 11L, 12L, 13L, 14L, 15L, 16L)
}
