use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        NonEmptyStringValueParser, PathBufValueParser, Styles, ValueParser,
    },
    error::{Error as ClapError, Result},
    Arg, ColorChoice, Command, ValueHint,
};

#[tokio::main]
async fn main() -> Result<(), ClapError> {
    let mut cmd = Command::new("cv")
        .author("Annie Ehler <annie.ehler.4@gmail.com>")
        .version("0.1.4") 
        .about("A CLI program that builds application materials for any company or position") 
        .long_about(None)
        .display_name("CurriculumVitae")
        .arg_required_else_help(true)
        .allow_missing_positional(false)
        .bin_name("cv")
        .color(ColorChoice::Always)
        .styles(Styles::styled()
            .header(AnsiColor::Green.on_default() | Effects::BOLD)
            .usage(AnsiColor::Green.on_default() | Effects::BOLD)
            .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
            .placeholder(AnsiColor::Cyan.on_default()))
        .subcommand(Command::new("cover")
            .about("Build a cover letter for an application")
            .arg(Arg::new("company")
                .short('c')
                .long("company")
                .value_name("COMPANY")
                .value_parser(NonEmptyStringValueParser::new())
                .help("The company to write the cover letter for")
                .required(true))
            .arg(Arg::new("location")
                .short('l')
                .long("location")
                .value_name("LOCATION")
                .required(false)
                .help("The location of the office you're applying to"))
            .arg(Arg::new("position")
                .short('p')
                .long("position")
                .visible_short_alias('r')
                .visible_alias("role")
                .value_name("POSITION")
                .required(false)
                .help("The position you're applying to (leaving this blank will output a standard cover letter for any position)"))
            .arg(Arg::new("output")
                .value_name("OUTPUT")
                .help("The file to save the cover letter to.")
                .value_hint(ValueHint::FilePath)
                .value_parser(PathBufValueParser::new())
                .required(true))
            .arg(Arg::new("name")
                .value_name("NAME")
                .short('n')
                .value_parser(ValueParser::new(coverletter::helpers::NameValueParser))
                .long("name")
                .help("Your name for the cover letter. (This will be saved at ~/.coverletter so you only need to provide it once)")))
        .subcommand(Command::new("resume")
            .about("Build a resume for an application")
            .arg(Arg::new("position")
                .short('p')
                .long("position")
                .visible_short_alias('r')
                .visible_alias("role")
                .value_name("POSITION")
                .required(true)
                .help("The position you're applying to."))
            .arg(Arg::new("output")
                .value_name("OUTPUT")
                .help("The file to save the resume to.")
                .value_hint(ValueHint::FilePath)
                .value_parser(PathBufValueParser::new())
                .required(true))
            .arg(Arg::new("name")
                .value_name("NAME")
                .short('n')
                .value_parser(ValueParser::new(coverletter::helpers::NameValueParser))
                .long("name")
                .help("Your name for the resume.")))
        .subcommand(Command::new("config")
            .about("Configure the settings")
            .subcommand(Command::new("name")
                .about("Set your Name")
                .arg(Arg::new("name")
                     .value_name("NAME")
                     .help("Your full name as you want it to appear")
                     .value_parser(NonEmptyStringValueParser::new())
                     .required(true)))
            .subcommand(Command::new("api")
                .about("Set your OpenAI API key")
                .arg(Arg::new("api-key")
                     .value_name("API-KEY")
                     .value_parser(NonEmptyStringValueParser::new())
                     .help("Your OpenAI API key from https://platform.openai.com/account/api-keys")
                     .required(true))));

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("cover", sub_matches)) => {
            coverletter::commands::coverletter(cmd.clone(), sub_matches)?
        }
        Some(("config", sub_matches)) => coverletter::commands::config(cmd.clone(), sub_matches)?,
        Some(("resume", sub_matches)) => coverletter::commands::resume(cmd.clone(), sub_matches)?,
        Some((sub, _)) => {
            return Err(coverletter::helpers::unable_to_parse_subcommand(
                cmd,
                Some(sub),
            ))
        }
        None => return Err(coverletter::helpers::unable_to_parse_subcommand(cmd, None)),
    };

    Ok(cmd.build())
}
