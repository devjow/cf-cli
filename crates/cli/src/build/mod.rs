use crate::common::{self, CommonArgs};
use anyhow::{Context, bail};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct BuildArgs {
    /// Path to the module to build
    #[arg(short = 'p', long, default_value = ".")]
    path: PathBuf,
    /// Build in release mode
    #[arg(short = 'r', long)]
    release: bool,
    /// Build with OpenTelemetry support
    #[arg(long)]
    otel: bool,
    #[command(flatten)]
    common_args: CommonArgs,
}

impl BuildArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let path = self
            .path
            .canonicalize()
            .context("can't canonicalize workspace")?;

        let config_path = self
            .common_args
            .config
            .canonicalize()
            .context("can't canonicalize config")?;

        let dependencies = common::get_config(&path, &config_path)?.create_dependencies()?;
        common::generate_server_structure(&path, &config_path, &dependencies)?;

        let cargo_dir = path.join(common::BASE_PATH);
        let status = common::cargo_command("build", &cargo_dir, self.otel, self.release)
            .status()
            .context("failed to run cargo build")?;

        if !status.success() {
            bail!("cargo build exited with {status}");
        }

        Ok(())
    }
}
