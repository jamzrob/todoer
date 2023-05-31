use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use rust::{
    config::{Config, Operation},
    opts::Opts,
    todoer::Todoer,
};

use anyhow::Result;

pub fn print_todos() -> Result<()> {
    let print_opts = Opts {
        args: vec![],
        config: None,
    };
    let print_config: Config = print_opts.try_into()?;
    let print_proj = Todoer::from_config(print_config.config.clone());
    let print_value = print_proj.print_values();
    println!("{}", print_value);
    Ok(())
}

fn main() -> Result<()> {
    print_todos().unwrap();
    let mut args = vec![];
    let items = vec!["add", "done", "remove"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let operation = String::from(items[selection.unwrap()]).to_owned();
    if operation != "print" {
        args.push(operation.clone());
    }

    let input: String = Input::new().with_prompt(&operation).interact_text()?;
    args.push(input.clone());

    let opts = Opts { args, config: None };
    let config: Config = opts.try_into()?;
    let mut proj = Todoer::from_config(config.config.clone());

    match config.operation {
        Operation::Print() => {}
        Operation::PrintAll() => {}
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
