use anyhow::Result;
use clap::command;
use log::{debug, info};

mod config;

pub use config::*;

fn main() -> Result<()> {
    env_logger::init_from_env("KANTINNA_LOG");

    debug!("Initializing the configuration");
    let config = Config::from_file_or_default(CONFIG_TOML);

    cli(&config)
}

fn cli(config: &Config) -> Result<()>
{
    debug!("Starting the CLI with configuration:\n{}", &config);

    let cmd = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        // .add_builtins()
        .get_matches();
        // .exec(config)

    Ok(())
}

