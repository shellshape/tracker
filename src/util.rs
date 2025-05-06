use crate::config::Config;
use crate::store::Entry;
use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDate};
use inquire::DateSelect;
use regex::Regex;
use std::fmt;
use yansi::{Paint, Style};

static STYLE_START: Style = Style::new().cyan();
static STYLE_PAUSE: Style = Style::new().green();
static STYLE_END: Style = Style::new().cyan();

impl Entry {
    pub fn timestamp_formatted(&self) -> String {
        let time = self.timestamp.time();
        time.format("%H:%M").to_string()
    }

    pub fn formatted(&self, config: &Config, long: bool) -> Result<String> {
        let mut res = String::new();
        self.format(&mut res, config, long)?;
        Ok(res)
    }

    pub fn format(&self, mut f: impl fmt::Write, config: &Config, long: bool) -> Result<()> {
        write!(
            f,
            "{} {} {}",
            self.timestamp_formatted().rgb(244, 9, 84),
            ":".dim(),
            self.style_message(config)?
        )?;
        if let Some(ref long_message) = self.long {
            if long {
                write!(f, " {}", format_long(long_message).italic())?;
            } else {
                write!(f, " {}", "[...]".dim().italic())?;
            }
        }
        Ok(())
    }

    pub fn message_matches(&self, rx: &str) -> Result<bool> {
        Ok(Regex::new(rx)?.is_match(&self.message))
    }

    fn style_message<'a>(&'a self, config: &Config) -> Result<Box<dyn fmt::Display + 'a>> {
        Ok(if self.message_matches(&config.start_regex)? {
            Box::new(self.message.paint(STYLE_START))
        } else if self.message_matches(&config.pause_regex)? {
            Box::new(self.message.paint(STYLE_PAUSE))
        } else if self.message_matches(&config.end_regex)? {
            Box::new(self.message.paint(STYLE_END))
        } else {
            Box::new(&self.message)
        })
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

pub fn select_date() -> Result<NaiveDate> {
    let date = DateSelect::new("Select Date").prompt()?;
    Ok(date)
}

pub struct FormatableEntry<'a> {
    pub config: &'a Config,
    pub long: bool,
    pub entry: Entry,
}

impl<'a> FormatableEntry<'a> {
    pub fn new(entry: Entry, config: &'a Config, long: bool) -> Self {
        Self {
            entry,
            config,
            long,
        }
    }
}

impl fmt::Display for FormatableEntry<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.entry
            .format(f, self.config, self.long)
            .map_err(|_| fmt::Error)
    }
}
