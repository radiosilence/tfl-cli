//! Repo tasks. Run with `cargo xtask <task>`.

mod codegen;

use std::{fs, path::PathBuf, process::Command};

use anyhow::{Context, Result, bail};

const SPEC_URL: &str = "https://api.tfl.gov.uk/swagger/docs/v1";

fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("regen") => regen(),
        Some(other) => bail!("unknown task `{other}` (known tasks: regen)"),
        None => {
            eprintln!(
                "usage: cargo xtask <task>\n\ntasks:\n  regen    regenerate the REST client from TfL's Swagger spec"
            );
            Ok(())
        }
    }
}

/// Regenerates `crates/tfl-api-client/src/generated` from TfL's spec.
///
/// The spec and the generated sources are both committed, so a TfL change shows
/// up as a reviewable diff. Nothing under `generated/` should be hand-edited.
fn regen() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("xtask has no parent directory")?
        .to_path_buf();
    let spec_path = root.join("codegen/tfl-swagger.json");

    // Offline regeneration keeps `cargo xtask regen` runnable in CI, where it
    // guards against the committed output drifting from the committed spec.
    if std::env::var_os("TFL_SPEC_OFFLINE").is_none() {
        println!("==> fetching {SPEC_URL}");
        let status = Command::new("curl")
            .args(["-sSLf", "--retry", "3", "-o"])
            .arg(&spec_path)
            .arg(SPEC_URL)
            .status()
            .context("spawning curl")?;
        if !status.success() {
            bail!("curl exited with {status}");
        }
    } else {
        println!("==> using committed spec (TFL_SPEC_OFFLINE)");
    }

    println!("==> generating");
    let spec = serde_json::from_slice(&fs::read(&spec_path)?).context("parsing Swagger document")?;
    let out = codegen::generate(&spec)?;

    let dest = root.join("crates/tfl-api-client/src/generated");
    fs::create_dir_all(&dest)?;
    fs::write(dest.join("models.rs"), out.models)?;
    fs::write(dest.join("endpoints.rs"), out.endpoints)?;
    fs::write(
        dest.join("mod.rs"),
        "//! Generated from TfL's Swagger document by `cargo xtask regen`. Do not edit.\n\n\
         pub mod endpoints;\npub mod models;\n",
    )?;

    println!("==> formatting");
    let status = Command::new("cargo")
        .args(["fmt", "-p", "tfl-api-client"])
        .current_dir(&root)
        .status()
        .context("spawning cargo fmt")?;
    if !status.success() {
        bail!("cargo fmt exited with {status}");
    }

    println!(
        "==> done: {} models, {} endpoints",
        out.model_count, out.endpoint_count
    );
    Ok(())
}
