use crate::config::{load_settings, write_settings};
use clap::{
    error::{ContextKind, ContextValue, Error as ClapError, ErrorKind, Result},
    ArgMatches, Command,
};

pub fn config(cmd: Command, matches: &ArgMatches) -> Result<(), ClapError> {
    match matches.subcommand() {
        Some(("name", sub_matches)) => crate::commands::name(cmd.clone(), sub_matches),
        Some(("api", sub_matches)) => crate::commands::api(cmd.clone(), sub_matches),
        Some((sub, _)) => return Err(crate::helpers::unable_to_parse_subcommand(cmd, Some(sub))),
        None => return Err(crate::helpers::unable_to_parse_subcommand(cmd, None)),
    }
}

pub fn name(cmd: Command, matches: &ArgMatches) -> Result<(), ClapError> {
    let name: String = match matches.get_one::<String>("name") {
        Some(n) => n.to_string(),
        None => {
            let mut err = ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
            err.insert(ContextKind::InvalidArg, ContextValue::None);
            err.insert(
                ContextKind::SuggestedValue,
                ContextValue::String("`Johnny Appleseed'".to_string()),
            );
            let e = err.render();
            print!("{}", e.ansi());
            return Err(err);
        }
    };
    let existing = load_settings().expect("Unable to load config");
    write_settings(&cmd, Some(existing), Some(name), None)
}

pub fn api(cmd: Command, matches: &ArgMatches) -> Result<(), ClapError> {
    let api: String = match matches.get_one::<String>("api") {
        Some(a) => a.to_string(),
        None => {
            let mut err = ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
            err.insert(ContextKind::InvalidArg, ContextValue::None);
            err.insert(
                ContextKind::SuggestedValue,
                ContextValue::String("`Ab1Dc32'".to_string()),
            );
            let e = err.render();
            print!("{}", e);
            return Err(err);
        }
    };
    let existing = load_settings().expect("Unable to load config");
    write_settings(&cmd, Some(existing), None, Some(api))
}
