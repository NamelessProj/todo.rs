# ToDo.rs

A small to-do list for your terminal, written in Rust. Add a task, tick it off, find it later. Your list lives in a plain JSON file that you can open and read yourself.


## Getting Started

You'll need [Rust](https://www.rust-lang.org/tools/install) 1.85 or newer.

```sh
git clone https://github.com/NamelessProj/todo.rs.git
cd todo.rs
cargo install --path .
```

This gives you a `todo.rs` command. If you'd rather not install it, run it from the project folder with `cargo run --` followed by the command, for example `cargo run -- show`.


## A Quick Tour

```sh
$ todo.rs add --task "Buy milk"
$ todo.rs add --task "Write the report" --description "Due Friday"
$ todo.rs toggle --id 1 --done
$ todo.rs show
[x] #1 Buy milk
[ ] #2 Write the report
    Due Friday
```

Adding, toggling and removing print nothing when they work. Use `show` to see the result.


## Commands


### Add A Task

```sh
todo.rs add -t "Call the dentist"
todo.rs add -t "Call the dentist" -d "Ask about Tuesday"
todo.rs add -t "Water the planr" --done      # already done
```


### Mark A Task Done Or Not Done

```sh
todo.rs toggle -i 1 --done      # done
todo.rs toggle -i 1             # not done
```

Despite it's name, `toggle` doesn't flip the current state. It's sets it: with `--done` the task is done, without it the task is open again.


### Show Tasks

```sh 
todo.rs show        # everything
todo.rs show -i 2   # just task #2
```


### Search

```sh
todo.rs search --text report                    # in names and description
todo.rs search --text report --name             # in names only
todo.rs search --text friday --description      # in descriptions only
todo.rs search -i 2                             # by ID
```

Text search ignores case and matches parts of words. Search by `--id` or by `--text`, one at a time.


### Remove A Task

```sh
todo.rs remove -i 2
```

> [!WARNING]
> There's no undo!

Run `todo.rs --help` or `todo.rs <COMMAND> --help`, to see the available options.


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

You can edit it by hand, as long as it's stays valid JSON.é

A new task gets the highest current ID plus one, so if you remove the last task, it's number is used again.


## Project Layout

| File | What's in it |
| ---- | --- |
| `src/main.rs` | Command definitions and what they do |
| `src/file.rs` | Loading and saving `todo.json` |
| `src/help.rs` | Search, printing and ID numbering |

Built with [Clap](https://crates.io/crates/clap), [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json).


## License

MIT. See [LICENSE](./LICENSE).