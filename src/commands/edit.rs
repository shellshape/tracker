use super::Command;
use crate::config::Config;
use crate::store::{Entry, Store};
use crate::util::{FormatableEntry, parse_date, select_date};
use anyhow::Result;
use chrono::{Local, NaiveDateTime, NaiveTime};
use clap::Args;
use inquire::{CustomType, Editor, Select, Text};
use yansi::Paint;

/// Edit an entry from a tracking list
#[derive(Args)]
#[command(visible_aliases = ["e"])]
pub struct Edit {
    /// Date of the list
    date: Option<String>,

    /// Select date from an interactive calender
    #[arg(short, long)]
    select: bool,

    /// Edit the latest added entry
    #[arg(short, long)]
    last: bool,
}

impl Command for Edit {
    fn run(&self, store: &Store, config: &Config) -> Result<()> {
        let date = match self.date {
            Some(ref date_str) => parse_date(date_str)?,
            None if self.select => select_date()?,
            _ => Local::now().date_naive(),
        };

        let mut entries = store.list(date)?;

        if entries.is_empty() {
            println!("{}", "There are no entries for this day.".italic().dim());
            return Ok(());
        }

        if entries.is_empty() {
            return Err(anyhow::anyhow!("There are no entreis for the given date."));
        }

        let selected = match self.last {
            true => entries
                .last()
                .ok_or_else(|| anyhow::anyhow!("no entries found"))?,
            false => {
                entries.sort_by_key(|e| e.timestamp);
                let entries: Vec<_> = entries
                    .iter()
                    .map(|e| FormatableEntry::new(e, config, false))
                    .collect();
                Select::new("Select entry to edit", entries).prompt()?.entry
            }
        };

        let time: NaiveTime = CustomType::new("Time")
            .with_parser(&parse_time)
            .with_formatter(&format_time)
            .with_default_value_formatter(&format_time)
            .with_starting_input(&format_time(selected.timestamp.time()))
            .with_error_message("Invalid value. Must be time i nformat %H:%M")
            .prompt()?;

        let timestamp = NaiveDateTime::new(selected.timestamp.date(), time);

        let message = Text::new("Message")
            .with_initial_value(&selected.message)
            .prompt()?;

        let long = Editor::new("Long")
            .with_predefined_text(&selected.long.clone().unwrap_or_default())
            .prompt()?;

        let long = long.trim();
        let long = match long.is_empty() {
            true => None,
            false => Some(long.to_string()),
        };

        let new: Entry = Entry {
            timestamp,
            message,
            long,
        };

        let new = entries
            .iter()
            .map(|e| if e == selected { new.clone() } else { e.clone() })
            .collect();

        store.set(selected.timestamp.date(), new)
    }
}

fn parse_time(s: &str) -> std::result::Result<NaiveTime, ()> {
    NaiveTime::parse_from_str(s, "%H:%M").map_err(|_| ())
}

fn format_time(s: NaiveTime) -> String {
    s.format("%H:%M").to_string()
}
