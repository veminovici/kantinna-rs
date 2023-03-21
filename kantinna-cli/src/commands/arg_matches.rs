use anyhow::Result;
use clap::ArgMatches;
use log::debug;

use crate::{commands::config, config::Config};

pub trait ArgMatchesExt {
    fn exec(self, config: &Config) -> Result<()>;
}

impl ArgMatchesExt for ArgMatches {
    fn exec(self, config: &Config) -> Result<()> {
        debug!("Matching the command");

        match self.subcommand() {
            Some((config::NAME, args)) => config::exec(config, args),
            // Some((foam_meili::NAME, args)) => foam_meili::exec(&config.meili, args),
            // Some((foam_search::NAME, args)) => foam_search::exec(&config.search, args),
            _ => unreachable!(
                "Exhausted list of subcommands and subcommand_required prevents `None`"
            ),
        }
    }
}
