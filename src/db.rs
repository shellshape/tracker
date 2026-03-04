use crate::model::{Entry, NewEntry};
use anyhow::Result;
use chrono::NaiveDate;
use include_dir::{Dir, include_dir};
use rusqlite::{Connection, params};
use rusqlite_migration::Migrations;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self> {
        let base_dir = base_dir.as_ref();
        fs::create_dir_all(base_dir)?;
        let mut conn = Connection::open(base_dir.join("db.sqlite"))?;
        MIGRATIONS.to_latest(&mut conn)?;
        Ok(Self { conn })
    }

    pub fn add(&self, entry: NewEntry) -> Result<()> {
        self.conn.execute(
            "INSERT INTO entry (date, time, message, long)
            VALUES (?, ?, ?, ?)",
            params![
                entry.timestamp.date(),
                entry.timestamp.time(),
                entry.message,
                entry.long
            ],
        )?;
        Ok(())
    }

    pub fn list(&self, date: NaiveDate) -> Result<Vec<Entry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, datetime(date || ' ' || time) as timestamp, message, long
            FROM entry WHERE date = ?",
        )?;
        let rows = stmt.query_map(params![date], |row| {
            Ok(Entry {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                message: row.get(2)?,
                long: row.get(3)?,
            })
        })?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn update(&self, entry: Entry) -> Result<()> {
        self.conn.execute(
            "UPDATE entry
            SET date = ?, time = ?, message = ?, long = ?
            WHERE id = ?",
            params![
                entry.timestamp.date(),
                entry.timestamp.time(),
                entry.message,
                entry.long,
                entry.id
            ],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: u32) -> Result<()> {
        self.conn.execute(
            "DELETE FROM entry
            WHERE id = ?",
            params![id],
        )?;
        Ok(())
    }
}
