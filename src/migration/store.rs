use crate::model::Entry;
use anyhow::Result;
use chrono::NaiveDateTime;
use std::fs::{File, ReadDir};
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Err(anyhow::anyhow!("empty value"));
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
            id: 0,
            timestamp,
            message,
            long,
        })
    }
}

pub struct Store {
    rd: ReadDir,
    buf: String,
}

impl Store {
    pub fn new<P: Into<PathBuf>>(base_dir: P) -> Result<Self> {
        let rd = base_dir.into().read_dir()?;
        let buf = String::new();
        Ok(Self { rd, buf })
    }
}

impl Iterator for Store {
    type Item = Result<(PathBuf, Vec<Entry>)>;

    fn next(&mut self) -> Option<Self::Item> {
        let file_entry = match self.rd.next()? {
            Ok(e) if e.path().extension().is_some_and(|ext| ext == "log") => e,
            Ok(_) => return self.next(),
            Err(err) => return Some(Err(err.into())),
        };

        self.buf.clear();
        if let Err(err) = File::open(file_entry.path()).map(|mut f| f.read_to_string(&mut self.buf))
        {
            return Some(Err(err.into()));
        }

        let entries: Result<Vec<_>, _> = self
            .buf
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|line| line.parse())
            .collect();

        Some(entries.map(|v| (file_entry.path(), v)))
    }
}
