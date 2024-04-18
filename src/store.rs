use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

pub struct Store {
    base_dir: PathBuf,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    pub timestamp: NaiveDateTime,
    pub message: String,
    pub long: Option<String>,
}

impl TryFrom<&str> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        if value.is_empty() {
            anyhow::bail!("empty value")
        }

        let vals = value[1..value.len() - 1].replace("<NEWLINE>", "\n");
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

        let long = split.next().map(|v| v.replace("\\\"", "\"")).and_then(|v| {
            if v.is_empty() {
                None
            } else {
                Some(v)
            }
        });

        Ok(Self {
            timestamp,
            message,
            long,
        })
    }
}

impl Entry {
    fn to_csv(&self) -> Result<String> {
        use std::fmt::Write;

        let mut res = String::new();

        let long = match self.long {
            Some(ref v) => v.as_ref(),
            None => "",
        }
        .replace('"', "\\\"")
        .replace('\n', "<NEWLINE>");

        let msg = self.message.replace('"', "\\\"").replace('\n', "<NEWLINE>");
        // TODO: This is ugly AF and should write directly into a stream
        writeln!(res, r#""{}","{msg}","{long}""#, self.timestamp)?;

        Ok(res)
    }
}

impl Store {
    pub fn new<P: Into<PathBuf>>(base_dir: P) -> Result<Self> {
        let base_dir = base_dir.into();

        fs::create_dir_all(&base_dir)?;

        Ok(Self { base_dir })
    }

    pub fn push_entry(&self, entry: Entry) -> Result<()> {
        let mut track_file = self.get_track_file(&entry.timestamp)?;
        write!(track_file, "{}", entry.to_csv()?)?;

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
            .map(|line| line.try_into())
            .collect()
    }

    pub fn set(&self, date: NaiveDate, entries: Vec<Entry>) -> Result<()> {
        let path = self.get_track_file_name(date);
        let mut track_file = File::create(path)?;

        for e in entries {
            write!(track_file, "{}", e.to_csv()?)?;
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
