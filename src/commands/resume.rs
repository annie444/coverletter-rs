use crate::config;
use clap::{
    error::{ContextKind, ContextValue, Error as ClapError, ErrorKind, Result},
    ArgMatches, Command,
};
use std::path::PathBuf;

pub fn resume(cmd: Command, matches: &ArgMatches) -> Result<(), ClapError> {
    let name: String = match matches.get_one::<String>("name") {
        Some(namearg) => namearg.to_string(),
        None => match config::load_settings() {
            Ok(conf) => match conf.name {
                Some(name) => name,
                None => {
                    let mut err = ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
                    err.insert(
                        ContextKind::Usage,
                        ContextValue::StyledStr(crate::CONFIG_NAME.into()),
                    );
                    let e = err.render();
                    print!("{}", e.ansi());
                    return Err(err);
                }
            },
            Err(_) => {
                let mut err = ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
                err.insert(ContextKind::InvalidArg, ContextValue::None);
                err.insert(
                    ContextKind::SuggestedArg,
                    ContextValue::String("--name".to_string()),
                );
                err.insert(
                    ContextKind::SuggestedValue,
                    ContextValue::String("`Johnny Appleseed'".to_string()),
                );
                err.insert(
                    ContextKind::Usage,
                    ContextValue::StyledStr(crate::CONFIG_NAME.into()),
                );
                let e = err.render();
                print!("{}", e.ansi());
                return Err(err);
            }
        },
    };
    let position: String = match matches.get_one::<String>("position") {
        Some(position) => position.to_owned(),
        None => {
            let mut err = ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
            err.insert(ContextKind::ExpectedNumValues, ContextValue::Number(2));
            err.insert(
                ContextKind::SuggestedArg,
                ContextValue::Strings(vec![
                    "--position".to_string(),
                    "-p".to_string(),
                    "--role".to_string(),
                    "-r".to_string(),
                ]),
            );
            err.insert(
                ContextKind::SuggestedValue,
                ContextValue::String("Professional Thinker".to_string()),
            );
            let e = err.render();
            print!("{}", e.ansi());
            return Err(err);
        }
    };

    let output: PathBuf = match matches.get_one::<PathBuf>("output") {
        Some(output) => output.to_owned(),
        None => {
            let mut err = ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
            err.insert(ContextKind::TrailingArg, ContextValue::None);
            err.insert(
                ContextKind::SuggestedArg,
                ContextValue::String("./coverletter.pdf".to_string()),
            );
            let e = err.render();
            print!("{}", e.ansi());
            return Err(err);
        }
    };

    match config::load_settings() {
        Ok(conf) => {
            if (conf.name.is_some() && conf.name.as_ref().unwrap() != &name)
                || conf.name.as_ref().is_none()
            {
                let _ = config::write_settings(&cmd, Some(conf), Some(name.clone()), None)?;
            }
        }
        Err(_) => (),
    };

    crate::builders::resume::build(name, position, output);
    Ok(())
}
