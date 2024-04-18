use super::Command;
use crate::store::{Entry, Store};
use anyhow::Result;
use chrono::{Local, NaiveDateTime, NaiveTime};
use clap::Args;
use inquire::Confirm;

#[derive(Args)]
pub struct Add {
    message: Vec<String>,

    #[arg(short, long)]
    time: Option<String>,

    #[arg(short, long)]
    long: Option<String>,
}

impl Command for Add {
    fn run(&self, store: &Store) -> Result<()> {
        if self.message.is_empty() {
            anyhow::bail!("can not use empty message value")
        }

        let now = Local::now().naive_local();
        let timestamp = match self.time {
            Some(ref time) => {
                NaiveDateTime::new(now.date(), NaiveTime::parse_from_str(time, "%H:%M")?)
            }
            None => now,
        };

        let long = match self.long {
            Some(ref v) => Some(v.clone()),
            None => prompt_long()?,
        };

        store.push_entry(Entry {
            timestamp,
            message: self.message.join(" "),
            long,
        })
    }
}

fn prompt_long() -> Result<Option<String>> {
    let ans = Confirm::new("Do you want to add a long description?")
        .with_default(false)
        .prompt()?;

    if !ans {
        return Ok(None);
    }

    let long = edit::edit("")?.trim().to_string();
    Ok(match long.is_empty() {
        true => None,
        false => Some(long),
    })
}
