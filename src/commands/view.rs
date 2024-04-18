use super::Command;
use crate::store::Store;
use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDate};
use clap::Args;
use yansi::Paint;

#[derive(Args)]
pub struct View {
    date: Option<String>,

    #[arg(short, long)]
    long: bool,
}

impl Command for View {
    fn run(&self, store: &Store) -> Result<()> {
        let date = match self.date {
            Some(ref date_str) => parse_date(date_str)?,
            None => Local::now().date_naive(),
        };

        let entries = store.list(date)?;

        for (i, e) in entries.iter().enumerate() {
            let time = e.timestamp.time();
            print!(
                "{}{}{} {} {} {}",
                "[".dim(),
                format!("{i:>2}").cyan().dim(),
                "]".dim(),
                time.format("%H:%M").rgb(244, 9, 84),
                ":".dim(),
                e.message,
            );
            if let Some(ref long) = e.long {
                if self.long {
                    print!(" {}", format_long(long).italic());
                } else {
                    print!(" {}", "[...]".dim().italic())
                }
            }
            println!();
        }

        Ok(())
    }
}

fn format_long(long: &str) -> String {
    let mut str = String::new();
    for line in long.split('\n') {
        str.push_str("\n\t");
        str.push_str(line);
    }
    str
}

fn parse_date(date: &str) -> Result<NaiveDate> {
    let today = Local::now().date_naive();

    if let Some(days_str) = date.strip_prefix('-') {
        let days = days_str.parse()?;
        return Ok(today - Duration::days(days));
    }

    let delims = date.chars().filter(|&c| c == '-').count();

    let year = today.year();
    let month = today.month0() + 1;

    let date = match delims {
        0 => format!("{year}-{month}-{date}"),
        1 => format!("{year}-{date}"),
        2 => date.to_string(),
        _ => anyhow::bail!("invalid date format"),
    };

    Ok(NaiveDate::parse_from_str(&date, "%Y-%m-%d")?)
}
