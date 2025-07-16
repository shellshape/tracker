use crate::config;
use anyhow::Result;
use inquire::Select;
use std::fs::File;
use std::{fmt, fs};
use yansi::Paint;

enum PromtYesNoRemember {
    Yes,
    No,
    NoRemember,
}

impl fmt::Display for PromtYesNoRemember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yes => f.write_str("Yes"),
            Self::No => f.write_str("No"),
            Self::NoRemember => f.write_str("No (don't ask again)"),
        }
    }
}

impl PromtYesNoRemember {
    pub fn promt(message: &str) -> inquire::Select<Self> {
        let options = vec![Self::Yes, Self::No, Self::NoRemember];
        Select::new(message, options)
    }
}

pub fn migrate() -> Result<()> {
    default_config_dir()
}

// Default storage location in config changed
// from dirs::home_dir() / "time_trackings"
// to dirs::data_dir() / "time_trackings".
//
// Added:      v0.8.0
// Removal: >= v1.x
fn default_config_dir() -> Result<()> {
    let Some(home_dir) = dirs::home_dir() else {
        return Ok(());
    };

    let old_dir = home_dir.join("time_trackings");
    let ignored_old_dir = old_dir.join(".migration_ignored");

    if !old_dir.exists() || ignored_old_dir.exists() {
        return Ok(());
    }

    let new_dir = config::default_storage_dir();

    println!(
        "{}",
        &format!("In version 0.8.0, the default trackings location changed form {old_dir:?} to {new_dir:?}.\n\
        If you proceed, the time trackigns from the old directory are moved to the new directory.").dim());
    let res =
        PromtYesNoRemember::promt("Do you want to migrate your storage location?").prompt()?;

    match res {
        PromtYesNoRemember::No => {
            println!("{}", "No files have been moved.\n\
                You will not see your previous entries, if you are using the default storage location!\n".yellow())
        }
        PromtYesNoRemember::NoRemember => {
            File::create(&ignored_old_dir)?;
            println!("{}", format_args!("No files have been moved and you will no more be prompted anymore.\n\
                You can reverse this action by removing the file {ignored_old_dir:?}.\n\
                You will not see your previous entries, if you are using the default storage location!\n").yellow())
        }
        PromtYesNoRemember::Yes => {
            fs::rename(old_dir, new_dir)?;
            println!(
                "{}",
                "Successfully moved time tracking data to new location.\n".green()
            );
        }
    }

    Ok(())
}
