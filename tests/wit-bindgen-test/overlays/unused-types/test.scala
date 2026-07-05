package witbindgentest

import scala.scalajs.wit.annotation.WitImplementation
import wit_component.exports.foo.bar.Component

@WitImplementation
object TestComponent extends Component {
  override def foo(): Unit = ()
}
