use std::path::PathBuf;

use anyhow::{anyhow, Ok, Result};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> std::result::Result<Self, Self::Error> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;
        return Ok(Config {
            operation,
            config,
            pwd,
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect as already checked");

        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!("expected two args got {}", value.len() - 1));
            }
            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }

        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!("expected one args got {}", value.len() - 1));
            }
            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow!("expected 0 or 1 args got {}", value.len()));
        }
        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let loc = std::env::var("HOME").expect("unable to get XDG_CONFIG_HOME");
    let mut loc = PathBuf::from(loc);
    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }
    return Ok(std::env::current_dir().expect("error getting cur dir"));
}

#[cfg(test)]
mod test {
    use anyhow::{Ok, Result};

    use crate::{config::Operation, opts::Opts};

    use super::Config;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));

        return Ok(());
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["foo".to_string()],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some("foo".to_string())));

        return Ok(());
    }

    #[test]
    fn test_print_add_val() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["add".to_string(), "foo".to_string(), "bar".to_string()],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(
            opts.operation,
            Operation::Add("foo".to_string(), "bar".to_string())
        );

        return Ok(());
    }

    #[test]
    fn test_print_remove() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["rm".to_string(), "foo".to_string()],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Remove("foo".to_string()));

        return Ok(());
    }
}
