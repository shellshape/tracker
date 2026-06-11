use super::Command;
use crate::config::Config;
use crate::db::Database;
use crate::util::Parsable;
use anyhow::Result;
use chrono::{Duration, NaiveDate, Utc};
use clap::{Args, ValueEnum};
use std::io::{self, Write};

#[derive(Clone, Default, ValueEnum)]
pub enum Format {
    #[default]
    Csv,
    Json,
}

/// Export tracking data in different formats to standard out
#[derive(Args)]
pub struct Export {
    /// Start of date range
    #[arg(long)]
    start_date: Option<Parsable<NaiveDate>>,

    /// End of date range
    #[arg(long, conflicts_with = "end_duration")]
    end_date: Option<Parsable<NaiveDate>>,

    /// End duration after start date
    #[arg(long, conflicts_with = "end_date")]
    end_duration: Option<Parsable<Duration>>,

    /// Duration before given end date or today's date if not given
    #[arg(long, conflicts_with_all = ["start_date", "end_date", "end_duration"])]
    previous: Option<Parsable<Duration>>,

    /// Output format
    #[arg(short, long, default_value = "csv")]
    format: Format,
}

impl Command for Export {
    fn run(&self, db: &Database, _: &Config) -> Result<()> {
        let start = match (&self.start_date, &self.end_date, &self.previous) {
            (Some(Parsable(start_date)), None, None) => *start_date,
            (None, Some(Parsable(end_date)), Some(Parsable(previous))) => *end_date - *previous,
            (None, None, Some(Parsable(previous))) => Utc::now().naive_local().date() - *previous,
            _ => {
                return Err(anyhow::anyhow!(
                    "either --start-date, --end-date and --previous or --previous must be given"
                ));
            }
        };

        let end = match self.end_date {
            Some(Parsable(end_date)) => end_date,
            None => Utc::now().naive_local().date(),
        };

        let w = io::stdout();

        db.list_range(start, end, |entry| match self.format {
            Format::Csv => entry.to_csv(&w),
            Format::Json => {
                serde_json::to_writer(&w, &entry)?;
                (&w).write_all(b"\n")?;
                Ok(())
            }
        })
    }
}
