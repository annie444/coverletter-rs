pub mod coverletter;
pub mod resume;
pub mod utils;

use include_dir::{include_dir, Dir};
pub static FONTS_DIR: Dir<'_> = include_dir!("./fonts");
pub static FILES_DIR: Dir<'_> = include_dir!("./static");
