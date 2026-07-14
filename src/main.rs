use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::PathBuf;
use std::str;
use wit_bindgen_core::{Files, wit_parser::Resolve};

/// Generate Scala bindings for WIT packages targeting scala-wasm.
#[derive(Debug, Parser)]
#[command(version)]
struct Opt {
    #[clap(flatten)]
    scala: wit_bindgen_scala::Opts,

    #[clap(flatten)]
    common: Common,
}

#[derive(Debug, Parser)]
struct Common {
    /// Where to place output files.
    #[clap(long = "out-dir")]
    out_dir: Option<PathBuf>,

    /// Where to place WIT files carried by generated bindings.
    #[clap(long = "wit-out-dir")]
    wit_out_dir: Option<PathBuf>,

    /// Locations of WIT file(s) to generate bindings for.
    ///
    /// These paths can be directories containing `*.wit` files, `*.wit` files,
    /// or `*.wasm` files with wasm-encoded WIT packages.
    #[clap(value_name = "WIT", index = 1)]
    wit: Vec<PathBuf>,

    /// Optionally specified world that bindings are generated for.
    #[clap(short, long)]
    world: Option<String>,

    /// Check whether output files are up-to-date without writing them.
    #[clap(long)]
    check: bool,

    /// Comma-separated list of WIT features to enable.
    #[clap(long)]
    features: Vec<String>,

    /// Activate all WIT features.
    #[clap(long)]
    all_features: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::parse();
    let mut files = Files::default();
    let mut generator = opt.scala.build();

    let mut resolve = Resolve::default();
    resolve.all_features = opt.common.all_features;
    for features in opt.common.features.iter() {
        for feature in features
            .split(',')
            .flat_map(|s| s.split_whitespace())
            .filter(|f| !f.is_empty())
        {
            resolve.features.insert(feature.to_string());
        }
    }

    let mut main_packages = Vec::new();
    for wit in &opt.common.wit {
        let (pkg, _files) = resolve.push_path(wit)?;
        main_packages.push(pkg);
    }
    let world = resolve.select_world(&main_packages, opt.common.world.as_deref())?;
    generator.generate(&mut resolve, world, &mut files)?;

    write_files(&opt.common, &files)?;
    write_wit_files(&opt.common, &opt.common.wit, &resolve.worlds[world].name)
}

fn write_files(opt: &Common, files: &Files) -> Result<()> {
    for (name, contents) in files.iter() {
        let dst = match &opt.out_dir {
            Some(path) => path.join(name),
            None => name.into(),
        };
        eprintln!("Generating {:?}", dst);

        if opt.check {
            let prev = std::fs::read(&dst).with_context(|| format!("failed to read {:?}", dst))?;
            if prev != contents {
                if let (Ok(utf8_prev), Ok(utf8_contents)) =
                    (str::from_utf8(&prev), str::from_utf8(contents))
                {
                    if !utf8_prev
                        .chars()
                        .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
                        && utf8_prev.lines().eq(utf8_contents.lines())
                    {
                        bail!(
                            "{} differs only in line endings (CRLF vs. LF). If this is a text file, configure git to mark the file as `text eol=lf`.",
                            dst.display()
                        );
                    }
                }
                bail!("not up to date: {}", dst.display());
            }
            continue;
        }

        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {:?}", parent))?;
        }
        std::fs::write(&dst, contents).with_context(|| format!("failed to write {:?}", dst))?;
    }

    Ok(())
}

fn write_wit_files(opt: &Common, wit_inputs: &[PathBuf], world: &str) -> Result<()> {
    let Some(out_dir) = &opt.wit_out_dir else {
        return Ok(());
    };

    let safe_world = world
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();
    let dst_root = out_dir.join(safe_world);

    for wit in wit_inputs {
        if wit.is_dir() {
            copy_dir_all(wit, &dst_root)?;
        } else if wit.is_file() {
            std::fs::create_dir_all(&dst_root)
                .with_context(|| format!("failed to create {:?}", dst_root))?;
            let file_name = wit
                .file_name()
                .with_context(|| format!("WIT input has no file name: {:?}", wit))?;
            std::fs::copy(wit, dst_root.join(file_name))
                .with_context(|| format!("failed to copy {:?} to {:?}", wit, dst_root))?;
        }
    }

    std::fs::create_dir_all(&dst_root)
        .with_context(|| format!("failed to create {:?}", dst_root))?;
    std::fs::write(dst_root.join("world"), format!("{world}\n"))
        .with_context(|| format!("failed to write world file in {:?}", dst_root))?;

    Ok(())
}

fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(dst).with_context(|| format!("failed to create {:?}", dst))?;
    for entry in std::fs::read_dir(src).with_context(|| format!("failed to read {:?}", src))? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            std::fs::copy(entry.path(), &dst_path)
                .with_context(|| format!("failed to copy {:?} to {:?}", entry.path(), dst_path))?;
        }
    }
    Ok(())
}
