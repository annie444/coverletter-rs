use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Arg, ColorChoice, Command, ValueHint,
};
use dotenvy;
use home::home_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod builder;
pub mod helpers;

fn main() {
    let mut home: PathBuf = match home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!"),
    };

    home.push(".coverletter");
    dotenvy::from_path(home.clone().as_path()).ok();

    let mut cmd = Command::new("coverletter")
        .author("Annie Ehler <annie.ehler.4@gmail.com>")
        .version("0.1") 
        .about("A CLI program that builds a coverletter for any company or position") 
        .long_about(None)
        .display_name("coverletter")
        .arg_required_else_help(true)
        .allow_missing_positional(false)
        .bin_name("coverletter")
        .name("coverletter")
        .color(ColorChoice::Always)
        .styles(Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default()))
        .arg(Arg::new("company")
            .short('c')
            .long("company")
            .value_name("COMPANY")
            .help("The company to write the cover letter for")
            .required(true))
        .arg(Arg::new("location")
            .short('l')
            .long("location")
            .value_name("LOCATION")
            .help("The location of the office you're applying to"))
        .arg(Arg::new("position")
            .short('p')
            .long("position")
            .visible_short_alias('r')
            .visible_alias("role")
            .value_name("POSITION")
            .help("The position you're applying to (leaving this blank will output a standard cover letter for any position)"))
        .arg(Arg::new("output")
            .value_name("OUTPUT")
            .help("The file to save the cover letter to.")
            .value_hint(ValueHint::FilePath)
            .required(true))
        .arg(Arg::new("name")
            .value_name("NAME")
            .short('n')
            .long("name")
            .help("Your name for the cover letter. (This will be saved at ~/.coverletter so you only need to provide it once)")
            .env("MY_NAME"));

    let matches = cmd.clone().try_get_matches().unwrap_or_else(|e| e.exit());

    println!("{:?}", dotenvy::vars().collect::<Vec<(String, String)>>());

    let name: String = match matches.get_one::<String>("name") {
        Some(n) => n.to_string(),
        None => match dotenvy::var("MY_NAME") {
            Ok(n) => n.to_string(),
            Err(e) => panic!("Nothing is working! Who even are you!? : {}", e),
        },
    };
    let company: String = matches.get_one::<String>("company").unwrap().to_string();
    let location: String = matches.get_one::<String>("location").unwrap().to_string();
    let position: String = matches.get_one::<String>("position").unwrap().to_string();
    let output: String = matches.get_one::<String>("output").unwrap().to_owned();

    if dotenvy::var("MY_NAME").is_err() {
        let mut file = File::options()
            .append(true)
            .create(true)
            .open(home.as_path())
            .expect("Unable to open ~/.coverletter file");
        writeln!(&mut file, "MY_NAME='{}'", name).expect("Unable to write to ~/.coverletter file");
    }

    builder::build(name, company, location, position, output);

    cmd.build()
}
