# Command-Line Help for `tracker`

This document contains the help content for the `tracker` command-line program.

**Command Overview:**

* [`tracker`↴](#tracker)
* [`tracker add`↴](#tracker-add)
* [`tracker view`↴](#tracker-view)
* [`tracker delete`↴](#tracker-delete)
* [`tracker edit`↴](#tracker-edit)
* [`tracker insert`↴](#tracker-insert)

## `tracker`

Simple tool to do time tracking

**Usage:** `tracker [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `add` — Add a track entry
* `view` — Display tracking list entries
* `delete` — Remove entries from a tracking list
* `edit` — Edit an entry from a tracking list
* `insert` — Swaps the next entry with the given timestamp and sets the next entries info to the given info

###### **Options:**

* `-c`, `--config <CONFIG>` — Path to a config file



## `tracker add`

Add a track entry

**Usage:** `tracker add [OPTIONS] [MESSAGE]...`

**Command Alias:** `a`

###### **Arguments:**

* `<MESSAGE>` — A short message

###### **Options:**

* `-t`, `--time <TIME>` — Time to set the entry at
* `-d`, `--date <DATE>` — Date to set the entry at
* `-s`, `--select` — Select date from an interactive calender to set entry at
* `-l`, `--long` — Add a long description by opening an editor
* `--long-text <LONG_TEXT>` — Add a long description as text content



## `tracker view`

Display tracking list entries

**Usage:** `tracker view [OPTIONS] [DATE]`

**Command Alias:** `v`

###### **Arguments:**

* `<DATE>` — Date of the list

###### **Options:**

* `-s`, `--select` — Select date from an interactive calender
* `-l`, `--long` — Display additional description
* `--csv` — Output entries as CSV



## `tracker delete`

Remove entries from a tracking list

**Usage:** `tracker delete [OPTIONS] [DATE]`

**Command Alias:** `d`

###### **Arguments:**

* `<DATE>` — Date of the list

###### **Options:**

* `-s`, `--select` — Select date from an interactive calender



## `tracker edit`

Edit an entry from a tracking list

**Usage:** `tracker edit [OPTIONS] [DATE]`

**Command Alias:** `e`

###### **Arguments:**

* `<DATE>` — Date of the list

###### **Options:**

* `-s`, `--select` — Select date from an interactive calender
* `-l`, `--last` — Edit the latest added entry



## `tracker insert`

Swaps the next entry with the given timestamp and sets the next entries info to the given info

**Usage:** `tracker insert [OPTIONS] [MESSAGE]...`

**Command Alias:** `i`

###### **Arguments:**

* `<MESSAGE>` — A short message

###### **Options:**

* `-t`, `--time <TIME>` — Time to set the entry at
* `-d`, `--date <DATE>` — Date to set the entry at
* `-s`, `--select` — Select date from an interactive calender to set entry at
* `-l`, `--long` — Add a long description by opening an editor
* `--long-text <LONG_TEXT>` — Add a long description as text content



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

