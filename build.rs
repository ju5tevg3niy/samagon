use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;

fn read_env<K>(key: K) -> anyhow::Result<String>
where
    K: AsRef<OsStr>,
{
    env::var(&key).with_context(|| format!("Failed to read {:?} env var", key.as_ref()))
}

fn build_meson_targets<P: AsRef<Path>>(out_dir: P) -> anyhow::Result<()> {
    println!("\n*** Building meson targets ***\n");

    let debug = read_env("DEBUG")?;

    let buildtype = match debug.as_str() {
        "true" => "debug",
        "false" => "release",
        other => anyhow::bail!("Environment boolean variable DEBUG contains {other}"),
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

    println!(
        "\nMeson targets have been built into: {}\n",
        meson_build_dir.display()
    );

    Ok(())
}

fn generate_bindings<P: AsRef<Path>>(out_dir: P) -> anyhow::Result<()> {
    println!("\n*** Generating bindings ***\n");

    let bindings_path = out_dir.as_ref().join("bindings.rs");

    bindgen::builder()
        .header("src/lib.h")
        .clang_arg("-std=c23")
        .allowlist_item("smgn_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .generate()
        .context("Failed to generate bindings")?
        .write_to_file(&bindings_path)
        .context("Failed to write bindings to file")?;

    println!(
        "\nBindings have been generated into: {}\n",
        bindings_path.display()
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let out_dir = read_env("OUT_DIR").context("Failed to get output directory")?;
    let out_dir = PathBuf::from(out_dir);

    build_meson_targets(&out_dir).context("Failed to setup FFI linking")?;

    generate_bindings(&out_dir).context("Failed to generate C bindings with bindgen")?;

    Ok(())
}
