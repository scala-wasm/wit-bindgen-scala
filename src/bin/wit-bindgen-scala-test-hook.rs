use anyhow::{Context, Result, bail};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BASE_PACKAGE: &str = "wit_component";

fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        Some("prepare" | "verify") => Ok(()),
        Some("bindgen") => bindgen(),
        Some("compile") => compile(),
        Some(other) => bail!("unknown hook phase `{other}`"),
        None => bail!("usage: wit-bindgen-scala-test-hook prepare|bindgen|compile|verify"),
    }
}

fn bindgen() -> Result<()> {
    let wit = required_env_path("WIT")?;
    let bindings_dir = required_env_path("BINDINGS_DIR")?;
    let world = world_from_bindings_dir(&bindings_dir)?;

    let mut cmd = Command::new(current_exe_dir()?.join("wit-bindgen-scala"));
    cmd.arg(&wit)
        .arg("--world")
        .arg(&world)
        .arg("--out-dir")
        .arg(&bindings_dir)
        .arg("--base-package")
        .arg(BASE_PACKAGE);

    run(&mut cmd).context("failed to generate Scala bindings")
}

fn compile() -> Result<()> {
    let source = required_env_path("SOURCE")?;
    let artifacts_dir = required_env_path("ARTIFACTS_DIR")?;
    let output = required_env_path("OUTPUT")?;
    let bindings_dir = required_env_path("BINDINGS_DIR")?;
    let world = world_from_bindings_dir(&bindings_dir)?;

    let project_dir = artifacts_dir.join("sbt");
    let wit_dir = project_dir.join("wit");
    fs::create_dir_all(&wit_dir)?;
    let source_dir_for_wit = source.parent().context("SOURCE has no parent directory")?;
    fs::copy(
        source_dir_for_wit.join("test.wit"),
        wit_dir.join("package.wit"),
    )
    .context("failed to copy WIT package into sbt project")?;
    let deps_dir = source_dir_for_wit.join("deps");
    if deps_dir.is_dir() {
        copy_dir_all(&deps_dir, &wit_dir.join("deps"))
            .context("failed to copy WIT deps into sbt project")?;
    }

    write_sbt_project(&project_dir, &wit_dir, &bindings_dir, &world)?;

    let source_dir = project_dir.join("src/main/scala");
    fs::create_dir_all(&source_dir)?;
    fs::copy(&source, source_dir.join("Component.scala"))
        .context("failed to copy Scala source into sbt project")?;

    let log = artifacts_dir.join("sbt.log");
    let output_file = fs::File::create(&log).context("failed to create sbt log")?;
    let mut cmd = Command::new("sbt");
    cmd.current_dir(&project_dir)
        .arg("--batch")
        .arg("fastLinkJS")
        .stdin(Stdio::null())
        .stdout(output_file.try_clone()?)
        .stderr(output_file);
    let status = cmd.status().context("failed to spawn sbt")?;
    if !status.success() {
        let log = fs::read_to_string(&log).unwrap_or_default();
        bail!("sbt fastLinkJS failed with {status}\n{log}");
    }

    copy_linked_component(&project_dir, &output)
}

fn write_sbt_project(
    project_dir: &Path,
    wit_dir: &Path,
    bindings_dir: &Path,
    world: &str,
) -> Result<()> {
    copy_dir_all(
        &manifest_dir().join("tests/wit-bindgen-test/sbt-template"),
        project_dir,
    )?;

    render_template(
        &project_dir.join("build.sbt"),
        &[
            ("{{WIT_DIR}}", &escape_sbt_path(wit_dir)),
            ("{{BINDINGS_DIR}}", &escape_sbt_path(bindings_dir)),
            ("{{WORLD}}", world),
        ],
    )
}

fn render_template(path: &Path, replacements: &[(&str, &str)]) -> Result<()> {
    let mut contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    for (from, to) in replacements {
        contents = contents.replace(from, to);
    }
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src).with_context(|| format!("failed to read {}", src.display()))? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to).with_context(|| {
                format!("failed to copy {} to {}", from.display(), to.display())
            })?;
        }
    }
    Ok(())
}

fn copy_linked_component(project_dir: &Path, output: &Path) -> Result<()> {
    let mut matches = Vec::new();
    collect_wasm_files(&project_dir.join("target"), &mut matches)?;
    let wasm = matches
        .iter()
        .find(|path| path.file_name().and_then(OsStr::to_str) == Some("main.wasm"))
        .or_else(|| matches.first())
        .with_context(|| {
            format!(
                "no wasm output found under {}",
                project_dir.join("target").display()
            )
        })?;
    fs::copy(wasm, output)
        .with_context(|| format!("failed to copy {} to {}", wasm.display(), output.display()))?;
    Ok(())
}

fn collect_wasm_files(dir: &Path, matches: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_wasm_files(&path, matches)?;
        } else if path.extension().and_then(OsStr::to_str) == Some("wasm") {
            matches.push(path);
        }
    }
    matches.sort();
    Ok(())
}

fn world_from_bindings_dir(bindings_dir: &Path) -> Result<String> {
    let component_dir = bindings_dir
        .parent()
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .context("BINDINGS_DIR parent is not valid UTF-8")?;
    match component_dir.strip_suffix("-scala") {
        Some(world) if !world.is_empty() => Ok(world.to_string()),
        _ => bail!("unable to infer world from BINDINGS_DIR parent `{component_dir}`"),
    }
}

fn run(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("failed to spawn {cmd:?}"))?;
    if !status.success() {
        bail!("command {cmd:?} failed with {status}");
    }
    Ok(())
}

fn required_env_path(name: &str) -> Result<PathBuf> {
    env::var_os(name)
        .map(PathBuf::from)
        .with_context(|| format!("missing {name} environment variable"))
}

fn current_exe_dir() -> Result<PathBuf> {
    Ok(env::current_exe()?
        .parent()
        .context("current executable has no parent")?
        .to_path_buf())
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn escape_sbt_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}
