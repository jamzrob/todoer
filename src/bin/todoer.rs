
use clap::Parser;
use rust::{opts::Opts, config::{Config, Operation}, todoer::Todoer};

use anyhow::Result;

fn main () -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let mut proj = Todoer::from_config(config.config);

    match config.operation {
        Operation::Print() => {
            let value = proj.print_values();

            println!("{}", value);
        },
        Operation::Add(v) => {
            proj.set_value(v);
            proj.save()?;
        },
        Operation::Complete(i) => {
            proj.mark_done(i);
            proj.save()?;
        },
        Operation::Remove(i) => {
            proj.remove_value(i);
            proj.save()?;
        },
    }

    return Ok(());
}
