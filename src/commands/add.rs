use super::Command;
use crate::config::Config;
use crate::store::{Entry, Store};
use crate::util::{Parsable, select_date};
use anyhow::Result;
use chrono::{DurationRound, Local, NaiveDate, NaiveDateTime, NaiveTime};
use clap::Args;

/// Add a track entry
#[derive(Args)]
#[command(visible_aliases = ["a"])]
pub struct Add {
    /// A short message
    message: Vec<String>,

    /// Time to set the entry at
    #[arg(short, long)]
    time: Option<String>,

    /// Date to set the entry at
    #[arg(short, long)]
    date: Option<Parsable<NaiveDate>>,

    /// Select date from an interactive calender to set entry at
    #[arg(short, long)]
    select: bool,

    /// Add a long description by opening an editor
    #[arg(short, long)]
    long: bool,

    /// Add a long description as text content
    #[arg(long)]
    long_text: Option<String>,
}

impl Command for Add {
    fn run(&self, store: &Store, config: &Config) -> Result<()> {
        if self.message.is_empty() {
            return Err(anyhow::anyhow!("can not use empty message value"));
        }

        let date = match self.date {
            Some(Parsable(date_str)) => date_str,
            None if self.select => select_date()?,
            _ => Local::now().date_naive(),
        };

        let now = Local::now().naive_local();
        let timestamp = match self.time {
            Some(ref time) => NaiveDateTime::new(date, NaiveTime::parse_from_str(time, "%H:%M")?),
            None => now,
        };

        let long = match self.long_text {
            Some(ref txt) => Some(txt.clone()),
            None => match self.long {
                true => prompt_long()?,
                false => None,
            },
        };

        let timestamp = match config.round_steps {
            Some(ref round) => timestamp.duration_round(round.duration())?,
            None => timestamp,
        };

        store.push_entry(Entry {
            timestamp,
            message: self.message.join(" "),
            long,
        })
    }
}

fn prompt_long() -> Result<Option<String>> {
    let long = edit::edit("")?.trim().to_string();
    Ok(match long.is_empty() {
        true => None,
        false => Some(long),
    })
}
