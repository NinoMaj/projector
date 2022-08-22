use anyhow::Result;
use clap::Parser;
use projector::{
    config::{Config, Operation},
    opts::Opts,
    projector::Projector,
};

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let mut proj = Projector::from_config(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string(&value)?;

            println!("{}", value);
        }
        Operation::Print(Some(key)) => {
            proj.get_value(&key).map(|value| {
                println!("{}", value);
            });
        }
        Operation::Add(key, value) => {
            proj.set_value(&key, &value);
            proj.save()?;
        }
        Operation::Remove(key) => {
            proj.remove_value(&key);
            proj.save()?;
        }
    }

    Ok(())
}
