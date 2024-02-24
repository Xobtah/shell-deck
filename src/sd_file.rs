use std::{collections::HashMap, process};

use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};

use crate::sd_error::ShellDeckErrorKind;

type VarDict = HashMap<String, String>;

const VARIABLES_REGEX: &str = r"%%([A-Za-z]+)%%";

fn parse_overrides(overrides: Vec<String>) -> VarDict {
    let mut map = HashMap::new();
    for or in overrides {
        let mut split = or.split('=');
        let key = split.next().unwrap();
        let value = split.next().unwrap();
        map.insert(key.to_string(), value.to_string());
    }
    map
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Executable {
    pub command: String,
    #[serde(default)]
    pub defaults: VarDict,
    #[serde(default)]
    pub overrides: VarDict,
    #[serde(default)]
    pub fatal: bool,
}

impl Executable {
    pub fn run(&self, overrides: Option<VarDict>) -> Result<(), ShellDeckErrorKind> {
        // env::var("SHELLDECK_VARIABLES").ok().map(parse_variables);
        // Merging variables
        let variables = if let Some(overrides) = overrides {
            Self::override_defaults(self.defaults.clone(), overrides)
        } else {
            self.defaults.clone()
        };

        // Replace variables in command
        let empty_string = String::new();
        let cmd = regex::Regex::new(VARIABLES_REGEX)?
            .captures_iter(&self.command)
            .fold(self.command.clone(), |acc, cap| {
                let variable = cap.get(1).unwrap().as_str();
                let value = variables.get(variable).unwrap_or(&empty_string);
                acc.replace(&format!("%%{}%%", variable), value)
            });

        // Override variables
        let cmd = if !self.overrides.is_empty() {
            self.overrides
                .iter()
                .fold(cmd, |acc, (key, value)| format!("{acc} -o {key}={value}"))
        } else {
            cmd
        };

        // Execute command
        log::debug!("Variables: {variables:?}");
        log::debug!("Running: {cmd}");
        let status = process::Command::new("sh").arg("-c").arg(cmd).status()?;
        if !status.success() && self.fatal {
            return Err(ShellDeckErrorKind::FailedToExecute);
        }
        Ok(())
    }

    fn override_defaults(old_vars: VarDict, new_vars: VarDict) -> VarDict {
        new_vars
            .into_iter()
            .fold(old_vars, |mut acc, (key, value)| {
                acc.insert(key, value);
                acc
            })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellDeckFile {
    #[serde(default)]
    pub before: Option<Executable>,
    #[serde(default)]
    pub description: String,
    pub executable: Executable,
}

impl ShellDeckFile {
    pub fn new_from_command_interactive(cmd: &str) -> Result<Self, ShellDeckErrorKind> {
        Ok(Self {
            before: None,
            description: String::new(),
            executable: Executable {
                command: cmd.to_string(),
                defaults: regex::Regex::new(VARIABLES_REGEX)?.captures_iter(cmd).fold(
                    HashMap::new(),
                    |mut acc, cap| {
                        let variable = cap.get(1).unwrap().as_str().to_uppercase();
                        acc.entry(variable.clone()).or_insert(
                            Input::<String>::with_theme(&ColorfulTheme::default())
                                .with_prompt(format!("Default value for variable `{variable}`:"))
                                .interact()
                                .unwrap(),
                        );
                        acc
                    },
                ),
                overrides: HashMap::new(),
                fatal: true,
            },
        })
    }

    pub fn from_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn execute(&self, overrides: Option<Vec<String>>) -> Result<(), ShellDeckErrorKind> {
        if let Some(before) = &self.before {
            before.run(None)?;
        }
        self.executable
            .run(overrides.and_then(|overrides| Some(parse_overrides(overrides))))?;
        Ok(())
    }
}
