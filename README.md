# Todoer

A simple CLI application allowing one to keep track of a todo list. ***Written in Rust***

Todos are defaulted to save under one's home directory under todos/ named via the current date if no config is specified.


### How to 

Specify where the todo file will be created
```
   cargo run --bin todoer -- --config "~(PATH)/<filename>.md"
```
1. Print current todos
```
  cargo run --bin todoer
```
2. Add todo
```
  cargo run --bin todoer add "Write CLI application to keep track of todos"
```
3. Mark todo as done (based on index)
```
  cargo run --bin todoer done 0
```
4. Remove todo (based on index)
```
  cargo run --bin todoer remove 0
```


