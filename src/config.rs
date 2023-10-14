use crate::builders::utils;
use clap::{
    error::{Error as ClapError, ErrorKind},
    Command,
};
use config::{Config, ConfigError, Environment, File, FileFormat};
use dotenvy;
use home::home_dir;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{default::Default, fs::OpenOptions, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "CVConfig", rename_all = "snake_case")]
pub struct ConfigFile {
    pub name: Option<String>,
    pub api_key: Option<String>,
    pub resume: Option<Resume>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Resume {
    skills: Vec<utils::Skill>,
    employment: Vec<utils::WorkExperience>,
    education: Vec<utils::Degree>,
    projects: Vec<utils::Project>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            name: None,
            api_key: None,
            resume: None,
        }
    }
}

pub fn load_settings() -> Result<ConfigFile, ConfigError> {
    let mut envfile: PathBuf = match home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };
    envfile.push(".cvrc");
    dotenvy::from_path(envfile.clone()).ok();

    let mut mainfile: PathBuf = match home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };
    mainfile.push(".cv.config.yaml");

    let mut builder = Config::builder();
    builder = builder.add_source(File::from(envfile).format(FileFormat::Ini).required(false));
    builder = builder.add_source(
        Environment::with_prefix("CV_")
            .ignore_empty(true)
            .keep_prefix(false),
    );
    builder = builder.add_source(
        File::from(mainfile)
            .format(FileFormat::Yaml)
            .required(false),
    );

    let config = builder.build()?;
    let configfile = config.try_deserialize::<ConfigFile>()?;

    Ok(configfile)
}

pub fn write_settings(
    cmd: &Command,
    existing: Option<ConfigFile>,
    name: Option<String>,
    api_key: Option<String>,
) -> Result<(), ClapError> {
    let mut configfile = match existing {
        Some(existing) => existing,
        None => ConfigFile::default(),
    };
    if name.is_some() {
        configfile.name = name;
    }
    if api_key.is_some() {
        configfile.api_key = api_key;
    }

    let mut mainfile: PathBuf = match home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };
    mainfile.push(".cv.config.yaml");

    let writer = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(mainfile)
    {
        Ok(file) => file,
        Err(_) => {
            let err = ClapError::new(ErrorKind::Io).with_cmd(cmd);
            let e = err.render();
            print!("{}", e);
            return Err(err);
        }
    };

    match serde_yaml::to_writer(writer, &configfile) {
        Ok(_) => Ok(()),
        Err(_) => {
            let err = ClapError::new(ErrorKind::Io).with_cmd(cmd);
            let e = err.render();
            print!("{}", e);
            Err(err)
        }
    }
}
