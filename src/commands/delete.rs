use super::Command;
use crate::{store::Store, util::parse_date};
use anyhow::Result;
use chrono::Local;
use clap::Args;
use inquire::MultiSelect;

/// Remove entries from a tracking list
#[derive(Args)]
pub struct Delete {
    /// Date of the list
    date: Option<String>,
}

impl Command for Delete {
    fn run(&self, store: &Store) -> Result<()> {
        let date = match self.date {
            Some(ref date_str) => parse_date(date_str)?,
            None => Local::now().date_naive(),
        };

        let mut entries = store.list(date)?;
        entries.sort_by_key(|e| e.timestamp);

        let selected = MultiSelect::new("Select entries to delete", entries.clone()).prompt()?;

        let new = entries
            .iter()
            .filter(|&e| !selected.contains(e))
            .cloned()
            .collect();

        store.set(date, new)
    }
}
