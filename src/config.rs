use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::opts::Opts;

use chrono::{Datelike, Local};

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let filename = value.filename;
        let config = get_config(value.config, filename)?;

        Ok(Config { operation, config })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Print(),
    PrintAll(),
    Add(String),
    Complete(u32),
    Remove(u32),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;
        if value.is_empty() {
            return Ok(Operation::Print());
        }

        let term = value.get(0).expect("expect to exist");

        if term == "add" {
            if value.len() != 2 {
                let err = anyhow!(
                    "operation add expects 1 arguments but got {}",
                    value.len() - 1
                );
                return Err(err);
            }

            let arg = value.pop().expect("to exist");
            return Ok(Operation::Add(arg));
        }

        if term == "done" {
            if value.len() != 2 {
                let err = anyhow!(
                    "operation done expects 1 arguments but got {}",
                    value.len() - 1
                );
                return Err(err);
            }

            let arg = value.pop().expect("to exist");
            return Ok(Operation::Complete(
                arg.parse().expect("Please type a valid number"),
            ));
        }

        if term == "remove" {
            if value.len() != 2 {
                let err = anyhow!(
                    "operation remove expects 1 arguments but got {}",
                    value.len() - 1
                );
                return Err(err);
            }

            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(
                arg.parse().expect("Please type a valid number"),
            ));
        }

        if value.len() > 1 {
            let err = anyhow!(
                "operation print expects 0 or 1 arguments but got {}",
                value.len()
            );
            return Err(err);
        }

        Ok(Operation::PrintAll())
    }
}

pub fn get_config(config: Option<PathBuf>, filename: Option<String>) -> Result<PathBuf> {
    let now = Local::now();
    let current_date_filename = format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());
    let f = filename.unwrap_or(current_date_filename);

    if let Some(mut c) = config {
        c.push("wiki");
        c.push("todo");
        c.push(format!("{}.md", f));
        return Ok(c);
    }

    if let Ok(home) = std::env::var("XDG_CONFIG_HOME") {
        let mut home = PathBuf::from(home);
        home.push("wiki");
        home.push("todo");
        home.push(format!("{}.md", f));
        return Ok(home);
    }

    if let Ok(home) = std::env::var("HOME") {
        let mut home = PathBuf::from(home);
        home.push("wiki");
        home.push("todo");
        home.push(format!("{}.md", f));
        print!("{}", home.display());
        return Ok(home);
    }

    Err(anyhow!("unable to find config location"))
}

#[cfg(test)]
mod test {

    use anyhow::Result;

    use crate::{config::Operation, opts::Opts};

    use super::Config;

    use std::path::PathBuf;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            config: Some(PathBuf::from("")),
            filename: None,
        }
        .try_into()?;

        assert_eq!(opts.config, PathBuf::from(""));
        assert_eq!(opts.operation, Operation::Print());
        Ok(())
    }

    #[test]
    fn test_add_todo() -> Result<()> {
        let opts: Config = Opts {
            args: vec![String::from("add"), String::from("foo")],
            config: None,
            filename: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Add(String::from("foo")));
        Ok(())
    }

    #[test]
    fn test_complete_todo() -> Result<()> {
        let opts: Config = Opts {
            args: vec![String::from("done"), String::from("1")],
            config: None,
            filename: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Complete(1));
        Ok(())
    }

    #[test]
    fn test_remove_todo() -> Result<()> {
        let opts: Config = Opts {
            args: vec![String::from("remove"), String::from("1")],
            config: None,
            filename: String::new(),
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Remove(1));
        Ok(())
    }
}
