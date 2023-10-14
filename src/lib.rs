pub mod builders;
pub mod commands;
pub mod config;
pub mod helpers;

use color_print::cstr;
pub static CONFIG_NAME: &'static str = cstr!(
    r#"<bold><bright-white>Your name hasn't been set, try running the config:</></>

  <white><dim>$</></> <bold><cyan>cv config</> <green>name</> <yellow>'<<YOUR NAME>>'</></>
"#
);

pub static CONFIG_API: &'static str = cstr!(
    r#"<bold><bright-white>Your api key hasn't been set, try running the config:</></>

  <white><dim>$</></> <bold><cyan>cv config</> <green>api</> <yellow>'<<YOUR OPENAI API KEY>>'</></>
"#
);
