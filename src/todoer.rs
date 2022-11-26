use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::Lines;

#[derive(Debug)]
pub struct Todo {
    pub name: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct Todos(pub HashMap<u32, Todo>);

#[derive(Debug)]
pub struct Todoer {
    pub config: PathBuf,
    pub data: Todos,
    size: u32,
    done_count: u32,
}

pub fn default_data() -> Todos {
    Todos(HashMap::new())
}

impl<'a> TryFrom<Lines<'a>> for Todos {
    type Error = anyhow::Error;

    fn try_from(lines: Lines<'a>) -> Result<Self, Self::Error> {
        let mut index: u32 = 0;
        let mut data = HashMap::new();
        lines.for_each(|line| {
            let done = line.contains("[x]");
            let name = line.replace("- [ ] ", "").replace("- [x] ", "");
            let todo = Todo { name, done };
            data.insert(index, todo);
            index += 1;
        });
        Ok(Todos(data))
    }
}

impl TryFrom<String> for Todoer {
    type Error = anyhow::Error;

    fn try_from(data: String) -> Result<Self, Self::Error> {
        let mut lines = data.lines();

        let config = PathBuf::from(
            lines
                .next()
                .expect("Expect file path to be first line of file"),
        );
        let mut second_line = lines.next().unwrap().splitn(2, '/');
        let done_count = second_line
            .next()
            .unwrap()
            .parse()
            .expect("Expected done_count to be a int");
        let size = second_line
            .next()
            .unwrap()
            .parse()
            .expect("Expected size to be a int");
        let data = lines.try_into().expect("Error parsing todos");

        Ok(Todoer {
            config,
            data,
            done_count,
            size,
        })
    }
}

impl TryFrom<&Todoer> for String {
    type Error = anyhow::Error;

    fn try_from(todoer: &Todoer) -> Result<Self, Self::Error> {
        let filename = todoer.config.as_os_str().to_str().unwrap();

        let done_count = todoer.done_count.to_string();
        let size = todoer.size.to_string();

        let mut formatted_data = filename.to_owned() + "\n";
        formatted_data += &(done_count + "/" + &size + "\n");

        let Todos(todos) = &todoer.data;
        for index in 0..todos.keys().len().try_into().unwrap() {
            let todo = todos.get(&index).unwrap();
            if todo.done {
                formatted_data += "- [x] ";
            } else {
                formatted_data += "- [ ] ";
            }
            formatted_data += &todo.name;
            formatted_data += "\n";
        }

        Ok(formatted_data)
    }
}

impl Todoer {
    pub fn default_todoer(config: PathBuf) -> Self {
        Todoer {
            config,
            data: default_data(),
            size: 0,
            done_count: 0,
        }
    }
    pub fn get_value_names(&self) -> Vec<&String> {
        let mut ret = Vec::new();
        let mut index: u32 = 0;
        let Todos(todos) = &self.data;
        while index < self.size {
            ret.push(&todos[&index].name);
            index += 1;
        }
        ret
    }

    pub fn get_value_all(&self) -> Vec<(&String, bool)> {
        let mut ret = Vec::new();
        let Todos(todos) = &self.data;
        let mut index: u32 = 0;
        while index < self.size {
            ret.push((&todos[&index].name, todos[&index].done));
            index += 1;
        }
        ret
    }

    pub fn print_values(&self) -> String {
        let mut res = String::from("\nTodo\n");

        let Todos(todos) = &self.data;
        for index in 0..todos.keys().len().try_into().unwrap() {
            let todo = todos.get(&index).unwrap();
            if !todo.done {
                res += &(index.to_string() + "). " + &todo.name + "\n");
            }
        }

        res += &String::from("\nDone\n");
        for index in 0..todos.keys().len().try_into().unwrap() {
            let todo = todos.get(&index).unwrap();
            if todo.done {
                res += &(index.to_string() + "). " + &todo.name + "\n");
            }
        }
        res
    }

    pub fn set_value(&mut self, name: String) {
        self.data.0.insert(self.size, Todo { name, done: false });
        self.size += 1
    }

    pub fn remove_value(&mut self, index: u32) {
        let value = &self.data.0.get(&index).expect("Invalid index");
        if value.done {
            self.done_count -= 1;
        }
        for i in index + 1..self.size {
            let Todo { name, done } = self.data.0.get(&i).unwrap();
            self.data.0.insert(
                i - 1,
                Todo {
                    name: name.clone(),
                    done: *done,
                },
            );
        }
        self.data.0.remove(&(self.size - 1));
        self.size -= 1;
    }

    pub fn mark_done(&mut self, index: u32) {
        let todo = &self.data.0.get(&index).expect("Invalid index");
        let name = &todo.name;
        self.data.0.insert(
            index,
            Todo {
                name: name.to_string(),
                done: true,
            },
        );
        self.done_count += 1;
    }

    pub fn save(&self) -> Result<()> {
        if let Some(p) = self.config.parent() {
            if std::fs::metadata(&p).is_err() {
                std::fs::create_dir_all(p)?;
            }
        }
        let contents: String = self.try_into()?;
        std::fs::write(&self.config, contents)?;

        Ok(())
    }

    pub fn from_config(config: PathBuf) -> Self {
        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or_else(|_| String::from("{\"todos\":[]}"));
            return contents.try_into().expect("Error parsing data");
        }

        Todoer {
            config,
            data: default_data(),
            size: 0,
            done_count: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use super::{Todo, Todoer, Todos};

    fn get_data() -> HashMap<u32, Todo> {
        HashMap::from([
            (
                0,
                Todo {
                    name: "foo".into(),
                    done: true,
                },
            ),
            (
                1,
                Todo {
                    name: "bar".into(),
                    done: false,
                },
            ),
        ])
    }

    fn get_todoer() -> Todoer {
        Todoer {
            config: PathBuf::from(""),
            data: Todos(get_data()),
            size: 2,
            done_count: 1,
        }
    }

    #[test]
    fn set_value() {
        let mut proj = get_todoer();
        proj.set_value(String::from("fam"));

        assert_eq!(
            proj.get_value_names(),
            vec![
                &String::from("foo"),
                &String::from("bar"),
                &String::from("fam")
            ]
        );
    }

    #[test]
    fn remove_value() {
        let mut proj = get_todoer();
        proj.remove_value(0);

        assert_eq!(proj.get_value_names(), vec![&String::from("bar")]);
    }

    #[test]
    fn remove_value_end() {
        let mut proj = get_todoer();
        proj.set_value(String::from("fam"));
        proj.remove_value(2);

        assert_eq!(
            proj.get_value_names(),
            vec![&String::from("foo"), &String::from("bar")]
        );
    }

    #[test]
    fn remove_value_start() {
        let mut proj = get_todoer();
        proj.set_value(String::from("fam"));
        proj.remove_value(0);

        assert_eq!(proj.done_count, 0);

        assert_eq!(
            proj.get_value_names(),
            vec![&String::from("bar"), &String::from("fam")]
        );
    }

    #[test]
    fn get_value_all() {
        let mut proj = get_todoer();
        proj.set_value(String::from("fam"));
        assert_eq!(
            proj.get_value_all(),
            vec![
                (&String::from("foo"), true),
                (&String::from("bar"), false),
                (&String::from("fam"), false)
            ]
        );
    }

    #[test]
    fn mark_done() {
        let mut proj = get_todoer();
        proj.mark_done(1);

        assert_eq!(
            proj.get_value_all(),
            vec![(&String::from("foo"), true), (&String::from("bar"), true)]
        );
        assert_eq!(proj.done_count, 2);
    }

    #[test]
    fn print_values() {
        let proj = get_todoer();
        print!("{}", proj.print_values());
        assert_eq!(
            proj.print_values(),
            String::from("\nTodo\n1). bar\n\nDone\n0). foo\n")
        );
    }
}
