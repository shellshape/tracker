use chrono::{Datelike, Duration, Local, NaiveDate};
use fancy_duration::FancyDuration;
use std::str::FromStr;

#[derive(Clone, Default)]
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

impl FromStr for Parsable<Duration> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(FancyDuration::parse(s)?.duration()))
    }
}
