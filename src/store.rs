use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

pub struct Store {
    base_dir: PathBuf,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    pub timestamp: NaiveDateTime,
    pub message: String,
    pub long: Option<String>,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            anyhow::bail!("empty value")
        }

        let vals = s[1..s.len() - 1].replace("<NEWLINE>", "\n");
        let mut split = vals.split("\",\"");

        let timestamp_str = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("no timestamp value"))?
            .replace("\\\"", "\"");

        let timestamp = NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S%.f")?;

        let message = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("no message value"))?
            .replace("\\\"", "\"");

        let long = split
            .next()
            .map(|v| v.replace("\\\"", "\""))
            .and_then(|v| if v.is_empty() { None } else { Some(v) });

        Ok(Self {
            timestamp,
            message,
            long,
        })
    }
}

impl Entry {
    pub fn to_csv<W: Write>(&self, mut w: W) -> Result<()> {
        let long = match self.long {
            Some(ref v) => v.as_ref(),
            None => "",
        }
        .replace('"', "\\\"")
        .replace("\r\n", "<NEWLINE>")
        .replace('\n', "<NEWLINE>");

        let msg = self.message.replace('"', "\\\"").replace('\n', "<NEWLINE>");
        writeln!(w, r#""{}","{msg}","{long}""#, self.timestamp)?;

        Ok(())
    }
}

impl Store {
    pub fn new<P: Into<PathBuf>>(base_dir: P) -> Result<Self> {
        let base_dir = base_dir.into();

        fs::create_dir_all(&base_dir)?;

        Ok(Self { base_dir })
    }

    pub fn push_entry(&self, entry: Entry) -> Result<()> {
        let track_file = self.get_track_file(&entry.timestamp)?;
        entry.to_csv(track_file)?;
        Ok(())
    }

    pub fn list(&self, date: NaiveDate) -> Result<Vec<Entry>> {
        let path = self.get_track_file_name(date);

        if !path.exists() {
            return Ok(vec![]);
        }

        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;

        buf.split('\n')
            .filter(|l| !l.is_empty())
            .map(|line| line.parse())
            .collect()
    }

    pub fn set(&self, date: NaiveDate, entries: Vec<Entry>) -> Result<()> {
        let path = self.get_track_file_name(date);
        let mut track_file = File::create(path)?;

        for e in entries {
            e.to_csv(&mut track_file)?
        }

        Ok(())
    }

    fn get_track_file_name(&self, date: NaiveDate) -> PathBuf {
        self.base_dir
            .join(format!("{}.log", date.format("%Y-%m-%d")))
    }

    fn get_track_file(&self, timestamp: &NaiveDateTime) -> Result<File> {
        let date = timestamp.date();
        let path = self.get_track_file_name(date);
        Ok(match !path.exists() {
            true => File::create(path)?,
            false => File::options().append(true).open(path)?,
        })
    }
}
