mod opts;
mod process;

pub use opts::{GenPassOpts, Opts, OutputFormat, SubCommand};
pub use process::process_csv;
pub use process::process_genpass;
