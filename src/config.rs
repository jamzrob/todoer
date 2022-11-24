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
        let config = get_config(value.config)?;

        Ok(Config { operation, config })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Print(),
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

        Ok(Operation::Print())
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(c) = config {
        return Ok(c);
    }

    let now = Local::now();
    let filename = format!("{}-{:02}-{:02}", now.year() % 100, now.month(), now.day());

    if let Ok(home) = std::env::var("XDG_CONFIG_HOME") {
        let mut home = PathBuf::from(home);
        home.push("todo");
        home.push(format!("{}.md", filename));
        return Ok(home);
    }

    if let Ok(home) = std::env::var("HOME") {
        let mut home = PathBuf::from(home);
        home.push("todo");
        home.push(format!("{}.md", filename));
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
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Remove(1));
        Ok(())
    }
}
