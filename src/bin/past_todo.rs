use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use rust::{
    config::{Config, Operation},
    opts::Opts,
    todoer::{Todoer, Todos},
};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;

pub fn get_proj(file: String) -> Result<Todoer> {
    let opts = Opts {
        args: vec![],
        config: None,
        filename: Some(file),
    };
    let config: Config = opts.try_into()?;
    return Ok(Todoer::from_config(config.config.clone(), true));
}

pub fn get_initial_todos(file: String) -> Result<()> {
    let proj = get_proj(file).unwrap();
    let value = proj.print_values();
    println!("{}", value);
    Ok(())
}

// function that allows user to select a file located in ~/todo
pub fn get_file() -> Result<String> {
    let home = std::env::var("HOME").unwrap();
    let mut home = PathBuf::from(home);
    home.push("todo");
    let paths = fs::read_dir(home)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let extension = e
                .path()
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .into_owned();
            if extension == "md" {
                Some(e.path().file_stem().unwrap().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&paths)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    let s = String::from(&paths[selection.unwrap()]).to_owned();

    Ok(s)
}

pub fn get_todos(file: String) -> Result<()> {
    let proj = get_proj(file).unwrap();
    let value = proj.print_values();
    println!("{}", value);
    Ok(())
}

pub fn get_delete_index(file: String) -> Result<String> {
    let proj = get_proj(file).unwrap();
    let size = usize::try_from(proj.size).unwrap();
    let mut new_todos = Vec::with_capacity(size);
    let Todos(todos) = proj.data;
    for _ in 0..todos.len() {
        new_todos.push(String::from(""));
    }

    for index in 0..todos.keys().len() {
        let index_t = index.try_into().unwrap();
        let todo = todos.get(&index_t).unwrap();
        new_todos[index] = todo.name.clone();
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&new_todos)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let select_num: u32 = selection.unwrap() as u32;
    Ok(select_num.to_string())
}

pub fn get_done_index(file: String) -> Result<String> {
    let proj = get_proj(file).unwrap();
    let size = usize::try_from(proj.size).unwrap();
    let done_count = usize::try_from(proj.done_count).unwrap();
    let not_done_size = size - done_count;
    let mut todo_index_map = HashMap::new();
    let mut not_done_todos = Vec::with_capacity(size);
    let Todos(todos) = proj.data;

    for _ in 0..not_done_size {
        not_done_todos.push(String::from(""));
    }

    let mut not_done_index = 0;
    for index in 0..todos.keys().len() {
        let index_t = index.try_into().unwrap();
        let todo = todos.get(&index_t).unwrap();
        if !todo.done {
            todo_index_map.insert(not_done_index, index);
            not_done_todos[not_done_index] = todo.name.clone();
            not_done_index += 1;
        }
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&not_done_todos)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let index_to_delete = todo_index_map.get(&selection.unwrap()).unwrap().clone();
    let select_num: u32 = index_to_delete as u32;
    Ok(select_num.to_string())
}

fn main() -> Result<()> {
    let file = get_file().unwrap();
    loop {
        print!("{esc}c", esc = 27 as char);
        get_initial_todos(file.clone()).unwrap();
        let mut args = vec![];
        let items = vec!["add", "done", "remove"];
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        let operation = String::from(items[selection.unwrap()]).to_owned();
        args.push(operation.clone());

        if operation == "add" {
            let input: String = Input::new().with_prompt(&operation).interact_text()?;
            args.push(input.clone());
        }

        if operation == "remove" {
            let index = get_delete_index(file.clone()).unwrap();
            args.push(index);
        }

        if operation == "done" {
            let index = get_done_index(file.clone()).unwrap();
            args.push(index);
        }

        let opts = Opts {
            args,
            config: None,
            filename: Some(file.clone()),
        };
        let config: Config = opts.try_into()?;
        let mut proj = Todoer::from_config(config.config.clone(), true);

        match config.operation {
            Operation::Print() => {}
            Operation::PrintAll() => {
                break;
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
    }

    Ok(())
}
