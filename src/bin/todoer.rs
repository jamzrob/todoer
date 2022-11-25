use clap::Parser;
use rust::{
    config::{Config, Operation},
    opts::Opts,
    todoer::Todoer,
    todoers::Todoers,
};

use anyhow::Result;

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let mut proj = Todoer::from_config(config.config);

    match config.operation {
        Operation::Print() => {
            let value = proj.print_values();

            println!("{}", value);
        }
        Operation::PrintAll() => {
            let config: Config = Opts::parse().try_into()?;
            let projs =
                Todoers::from_todos_dir(config.config.parent().unwrap().to_path_buf()).unwrap();

            println!("{}", projs.print_all_todos_together());
        }
        Operation::Add(v) => {
            proj.set_value(v);
            proj.save()?;
        }
        Operation::Complete(i) => {
            proj.mark_done(i);
            proj.save()?;
        }
        Operation::Remove(i) => {
            proj.remove_value(i);
            proj.save()?;
        }
    }

    Ok(())
}
