use super::util::PromtYesNoRemember;
use crate::config::{self, Config};
use crate::db::Database;
use crate::migration::store::Store;
use crate::model::NewEntry;
use anyhow::Result;
use std::fs::{self, File};
use yansi::Paint;

// Default storage location in config changed
// from dirs::home_dir() / "time_trackings"
// to dirs::data_dir() / "time_trackings".
//
// Added:      v0.8.0
// Removal: >= v1.x
pub(crate) fn default_config_dir() -> Result<()> {
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

// Storage format changed from CSV files to an
// SQLite3 database.
//
// Added:      v0.11.0
// Removal: >= v1.x
pub(crate) fn sqlite_database(config: &Config) -> Result<()> {
    if !config.storage_dir.exists() {
        return Ok(());
    }

    let ignore_file = config.storage_dir.join(".migration_sqlite_ignored");
    if ignore_file.exists() {
        return Ok(());
    }

    let store = Store::new(&config.storage_dir)?;
    let count = store.file_count()?;
    if count == 0 {
        return Ok(());
    }

    println!(
        "{}",
        "In version 0.11.0, the storage model for entries changed from CSV files to a SQLite3 database.\n\
        If you proceed, your old entries will automatically be migrated to the new storage model.".dim());
    let res = PromtYesNoRemember::promt("Do you want to migrate your storage?").prompt()?;

    match res {
        PromtYesNoRemember::No => {
            println!("{}", "Migration has been skipped.\n\
                A new database empty has been created, which will be used. Your old entries are still available on disk.\n".yellow());
            return Ok(());
        }
        PromtYesNoRemember::NoRemember => {
            File::create(&ignore_file)?;
            println!("{}", format_args!("Migration has been skipped and you will no more be prompted anymore.\n\
                You can reverse this action by removing the file {ignore_file:?}.\n\
                A new database empty has been created, which will be used. Your old entries are still available on disk.\n").yellow());
            return Ok(());
        }
        _ => {}
    }

    let db = Database::new(&config.storage_dir)?;

    println!();

    for (i, v) in store.enumerate() {
        let (path, entries) = v?;
        for entry in entries {
            db.add(NewEntry {
                timestamp: entry.timestamp,
                message: entry.message,
                long: entry.long,
            })?;
        }
        fs::remove_file(path)?;
        print!(
            "{}",
            format!("\rMigrated {i} of {count} entry files ...")
                .dim()
                .italic()
        );
    }

    print!(
        "{}",
        "\rSuccessfully migrated tracking data to the SQLite3 database.\n".green()
    );

    Ok(())
}
