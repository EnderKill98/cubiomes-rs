use anyhow::{bail, Context, Result};
use std::{env, path::PathBuf, process::Command};

mod ignore_macros;

enum MakeResult {
    Success,
    MakeNotFound,
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build/");
    println!("cargo:rerun-if-changed=vendor/");

    generate_bindings().context("Generating bindings with bindgen")?;

    if let MakeResult::MakeNotFound = build_with_make().context("Building libcubiomes with make")? {
        println!("cargo:warning=Failed to build with system-provided make. Falling back to using cc crate for building instead...");
        build_with_cc().context("Building libcubiomes with cc crate")?;
    }
    Ok(())
}

fn generate_bindings() -> Result<()> {
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // Includes all other headers (except util.h)
        .header("vendor/quadbase.h")
        // Include has not implicitly included by quadbase.h
        .header("vendor/util.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Ignore some common macros that cause issues due to redefinition
        .parse_callbacks(Box::new(ignore_macros::IgnoreMacros::new()))
        // Finish the builder and generate the bindings.
        .generate()?;
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;
    Ok(())
}

fn build_with_make() -> Result<MakeResult> {
    // Copy to OUT_DIR as objects and lib are generated within the same directory
    let ref cubiomes_build_dir = PathBuf::from(env::var("OUT_DIR")?).join("cubiomes_build");
    if !cubiomes_build_dir.exists() {
        println!("Copying folder vendor to {cubiomes_build_dir:?} for building");
        let mut options = fs_extra::dir::CopyOptions::default();
        options.copy_inside = true;
        fs_extra::dir::copy("vendor", cubiomes_build_dir, &options)?;
        println!("Copied folder vendor to {cubiomes_build_dir:?} for building");
    }
    println!("Compiling libcubiomes in {cubiomes_build_dir:?}");
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let make_exit_code = match Command::new("make")
        .args(&["-C", cubiomes_build_dir.to_str().unwrap(), profile])
        .status()
    {
        Ok(exit_code) => exit_code,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                return Ok(MakeResult::MakeNotFound);
            }
            return Err(err.into());
        }
    };
    if !make_exit_code.success() {
        bail!("\"make\" returned a non-successful value!");
    }

    // Link library
    println!("cargo:rustc-link-lib=static=cubiomes");
    println!(
        "cargo:rustc-link-search=native={}",
        cubiomes_build_dir.to_str().unwrap()
    );
    Ok(MakeResult::Success)
}

fn build_with_cc() -> Result<(), cc::Error> {
    cc::Build::new()
        .file("vendor/biome_tree.c")
        .file("vendor/finders.c")
        .file("vendor/generator.c")
        .file("vendor/layers.c")
        .file("vendor/noise.c")
        .file("vendor/quadbase.c")
        .file("vendor/util.c")
        .try_compile("cubiomes")?;
    Ok(())
}
