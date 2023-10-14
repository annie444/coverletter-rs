use crate::config;
use clap::{
    error::{ContextKind, ContextValue, Error as ClapError, ErrorKind},
    Command,
};

pub fn capitalize(s: &str) -> String {
    let mut f = String::new();
    for c in s.chars() {
        f.push(c.to_ascii_uppercase());
    }
    f
}

pub fn unable_to_parse_subcommand(cmd: Command, sub: Option<&str>) -> ClapError {
    let mut err = ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
    if sub.is_some() {
        err.insert(
            ContextKind::InvalidSubcommand,
            ContextValue::String(sub.unwrap().into()),
        );
    } else {
        err.insert(ContextKind::InvalidSubcommand, ContextValue::None);
    }
    err.insert(
        ContextKind::SuggestedSubcommand,
        ContextValue::Strings(vec!["cover".to_string(), "config".to_string()]),
    );
    err
}

#[derive(Clone)]
pub struct Name(String);

#[derive(Clone)]
pub struct NameValueParser;

impl clap::builder::TypedValueParser for NameValueParser {
    type Value = Name;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let matches = cmd.clone().get_matches();
        let subcommand = match matches.subcommand() {
            Some(("config", _)) => true,
            Some((_, _)) => false,
            None => false,
        };
        if value.len() > 1 {
            match value.to_str() {
                Some(val) => return Ok(Name(val.to_string())),
                None => match config::load_settings() {
                    Ok(n) => match n.name {
                        Some(name) => return Ok(Name(name)),
                        None => {
                            let mut err =
                                ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
                            err.insert(ContextKind::InvalidArg, ContextValue::None);
                            if !subcommand {
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
                            } else {
                                err.insert(
                                    ContextKind::Usage,
                                    ContextValue::StyledStr(crate::CONFIG_NAME.into()),
                                );
                            }
                            let e = err.render();
                            print!("{}", e.ansi());
                            return Err(err);
                        }
                    },
                    Err(_) => {
                        let mut err =
                            ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
                        err.insert(ContextKind::InvalidArg, ContextValue::None);
                        if !subcommand {
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
                        } else {
                            err.insert(
                                ContextKind::Usage,
                                ContextValue::StyledStr(crate::CONFIG_NAME.into()),
                            );
                        }
                        let e = err.render();
                        print!("{}", e.ansi());
                        return Err(err);
                    }
                },
            };
        } else {
            match config::load_settings() {
                Ok(n) => match n.name {
                    Some(name) => return Ok(Name(name.to_string())),
                    None => {
                        let mut err =
                            ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
                        err.insert(ContextKind::InvalidArg, ContextValue::None);
                        if !subcommand {
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
                        } else {
                            err.insert(
                                ContextKind::Usage,
                                ContextValue::StyledStr(crate::CONFIG_NAME.into()),
                            );
                        }
                        let e = err.render();
                        print!("{}", e.ansi());
                        return Err(err);
                    }
                },
                Err(_) => {
                    let mut err = ClapError::new(ErrorKind::Io).with_cmd(&cmd);
                    err.insert(ContextKind::InvalidValue, ContextValue::None);
                    if !subcommand {
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
                    } else {
                        err.insert(
                            ContextKind::Usage,
                            ContextValue::StyledStr(crate::CONFIG_NAME.into()),
                        );
                    }
                    let e = err.render();
                    print!("{}", e.ansi());
                    return Err(err);
                }
            };
        }
    }
}

#[derive(Clone)]
pub struct ApiKey(String);

#[derive(Clone)]
pub struct ApiKeyValueParser;

impl clap::builder::TypedValueParser for ApiKeyValueParser {
    type Value = ApiKey;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if value.len() > 1 {
            match value.to_str() {
                Some(val) => return Ok(ApiKey(val.to_string())),
                None => match config::load_settings() {
                    Ok(conf) => match conf.api_key {
                        Some(api) => return Ok(ApiKey(api)),
                        None => {
                            let mut err =
                                ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
                            err.insert(ContextKind::InvalidArg, ContextValue::None);
                            err.insert(
                                ContextKind::Usage,
                                ContextValue::StyledStr(crate::CONFIG_API.into()),
                            );
                            let e = err.render();
                            print!("{}", e.ansi());
                            return Err(err);
                        }
                    },
                    Err(_) => {
                        let mut err =
                            ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
                        err.insert(ContextKind::InvalidArg, ContextValue::None);
                        err.insert(
                            ContextKind::Usage,
                            ContextValue::StyledStr(crate::CONFIG_API.into()),
                        );
                        let e = err.render();
                        print!("{}", e.ansi());
                        return Err(err);
                    }
                },
            };
        } else {
            match config::load_settings() {
                Ok(conf) => match conf.api_key {
                    Some(api) => return Ok(ApiKey(api.to_string())),
                    None => {
                        let mut err =
                            ClapError::new(ErrorKind::MissingRequiredArgument).with_cmd(&cmd);
                        err.insert(ContextKind::InvalidArg, ContextValue::None);
                        err.insert(
                            ContextKind::Usage,
                            ContextValue::StyledStr(crate::CONFIG_API.into()),
                        );
                        let e = err.render();
                        print!("{}", e.ansi());
                        return Err(err);
                    }
                },
                Err(_) => {
                    let mut err = ClapError::new(ErrorKind::Io).with_cmd(&cmd);
                    err.insert(ContextKind::InvalidValue, ContextValue::None);
                    err.insert(
                        ContextKind::Usage,
                        ContextValue::StyledStr(crate::CONFIG_API.into()),
                    );
                    let e = err.render();
                    print!("{}", e.ansi());
                    return Err(err);
                }
            };
        }
    }
}
