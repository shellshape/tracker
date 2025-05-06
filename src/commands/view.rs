use super::Command;
use crate::config::Config;
use crate::store::Store;
use crate::util::{parse_date, select_date};
use anyhow::Result;
use chrono::{Duration, Local};
use clap::Args;
use fancy_duration::AsFancyDuration;
use std::io;
use yansi::Paint;

/// Display tracking list entries
#[derive(Args)]
#[command(visible_aliases = ["v"])]
pub struct View {
    /// Date of the list
    date: Option<String>,

    /// Select date from an interactive calender
    #[arg(short, long)]
    select: bool,

    /// Display additional description
    #[arg(short, long)]
    long: bool,

    /// Output entries as CSV
    #[arg(long)]
    csv: bool,
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

        if self.csv {
            for e in entries {
                e.to_csv(io::stdout())?;
            }
            return Ok(());
        }

        if entries.is_empty() {
            println!("{}", "There are no entries for this day.".italic().dim());
            return Ok(());
        }

        let mut sum = Duration::zero();
        let mut pause_time = Duration::zero();
        let mut last_timestamp = None;

        for (i, e) in entries.iter().enumerate() {
            if let Some(last_timestamp) = last_timestamp {
                match e.message_matches(&config.pause_regex)? {
                    true => pause_time += e.timestamp - last_timestamp,
                    false => sum += e.timestamp - last_timestamp,
                }
            }

            last_timestamp = Some(e.timestamp);

            print!(
                "{}{}{} ",
                "[".dim(),
                format!("{:>2}", i + 1).cyan().dim(),
                "]".dim(),
            );

            println!("{}", e.formatted(config, self.long)?);
        }

        println!(
            "\n     {} ({})",
            sum.fancy_duration().truncate(2).to_string().cyan().bold(),
            format!("{} pause", pause_time.fancy_duration().truncate(2)).green()
        );

        Ok(())
    }
}
