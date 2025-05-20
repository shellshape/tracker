use super::Command;
use crate::config::Config;
use crate::store::{Entry, Store};
use anyhow::Result;
use chrono::{DurationRound, Local, NaiveDateTime, NaiveTime};
use clap::Args;

/// Swaps the next entry with the given timestamp and sets the next
/// entries info to the given info
#[derive(Args)]
#[command(visible_aliases = ["i"])]
pub struct Insert {
    /// A short message
    message: Vec<String>,

    /// Time to set the entry at
    #[arg(short, long)]
    time: Option<String>,

    /// Add a long description by opening an editor
    #[arg(short, long)]
    long: bool,

    /// Add a long description as text content
    #[arg(long)]
    long_text: Option<String>,
}

impl Command for Insert {
    fn run(&self, store: &Store, config: &Config) -> Result<()> {
        if self.message.is_empty() {
            return Err(anyhow::anyhow!("can not use empty message value"));
        }

        let now = Local::now().naive_local();
        let timestamp = match self.time {
            Some(ref time) => {
                NaiveDateTime::new(now.date(), NaiveTime::parse_from_str(time, "%H:%M")?)
            }
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

        let mut entries = store.list(timestamp.date())?;
        entries.sort_by_key(|e| e.timestamp);

        let after_entry = entries
            .iter_mut()
            .find(|e| e.timestamp > timestamp)
            .ok_or_else(|| anyhow::anyhow!("no entries after the given timestamp"))?;

        let prev_timestamp = after_entry.timestamp;
        after_entry.timestamp = timestamp;

        entries.push(Entry {
            timestamp: prev_timestamp,
            message: self.message.join(" "),
            long,
        });

        store.set(timestamp.date(), entries)
    }
}

fn prompt_long() -> Result<Option<String>> {
    let long = edit::edit("")?.trim().to_string();
    Ok(match long.is_empty() {
        true => None,
        false => Some(long),
    })
}
