use super::Command;
use crate::{store::Store, util::parse_date};
use anyhow::Result;
use chrono::Local;
use clap::Args;
use yansi::Paint;

/// Display tracking list entries
#[derive(Args)]
pub struct View {
    /// Date of the list
    date: Option<String>,

    /// Display additional description
    #[arg(short, long)]
    long: bool,
}

impl Command for View {
    fn run(&self, store: &Store) -> Result<()> {
        let date = match self.date {
            Some(ref date_str) => parse_date(date_str)?,
            None => Local::now().date_naive(),
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
            if self.long {
                print!("{e:#}");
            } else {
                print!("{e}");
            }
            println!();
        }

        Ok(())
    }
}
