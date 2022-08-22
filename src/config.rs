use crate::opts::Opts;
use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        Ok(Config {
            operation,
            config,
            pwd,
        })
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

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;

        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).unwrap();
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!(
                    "operation add expects 2 arguments, but got {}",
                    value.len() - 1
                ));
            }

            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(drain.next().unwrap(), drain.next().unwrap()));
        }

        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "operation remove expects 1 arguments, but got {}",
                    value.len() - 1
                ));
            }

            return Ok(Operation::Remove(value.pop().unwrap()));
        }

        if value.len() > 1 {
            return Err(anyhow!(
                "operation expects 0 or 1 arguments, but got {}",
                value.len()
            ));
        }

        let arg = value.pop().unwrap();
        Ok(Operation::Print(Some(arg)))
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let location = std::env::var("PROJECTOR_CONFIG_PATH").unwrap_or_else(|_| String::from("/"));
    let mut location = PathBuf::from(location);

    location.push("projector");
    location.push("projector.json");

    Ok(location)
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    Ok(std::env::current_dir().context("unable to get current working dir")?)
}

#[cfg(test)]
mod test {
    use super::Config;
    use crate::{config::Operation, opts::Opts};
    use anyhow::Result;

    #[test]
    fn print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));
        Ok(())
    }

    #[test]
    fn print_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["foo".to_string()],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some("foo".to_string())));
        Ok(())
    }

    #[test]
    fn add_key_value() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["add".to_string(), "foo".to_string(), "bar".to_string()],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(
            opts.operation,
            Operation::Add("foo".to_string(), "bar".to_string())
        );
        Ok(())
    }

    #[test]
    fn rm_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["rm".to_string(), "foo".to_string()],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Remove("foo".to_string()));
        Ok(())
    }
}
