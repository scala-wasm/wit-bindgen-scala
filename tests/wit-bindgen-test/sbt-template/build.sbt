import org.scalajs.linker.interface.ESVersion

ThisBuild / resolvers += "Sonatype Central Snapshots" at "https://central.sonatype.com/repository/maven-snapshots/"

ThisBuild / scalaVersion := "2.13.18"

lazy val root = project
  .in(file("."))
  .enablePlugins(ScalaJSPlugin)
  .settings(
    name := "wit-bindgen-test-scala-{{WORLD}}",
    scalaJSUseMainModuleInitializer := false,
    Compile / sourceGenerators := Nil,
    Test / sourceGenerators := Nil,
    Compile / unmanagedSourceDirectories += file("{{BINDINGS_DIR}}"),
    Compile / scalaJSLinkerConfig := {
      (Compile / scalaJSLinkerConfig).value
        .withPrettyPrint(true)
        .withESFeatures(_.withUseWebAssembly(true).withESVersion(ESVersion.ES2022))
        .withModuleKind(ModuleKind.WasmComponent)
        .withWasmFeatures(_.withWitDirectory(Some("{{WIT_DIR}}")).withWitWorld(Some("{{WORLD}}")))
    }
  )
