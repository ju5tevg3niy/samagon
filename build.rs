use anyhow::Context;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn read_env<K>(key: K) -> anyhow::Result<String>
where
    K: AsRef<OsStr>,
{
    env::var(&key).with_context(|| format!("Failed to read {:?} env var", key.as_ref()))
}

fn setup_ffi_linking<P: AsRef<Path>>(out_dir: P) -> anyhow::Result<()> {
    let debug = read_env("DEBUG")?;

    let buildtype = match debug.as_str() {
        "true" => "debug",
        "false" => "release",
        _ => anyhow::bail!("DEBUG env var contains {debug}"),
    };

    let meson_build_dir = out_dir.as_ref().join("meson-build");

    let setup_status = Command::new("meson")
        .arg("setup")
        .arg("--reconfigure")
        .arg(meson_build_dir.as_os_str())
        .arg("--buildtype")
        .arg(buildtype)
        .status()
        .context("Failed to setup meson build directory")?;

    assert!(setup_status.success());

    let compile_status = Command::new("ninja")
        .arg("--verbose")
        .arg("-C")
        .arg(meson_build_dir.as_os_str())
        .status()
        .context("Failed to compile meson build directory")?;

    assert!(compile_status.success());

    // Add meson output dir to rustc's library search path
    println!("cargo::rustc-link-search={}", meson_build_dir.display());

    // Ask rustc to link rust binary to library produced by meson
    println!("cargo::rustc-link-lib=samagon");

    // Rerun this build script if meson build definition changes
    println!("cargo::rerun-if-changed=meson.build");

    // Rerun this if anything inside src/ changes
    println!("cargo::rerun-if-changed=src");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let out_dir = read_env("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);

    setup_ffi_linking(out_dir).context("Failed to setup FFI linking")?;

    Ok(())
}
