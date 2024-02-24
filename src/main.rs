use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};
use std::{
    fs,
    io::{self, BufRead},
    os::unix::fs::PermissionsExt,
};

mod args;
mod sd_error;
mod sd_file;

use sd_error::ShellDeckErrorKind;

use crate::{args::Args, sd_file::ShellDeckFile};

fn main() -> Result<(), ShellDeckErrorKind> {
    env_logger::init();
    if let Ok(args) = Args::try_parse() {
        log::debug!("{args:?}");

        let file_path = args.file;
        let file_content = fs::read_to_string(file_path)?
            .split('\n')
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");
        sd_file::ShellDeckFile::from_str(&file_content)?.execute(args.overrides)
    } else {
        if let Some(stdin) = io::stdin().lock().lines().next() {
            let script = stdin?;
            println!("Creating new script: `{script}`");
            let file_path = format!(
                ".sd/files/{}.sdf",
                Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Script name")
                    .interact()?
            );
            fs::write(
                &file_path,
                format!(
                    "#!//Users/sylvain/Work/shell-deck/target/release/shell-deck\n{}",
                    ShellDeckFile::new_from_command_interactive(&script)?.to_string()?
                ),
            )?;
            let mut perms = fs::metadata(&file_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&file_path, perms)?;
        }
        Ok(())
    }
}
