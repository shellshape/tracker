use anyhow::Result;
use chrono::NaiveDateTime;
use std::io::Write;

pub struct NewEntry {
    pub timestamp: NaiveDateTime,
    pub message: String,
    pub long: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    pub id: u32,
    pub timestamp: NaiveDateTime,
    pub message: String,
    pub long: Option<String>,
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
