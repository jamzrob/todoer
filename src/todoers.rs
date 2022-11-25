use crate::todoer::{Todoer, Todos};
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Todoers {
    pub todoers: Vec<Todoer>,
}

pub fn default_data() -> HashMap<String, Todoer> {
    HashMap::new()
}
impl Todoers {
    pub fn from_todos_dir(todos_dir: PathBuf) -> Result<Self> {
        let todoers = std::fs::read_dir(&todos_dir)?
            .map(|res| {
                res.map(|entry| {
                    let path = entry.path();
                    if std::fs::metadata(&path).is_ok() {
                        let contents = std::fs::read_to_string(&path).unwrap();
                        let data: Todoer = contents.try_into().expect("Error parsing data");
                        return data;
                    }
                    Todoer::default_todoer(path)
                })
            })
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        Ok(Todoers { todoers })
    }

    pub fn print_all_todos(&self) -> String {
        let mut all = String::from("\n");
        let tododers = &self.todoers;
        tododers.iter().for_each(|todoer| {
            all += "\n";
            all += todoer.config.as_os_str().to_str().unwrap();
            all += &todoer.print_values();
        });
        all
    }

    pub fn print_all_todos_together(&self) -> String {
        let mut res = String::from("\nTodo\n");
        let tododers = &self.todoers;
        let mut index = 0;
        tododers.iter().for_each(|todoer| {
            let Todos(todos) = &todoer.data;
            todos.values().for_each(|todo| {
                if !todo.done {
                    res += &(index.to_string() + "). " + &todo.name + "\n");
                    index += 1;
                }
            });
        });
        res += "\nDone\n";
        let mut index = 0;
        tododers.iter().for_each(|todoer| {
            let Todos(todos) = &todoer.data;
            todos.values().for_each(|todo| {
                if todo.done {
                    res += &(index.to_string() + "). " + &todo.name + "\n");
                    index += 1;
                }
            });
        });
        res
    }
}
