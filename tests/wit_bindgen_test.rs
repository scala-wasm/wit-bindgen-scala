use anyhow::{Context, Result, bail};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use wit_bindgen_test::Opts;

const FIXTURE_FILTER_ENV: &str = "WIT_BINDGEN_SCALA_FIXTURES";

#[test]
fn rust_scala_interop_runtime_tests() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let overlay_root = root.join("tests/wit-bindgen-test/overlays");
    let hook = test_hook_path();
    let work = temp_work_dir()?;
    let tests = work.join("runtime");
    let artifacts = work.join("artifacts");
    fs::create_dir_all(&artifacts)?;

    let fixtures = overlay_fixtures(&overlay_root)?;
    let selected_fixtures = selected_fixtures(&fixtures)?;
    eprintln!(
        "running Rust/Scala interop fixtures: {}",
        selected_fixtures.join(",")
    );
    let fixture_dirs =
        prepare_fixtures(&root, &overlay_root, &tests, &fixtures, &selected_fixtures)?;

    let custom = format!("scala={}", hook.display());
    let artifacts_arg = artifacts.display().to_string();

    let mut args = vec!["wit-bindgen-test".to_string()];
    args.extend(fixture_dirs.iter().map(|dir| dir.display().to_string()));
    args.extend([
        "--artifacts".to_string(),
        artifacts_arg,
        "--languages".to_string(),
        "rust,scala".to_string(),
        "--custom".to_string(),
        custom,
        "--runner".to_string(),
        "wasmtime run -W gc,function-references,exceptions".to_string(),
    ]);

    let opts = Opts::parse_from(args);
    let result = opts.run(Path::new("wit-bindgen"));
    fs::remove_dir_all(&work).ok();
    result
}

fn test_hook_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_wit-bindgen-scala-test-hook"))
}

fn overlay_fixtures(overlay_root: &Path) -> Result<Vec<String>> {
    let mut fixtures = Vec::new();
    for entry in fs::read_dir(overlay_root)
        .with_context(|| format!("failed to read {}", overlay_root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let name = entry.file_name();
        let name = name
            .to_str()
            .with_context(|| format!("fixture directory name must be valid UTF-8: {name:?}"))?;
        fixtures.push(name.to_owned());
    }
    fixtures.sort();
    Ok(fixtures)
}

fn selected_fixtures(fixtures: &[String]) -> Result<Vec<String>> {
    let Some(filter) = std::env::var_os(FIXTURE_FILTER_ENV) else {
        return Ok(fixtures.to_vec());
    };
    let filter = filter
        .to_str()
        .with_context(|| format!("{FIXTURE_FILTER_ENV} must be valid UTF-8"))?;
    if filter.trim().is_empty() {
        return Ok(fixtures.to_vec());
    }

    let mut selected = Vec::new();
    for fixture in filter.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if !fixtures.iter().any(|active| active == fixture) {
            bail!("unknown fixture `{fixture}` in {FIXTURE_FILTER_ENV}");
        }
        selected.push(fixture.to_owned());
    }

    if selected.is_empty() {
        bail!("{FIXTURE_FILTER_ENV} selected no fixtures");
    }

    Ok(selected)
}

fn prepare_fixtures(
    root: &Path,
    overlay_root: &Path,
    tests_dir: &Path,
    all_fixtures: &[String],
    selected_fixtures: &[String],
) -> Result<Vec<PathBuf>> {
    let upstream_runtime = root.join("tests/upstream/wit-bindgen/tests/runtime");
    for fixture in all_fixtures {
        copy_dir_all(&upstream_runtime.join(fixture), &tests_dir.join(fixture))
            .with_context(|| format!("failed to copy upstream fixture `{fixture}`"))?;
    }

    let mut fixture_dirs = Vec::new();
    for fixture in selected_fixtures {
        let fixture = fixture.as_str();
        let upstream = upstream_runtime.join(fixture);
        let overlay = overlay_root.join(fixture);
        if let Some(fixture_dir) = prepare_fixture_direction(
            tests_dir,
            fixture,
            "rust-to-scala",
            &upstream,
            &overlay,
            &["test.scala".to_string()],
        )? {
            fixture_dirs.push(fixture_dir);
        }
        if let Some(fixture_dir) = prepare_fixture_direction(
            tests_dir,
            fixture,
            "scala-to-rust",
            &upstream,
            &overlay,
            &scala_to_rust_overlay_files(&overlay)?,
        )? {
            fixture_dirs.push(fixture_dir);
        }
    }

    Ok(fixture_dirs)
}

fn scala_to_rust_overlay_files(overlay: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in
        fs::read_dir(overlay).with_context(|| format!("failed to read {}", overlay.display()))?
    {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if name.ends_with(".scala") && name != "test.scala" {
            files.push(name.to_owned());
        }
    }
    files.sort();
    Ok(files)
}

fn prepare_fixture_direction(
    tests_dir: &Path,
    fixture: &str,
    direction: &str,
    upstream: &Path,
    overlay: &Path,
    selected_files: &[String],
) -> Result<Option<PathBuf>> {
    if selected_files
        .iter()
        .any(|file| file.ends_with(".scala") && !overlay.join(file).exists())
    {
        return Ok(None);
    }

    let fixture_dir = tests_dir.join(format!("{fixture}-{direction}"));
    copy_dir_all(upstream, &fixture_dir)
        .with_context(|| format!("failed to copy upstream fixture `{fixture}`"))?;

    for entry in fs::read_dir(&fixture_dir)
        .with_context(|| format!("failed to read {}", fixture_dir.display()))?
    {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };

        let is_replaced_rust_component = match direction {
            "rust-to-scala" => name.ends_with(".rs") && name.starts_with("test"),
            "scala-to-rust" => name.ends_with(".rs") && name.starts_with("runner"),
            _ => false,
        };
        if entry.file_type()?.is_file() && is_replaced_rust_component {
            fs::remove_file(entry.path())
                .with_context(|| format!("failed to remove {}", entry.path().display()))?;
        }
    }

    for file in selected_files {
        let src = if file.ends_with(".scala") {
            overlay.join(file)
        } else {
            upstream.join(file)
        };
        fs::copy(&src, fixture_dir.join(file))
            .with_context(|| format!("failed to copy `{}` for `{fixture}`", src.display()))?;
    }

    Ok(Some(fixture_dir))
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

fn temp_work_dir() -> Result<PathBuf> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!(
        "wit-bindgen-scala-harness-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir_all(&root)?;
    Ok(root)
}
