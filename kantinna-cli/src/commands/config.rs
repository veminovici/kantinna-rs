use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use log::debug;

use crate::config::Config;

pub const NAME: &str = "config";

const ABOUT: &str = "Configuration information";
const FLAG: char = 'C';
const HELP: &str = "Run `foam help cfg` for more detailed information.\n";

const LIST_NAME: &str = "list";
const LIST_SHORT: char = 'L';
const LIST_ABOUT: &str = "Print the entire configuration";

fn list_command() -> Command {
    Command::new(LIST_NAME)
        .short_flag(LIST_SHORT)
        .about(LIST_ABOUT)
}

const GET_NAME: &str = "get";
const GET_SHORT: char = 'G';
const GET_ABOUT: &str = "Retrieve the value of a configuration key";

const ARG_KEY: &str = "key";
const ARG_KEY_HELP: &str = "The configuration key to be displayed";

fn get_command() -> Command {
    Command::new(GET_NAME)
        .short_flag(GET_SHORT)
        .about(GET_ABOUT)
        .arg(
            Arg::new(ARG_KEY)
                .help(ARG_KEY_HELP)
                .required(true)
                .num_args(1..),
        )
}

pub fn command() -> Command {
    Command::new(NAME)
        .short_flag(FLAG)
        .about(ABOUT)
        .after_help(HELP)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(list_command())
        .subcommand(get_command())
}

pub fn exec(config: &Config, args: &ArgMatches) -> Result<()> {
    debug!("Exec'ing <{NAME}> subcommand");

    match args.subcommand() {
        Some((LIST_NAME, _)) => print_config(config),
        Some((GET_NAME, args)) => print_keys(config, args),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}

fn print_keys(config: &Config, args: &ArgMatches) {
    let _: Vec<_> = args
        .get_many::<String>(ARG_KEY)
        .unwrap()
        .map(|s| s.as_str())
        .flat_map(|key| print_key(config, key))
        .inspect(|(k, v)| println!("{k}: {v}"))
        .collect();
}

fn print_key<K>(config: &Config, key: K) -> Vec<(String, String)>
where
    K: AsRef<str>,
{
    match key.as_ref() {
        // foam_meili::NAME => println!("{}", config.meili),
        // foam_search::NAME => println!("{}", config.search),
        //APP => println!("{}", AppInfo::default()),
        _ => println!("Get_key_values: {}", key.as_ref()),
    }

    vec![]
}

fn print_config(config: &Config) {
    // println!("{}", AppInfo::default());
    println!("{config}")
}
