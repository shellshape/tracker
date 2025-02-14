use std::fmt;

use super::Command;
use crate::{
    config::Config,
    store::Store,
    util::{parse_date, select_date},
};
use anyhow::Result;
use chrono::Local;
use clap::Args;
use regex::Regex;
use yansi::{Paint, Style};

/// Display tracking list entries
#[derive(Args)]
pub struct View {
    /// Date of the list
    date: Option<String>,

    /// Select date from an interactive calender
    #[arg(short, long)]
    select: bool,

    /// Display additional description
    #[arg(short, long)]
    long: bool,
}

impl Command for View {
    fn run(&self, store: &Store, config: &Config) -> Result<()> {
        let date = match self.date {
            Some(ref date_str) => parse_date(date_str)?,
            None if self.select => select_date()?,
            _ => Local::now().date_naive(),
        };

        let mut entries = store.list(date)?;
        entries.sort_by_key(|e| e.timestamp);

        for (i, e) in entries.iter().enumerate() {
            print!(
                "{}{}{} ",
                "[".dim(),
                format!("{:>2}", i + 1).cyan().dim(),
                "]".dim(),
            );

            println!("{}", e.formatted(config, self.long)?);
        }

        Ok(())
    }
}
