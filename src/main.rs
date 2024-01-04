use std::fs;
use clap::Parser;

mod args;
mod sd_error;
mod sd_file;

use sd_error::ShellDeckErrorKind;

use crate::args::Args;

fn main() -> Result<(), ShellDeckErrorKind> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("{args:?}");

    let file_path = args.file;
    let file_content = fs::read_to_string(file_path)?
        .split('\n')
        .skip(1)
        .collect::<Vec<&str>>()
        .join("\n");
    sd_file::ShellDeckFile::from_str(&file_content)?.execute(args.overrides)
}
