use crate::config::Config;
use crate::store::Entry;
use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDate};
use fancy_duration::{AsFancyDuration, AsTimes};
use inquire::DateSelect;
use regex::Regex;
use std::fmt;
use std::str::FromStr;
use yansi::{Paint, Style};

static STYLE_START: Style = Style::new().cyan();
static STYLE_PAUSE: Style = Style::new().green();
static STYLE_END: Style = Style::new().cyan();

impl Entry {
    pub fn timestamp_formatted(&self) -> String {
        let time = self.timestamp.time();
        time.format("%H:%M").to_string()
    }

    pub fn formatted<D, T>(
        &self,
        config: &Config,
        long: bool,
        duration: Option<D>,
    ) -> Result<String>
    where
        D: AsFancyDuration<T>,
        T: AsTimes + Clone,
    {
        let mut res = String::new();
        self.format(&mut res, config, long, duration)?;
        Ok(res)
    }

    pub fn format<D, T>(
        &self,
        mut f: impl fmt::Write,
        config: &Config,
        long: bool,
        duration: Option<D>,
    ) -> Result<()>
    where
        D: AsFancyDuration<T>,
        T: AsTimes + Clone,
    {
        write!(
            f,
            "{} {} {}",
            self.timestamp_formatted().rgb(244, 9, 84),
            ":".dim(),
            self.style_message(config)?
        )?;

        if !long && self.long.is_some() {
            write!(f, " {}", "[...]".dim().italic())?;
        }

        if let Some(duration) = duration {
            write!(
                f,
                " {}{}{}",
                '('.dim(),
                duration.fancy_duration().truncate(2).dim(),
                ')'.dim()
            )?;
        }

        if long && let Some(ref long_message) = self.long {
            write!(f, "\n{}", prefix_lines(long_message, "\r\t").italic())?;
        }

        Ok(())
    }

    pub fn message_matches(&self, rx: &str) -> Result<bool> {
        Ok(Regex::new(rx)?.is_match(&self.message))
    }

    fn style_message<'a>(&'a self, config: &Config) -> Result<Box<dyn fmt::Display + 'a>> {
        Ok(if self.message_matches(&config.start_regex)? {
            Box::new(self.message.paint(STYLE_START))
        } else if self.message_matches(&config.break_regex)? {
            Box::new(self.message.paint(STYLE_PAUSE))
        } else if self.message_matches(&config.end_regex)? {
            Box::new(self.message.paint(STYLE_END))
        } else {
            Box::new(&self.message)
        })
    }
}

/// Splits the given string by newline character (`\n`) and prepends
/// the given prefix in front of each line.
fn prefix_lines(long: &str, prefix: &str) -> String {
    let mut str = String::new();

    let mut split = long.split('\n');

    let Some(line) = split.next() else {
        return str;
    };

    str.push_str(prefix);
    str.push_str(line);

    for line in split {
        str.push('\n');
        str.push_str(prefix);
        str.push_str(line);
    }

    str
}

#[derive(Clone)]
pub struct Parsable<T: Clone>(pub T);

impl FromStr for Parsable<NaiveDate> {
    type Err = anyhow::Error;

    /// Parses a given date string as following:
    ///
    /// The following table assumes the current date as `2025-07-24` for the
    /// examples.
    ///
    /// | Format | Description | Example | Resulting Date |
    /// |--------|-------------|---------|----------------|
    /// | `<yyyy>-<mm>-<dd>` | Concrete date. | `2025-07-24` | `2025-07-24` |
    /// | `<mm>-<dd>` | Concrete date; year is taken from current date. | `07-24` | `2025-07-24` |
    /// | `<dd>` | Concrete date; year and month is taken from current date. | `24` | `2025-07-24` |
    /// | `-<n_days>` | Today minus <n_days>. | `-2` | `2025-07-22` |
    /// | `y[y...]` | Today minus count of 'y'. | `yyy` | `2025-07-21` |
    fn from_str(date: &str) -> std::result::Result<Self, Self::Err> {
        let today = Local::now().date_naive();

        let y_count = date.chars().take_while(|&c| c == 'y' || c == 'Y').count();
        if y_count > 0 {
            if y_count != date.len() {
                return Err(anyhow::anyhow!("additional characters after 'y'"));
            }
            return Ok(Self(today - Duration::days(y_count as i64)));
        }

        if let Some(days_str) = date.strip_prefix('-') {
            let days = days_str.parse()?;
            return Ok(Self(today - Duration::days(days)));
        }

        let delims = date.chars().filter(|&c| c == '-').count();

        let year = today.year();
        let month = today.month0() + 1;

        let date = match delims {
            0 => format!("{year}-{month}-{date}"),
            1 => format!("{year}-{date}"),
            2 => date.to_string(),
            _ => return Err(anyhow::anyhow!("invalid date format")),
        };

        Ok(Self(NaiveDate::parse_from_str(&date, "%Y-%m-%d")?))
    }
}

pub fn select_date() -> Result<NaiveDate> {
    let date = DateSelect::new("Select Date").prompt()?;
    Ok(date)
}

pub struct FormatableEntry<'c, 'e> {
    pub config: &'c Config,
    pub long: bool,
    pub entry: &'e Entry,
}

impl<'c, 'e> FormatableEntry<'c, 'e> {
    pub fn new(entry: &'e Entry, config: &'c Config, long: bool) -> Self {
        Self {
            entry,
            config,
            long,
        }
    }
}

impl fmt::Display for FormatableEntry<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.entry
            .format(f, self.config, self.long, None::<chrono::TimeDelta>)
            .map_err(|_| fmt::Error)
    }
}

#[cfg(test)]
mod test {
    use crate::util::Parsable;
    use chrono::{Datelike, Duration, Local, NaiveDate};

    #[test]
    fn format_long() {
        assert_eq!("\t", super::prefix_lines("", "\t"));
        assert_eq!("\t\n\t\n\t\n\t", super::prefix_lines("\n\n\n", "\t"));
        assert_eq!("\tfoo", super::prefix_lines("foo", "\t"));
        assert_eq!("\tfoo\n\tbar", super::prefix_lines("foo\nbar", "\t"));
        assert_eq!("\tfoo\n\tbar\n\t", super::prefix_lines("foo\nbar\n", "\t"));
    }

    #[test]
    fn parse_date() {
        fn get_date<S: AsRef<str>>(date: S) -> NaiveDate {
            NaiveDate::parse_from_str(date.as_ref(), "%Y-%m-%d").unwrap()
        }

        fn get_date_parts(date: NaiveDate) -> (u32, u32, u32) {
            let (_, year) = date.year_ce();
            let month = date.month0() + 1;
            let day = date.day0() + 1;
            (year, month, day)
        }

        let (year, month, _) = get_date_parts(Local::now().date_naive());
        assert_eq!(
            get_date("2025-07-24"),
            "2025-07-24".parse::<Parsable<NaiveDate>>().unwrap().0
        );
        assert_eq!(
            get_date(format!("{year}-07-24",)),
            "07-24".parse::<Parsable<NaiveDate>>().unwrap().0
        );
        assert_eq!(
            get_date(format!("{year}-{month}-24",)),
            "24".parse::<Parsable<NaiveDate>>().unwrap().0
        );

        assert_eq!(
            (Local::now() - Duration::days(1)).date_naive(),
            "-1".parse::<Parsable<NaiveDate>>().unwrap().0
        );
        assert_eq!(
            (Local::now() - Duration::days(2)).date_naive(),
            "-2".parse::<Parsable<NaiveDate>>().unwrap().0
        );
        assert_eq!(
            (Local::now() - Duration::days(69)).date_naive(),
            "-69".parse::<Parsable<NaiveDate>>().unwrap().0
        );

        assert_eq!(
            (Local::now() - Duration::days(1)).date_naive(),
            "y".parse::<Parsable<NaiveDate>>().unwrap().0
        );
        assert_eq!(
            (Local::now() - Duration::days(2)).date_naive(),
            "yY".parse::<Parsable<NaiveDate>>().unwrap().0
        );
        assert_eq!(
            (Local::now() - Duration::days(6)).date_naive(),
            "YYYYYY".parse::<Parsable<NaiveDate>>().unwrap().0
        );
    }
}
