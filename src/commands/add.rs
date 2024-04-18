use super::Command;
use crate::store::Store;
use anyhow::Result;
use chrono::Local;
use clap::Args;
use inquire::Confirm;

#[derive(Args)]
pub struct Add {
    message: Vec<String>,

    #[arg(short, long)]
    long: Option<String>,
}

impl Command for Add {
    fn run(&self, store: &Store) -> Result<()> {
        if self.message.is_empty() {
            anyhow::bail!("can not use empty message value")
        }

        let timestamp = Local::now().naive_local();

        let long = match self.long {
            Some(ref v) => Some(v.clone()),
            None => prompt_long()?,
        };

        store.push_entry(timestamp, &self.message.join(" "), long.as_ref())?;
        Ok(())
    }
}

fn prompt_long() -> Result<Option<String>> {
    let ans = Confirm::new("Do you want to add a long description?")
        .with_default(false)
        .prompt()?;

    if !ans {
        return Ok(None);
    }

    let long = edit::edit("")?.trim().to_string();
    Ok(match long.is_empty() {
        true => None,
        false => Some(long),
    })
}
