# ToDo.rs

A small to-do list for your terminal, written in Rust. Add a task, tick it off, find it later. Your list lives in a plain JSON file that you can open and read yourself.


## Getting Started

You'll need [Rust](https://www.rust-lang.org/tools/install) 1.85 or newer.

```sh
git clone https://github.com/NamelessProj/todo.rs.git
cd todo.rs
cargo install --path .
```

This gives you a `todo.rs` command. If you'd rather not install it, run it from the project folder with `cargo run --` followed by the command, for example `cargo run -- print`.


## A Quick Tour

```sh
$ todo.rs add "Buy milk"
$ todo.rs add "Write the report" --description "Due Friday"
$ todo.rs mark 1
$ todo.rs print
[x] #1 Buy milk
[ ] #2 Write the report
    Due Friday
```

Adding, toggling and removing print nothing when they work. Use `print` to see the result.


## Commands

| Command | What it does |
| --- | --- |
| `add <NAME>` | Add a task |
| `edit <ID>` | Change a task's name, description or status |
| `mark <ID>` | Mark a task as done or not done |
| `print [ID]` | Show all taks or just one |
| `search` | Find tasks by ID or text |
| `remove <ID>` | Delete a task |

A status is either `done` or `undone`.


### Add A Task

```sh
todo.rs add "Call the dentist"
todo.rs add "Call the dentist" -d "Ask about Tuesday"
todo.rs add "Water the planr" -s done       # already done
```

New tasks are not done unless you say otherwise.


## Edit A Task

```sh
todo.rs edit 1 --name "Buy oat milk"
todo.rs edit 1 -d "Two cartons"
todo.rs edit 1 -s undone
todo.rs edit 1 -n "Buy bread" -s done       # several changes at once
```

Gives at least one of `--name`, `--description` or `--status`. Anything you leave out stays as it is.


### Mark A Task

```sh
todo.rs mark 1              # done
todo.rs mark 1 -s done      # also done
todo.rs mark 1 -s undone    # not done
```


### Print Tasks

```sh 
todo.rs print       # everything
todo.rs print 2     # just task #2
```


### Search

```sh
todo.rs search --text report                    # in names and description
todo.rs search --text report --name             # in names only
todo.rs search --text friday --description      # in descriptions only
todo.rs search -i 2                             # by ID
```

Text search ignores case and matches parts of words. Search by `--id` or by `--text`, one at a time. `--name` and `--description` only work with `--text`.


### Remove A Task

```sh
todo.rs remove 2
```

> [!WARNING]
> There's no undo!


### Help

Run `todo.rs --help` or `todo.rs <COMMAND> --help`, to see the available options. `todo.rs --version` shows the version.


## Where Your Tasks Are Saved

Tasks are stored in `todo.json` **in the folder you run the command from**. Each folder therefore gets it's own list, which is handy for per-project to-dos. If your tasks seems to have vanished, check which folder you're in.

The file is created the first time you run a command that needs it. It looks like this:

```json
[
    {
        "id": 1,
        "name": "Buy milk",
        "done": true,
        "description": null
    }
]
```

You can edit it by hand, as long as it's stays valid JSON.

A new task gets the highest current ID plus one, so if you remove the last task, it's number is used again.


## Project Layout

| File | What's in it |
| ---- | --- |
| `src/main.rs` | The `Task` type and what ech command does |
| `src/command.rs` | Command arguments and options |
| `src/file.rs` | Loading and saving `todo.json` |
| `src/help.rs` | Search, printing and ID numbering |

Built with [Clap](https://crates.io/crates/clap), [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json).


## License

MIT. See [LICENSE](./LICENSE).