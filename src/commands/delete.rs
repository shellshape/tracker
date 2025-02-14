use super::Command;
use crate::{
    config::Config,
    store::Store,
    util::{parse_date, select_date, FormatableEntry},
};
use anyhow::Result;
use chrono::Local;
use clap::Args;
use inquire::MultiSelect;

/// Remove entries from a tracking list
#[derive(Args)]
pub struct Delete {
    /// Date of the list
    date: Option<String>,

    /// Select date from an interactive calender
    #[arg(short, long)]
    select: bool,
}

impl Command for Delete {
    fn run(&self, store: &Store, config: &Config) -> Result<()> {
        let date = match self.date {
            Some(ref date_str) => parse_date(date_str)?,
            None if self.select => select_date()?,
            _ => Local::now().date_naive(),
        };

        let mut entries = store.list(date)?;
        entries.sort_by_key(|e| e.timestamp);

        let select_entries: Vec<_> = entries
            .clone()
            .into_iter()
            .map(|e| FormatableEntry::new(e, config, false))
            .collect();

        let selected: Vec<_> = MultiSelect::new("Select entries to delete", select_entries)
            .prompt()?
            .into_iter()
            .map(|e| e.entry)
            .collect();

        let new = entries
            .iter()
            .filter(|&e| !selected.contains(e))
            .cloned()
            .collect();

        store.set(date, new)
    }
}
