use crate::store::Entry;
use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDate};
use std::fmt;
use yansi::Paint;

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = self.timestamp.time();
        write!(
            f,
            "{} {} {}",
            time.format("%H:%M").rgb(244, 9, 84),
            ":".dim(),
            self.message,
        )?;
        if let Some(ref long) = self.long {
            if f.alternate() {
                write!(f, " {}", format_long(long).italic())?;
            } else {
                write!(f, " {}", "[...]".dim().italic())?;
            }
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

pub fn parse_date(date: &str) -> Result<NaiveDate> {
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
