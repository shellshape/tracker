use super::Command;
use crate::config::Config;
use crate::store::{Entry, Store};
use crate::util::{Parsable, select_date};
use anyhow::Result;
use chrono::{Duration, Local, NaiveDate, TimeDelta};
use clap::Args;
use crossterm::event::{Event, KeyCode};
use crossterm::{cursor, event, execute, terminal};
use fancy_duration::AsFancyDuration;
use scopeguard::defer;
use std::io;
use yansi::Paint;

macro_rules! println_cr {
    ($($arg:tt)*) => {
        print!("{}\r\n", format!($($arg)*));
    }
}

/// Display tracking list entries
#[derive(Args)]
#[command(visible_aliases = ["v"])]
pub struct View {
    /// Date of the list
    date: Option<Parsable<NaiveDate>>,

    /// Select date from an interactive calender
    #[arg(short, long)]
    select: bool,

    /// Display additional description
    #[arg(short, long)]
    long: bool,

    /// Output entries as CSV
    #[arg(long)]
    csv: bool,

    /// Interactively page through days
    #[arg(short, long)]
    paging: bool,
}

impl Command for View {
    fn run(&self, store: &Store, config: &Config) -> Result<()> {
        let date = match self.date {
            Some(Parsable(date_str)) => date_str,
            None if self.select => select_date()?,
            _ => Local::now().date_naive(),
        };

        if self.paging {
            return paging_view(store, config, date, self.long);
        }

        let mut entries = store.list(date)?;
        entries.sort_by_key(|e| e.timestamp);

        if self.csv {
            for e in entries {
                e.to_csv(io::stdout())?;
            }
            return Ok(());
        }

        print_entries(config, &entries, self.long)
    }
}

fn print_entries(config: &Config, entries: &[Entry], long: bool) -> Result<()> {
    if entries.is_empty() {
        println_cr!("{}", "There are no entries for this day.".italic().dim());
        return Ok(());
    }

    let mut sum = Duration::zero();
    let mut pause_time = Duration::zero();
    let mut last_timestamp = None;

    for (i, e) in entries.iter().enumerate() {
        let duration = match last_timestamp {
            Some(last_timestamp) => {
                let duration = e.timestamp - last_timestamp;
                match e.message_matches(&config.break_regex)? {
                    true => pause_time += duration,
                    false => sum += duration,
                }
                Some(duration)
            }
            None => None,
        };

        last_timestamp = Some(e.timestamp);

        print!(
            "{}{}{} ",
            "[".dim(),
            format!("{:>2}", i + 1).cyan().dim(),
            "]".dim(),
        );

        println_cr!("{}", e.formatted(config, long, duration)?);
    }

    println_cr!(
        "\n     {} ({})",
        sum.fancy_duration().truncate(2).to_string().cyan().bold(),
        format!("{} pause", pause_time.fancy_duration().truncate(2)).green()
    );

    Ok(())
}

fn paging_view(store: &Store, config: &Config, start_date: NaiveDate, long: bool) -> Result<()> {
    terminal::enable_raw_mode()?;
    defer! {
        terminal::disable_raw_mode().ok();
    }

    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    defer! {
        execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show).ok();
    }

    let (term_width, _) = terminal::size()?;

    let mut date = start_date;

    loop {
        let mut entries = store.list(date)?;
        entries.sort_by_key(|e| e.timestamp);

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        let header_text = format!(
            "{:<30} | [← / h] prev. Day | [→ / j] next Day | [esc / q] quit",
            date.format("%A, %-d %B, %C%y")
        );
        println_cr!(
            "{}\n",
            format_args!(" {header_text:<width$}", width = term_width as usize - 1)
                .on_bright_black()
        );

        print_entries(config, &entries, long)?;

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Right | KeyCode::Char('h') => date += TimeDelta::days(1),
                KeyCode::Left | KeyCode::Char('l') => date -= TimeDelta::days(1),
                KeyCode::Down | KeyCode::Char('j') => date += TimeDelta::days(7),
                KeyCode::Up | KeyCode::Char('k') => date -= TimeDelta::days(7),
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('c') => break,
                _ => {}
            }
        }
    }

    Ok(())
}
