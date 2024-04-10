[![CICD](https://github.com/BenGale93/cli-todo/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/BenGale93/cli-todo/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/cli-todo.svg)](https://crates.io/crates/cli-todo)

# cli-todo

Keep a todo list using the cli.

## Setup

Clone this repository and `cd` into it. Then run `cargo install --path .`

You'll then need to create a new sqlite database to store your todos.

You can just place it in the current directory using `todo init`, or you can
use the `-d` flag for more control. For example, `todo init -d ~/todo.db`.

This location will be recorded in a config file placed here:
`~/.config/todo/default-config.toml`.

## Usage

The CLI has the usual CRUD commands for managing your todos. Run `todo
<COMMAND> --help` for more info.

```bash
todo add email "Email John about a thing" Mon1700

todo go email # Now in progress

todo go email # Now finished.
```
