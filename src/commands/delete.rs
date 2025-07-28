use super::Command;
use crate::config::Config;
use crate::store::Store;
use crate::util::{FormatableEntry, Parsable, select_date};
use anyhow::Result;
use chrono::{Local, NaiveDate};
use clap::Args;
use inquire::MultiSelect;
use yansi::Paint;

/// Remove entries from a tracking list
#[derive(Args)]
#[command(visible_aliases = ["d"])]
pub struct Delete {
    /// Date of the list
    date: Option<Parsable<NaiveDate>>,

    /// Select date from an interactive calender
    #[arg(short, long)]
    select: bool,
}

impl Command for Delete {
    fn run(&self, store: &Store, config: &Config) -> Result<()> {
        let date = match self.date {
            Some(Parsable(date_str)) => date_str,
            None if self.select => select_date()?,
            _ => Local::now().date_naive(),
        };

        let mut entries = store.list(date)?;

        if entries.is_empty() {
            println!("{}", "There are no entries for this day.".italic().dim());
            return Ok(());
        }

        entries.sort_by_key(|e| e.timestamp);

        let select_entries: Vec<_> = entries
            .iter()
            .map(|e| FormatableEntry::new(e, config, false))
            .collect();

        let selected: Vec<_> = MultiSelect::new("Select entries to delete", select_entries)
            .prompt()?
            .into_iter()
            .map(|e| e.entry)
            .collect();

        let new = entries
            .iter()
            .filter(|&e| !selected.contains(&e))
            .cloned()
            .collect();

        store.set(date, new)
    }
}
