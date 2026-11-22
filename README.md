# lst

A better way to view directory contents in my shell. Looks better, easier to read, and just as fast as `ls`.

## Core Idea

The `ls` command is powerful but in large directories it can be unsatisfying or even challenging to view all the contents of a project at a glance. Many alternatives exist such as `tree` (see [here](https://github.com/peteretelej/tree)) and `eza` (see [here](https://github.com/eza-community/eza)). This project is _not_ meant to be a replacement for those, rather it's my opinionated take on what I'd like my `lst` command to show me.

Heavily inspired by the `nushell` appearance of `ls`.

## Installation

Requires a Rust toolchain ([rustup](https://rustup.rs)):

```bash
cargo install --git https://github.com/frznprograms/lst
```

## Pre-Requisites

1. Your terminal should be able to print ANSI colours.
2. You should have a [Nerd Font](https://www.nerdfonts.com) installed, so that icons render properly.

## CLI Commands

```
lst [OPTIONS] [PATH]
```

| Flag                 | Short | Description                                                                                           | Default           |
| -------------------- | ----- | ----------------------------------------------------------------------------------------------------- | ----------------- |
| `[PATH]`             |       | Directory to list                                                                                     | current directory |
| `--all`              | `-a`  | Show hidden files (those starting with a dot), like `ls -a`                                           | off               |
| `--time`             | `-t`  | Add "Created" and "Last Modified" columns                                                             | off               |
| `--depth <N>`        | `-d`  | How many levels deep to list (`1` = only the directory's own contents)                                | no limit          |
| `--no-ignore`        |       | Don't apply `.gitignore` rules                                                                        | off               |
| `--ignore <PATTERN>` |       | Leave out entries matching a gitignore-style pattern, e.g. `"*.toml"`. Repeat it for several patterns | none              |
| `--help`             | `-h`  | Print help                                                                                            |                   |
| `--version`          | `-V`  | Print version                                                                                         |                   |

Short flags can be combined, e.g. `lst -at -d 2 src`.

### Default Behaviour

Running just `lst` lists the whole tree below the current directory, skipping hidden and gitignored files. Directories are shown in cyan, and each name is prefixed with a Nerd Font icon (left out of the samples below, since most browsers can't display them):

```
╭──────────────────┬──────┬──────────╮
│       Name       │ Type │   Size   │
├──────────────────┼──────┼──────────┤
│ Cargo.lock       │ file │ 11.87 KB │
│ Cargo.toml       │ file │   298  B │
│ README.md        │ file │  1.94 KB │
│ src/             │ dir  │        - │
│   cli.rs         │ file │   876  B │
│   lib.rs         │ file │    45  B │
│   main.rs        │ file │  1.47 KB │
│   traverse.rs    │ file │  2.33 KB │
│   util/          │ dir  │        - │
│     entries.rs   │ file │  5.57 KB │
│     icons.rs     │ file │  5.05 KB │
│     ignore.rs    │ file │  1.34 KB │
│     mod.rs       │ file │    48  B │
╰──────────────────┴──────┴──────────╯
```

### Examples

Only the top level, with timestamps (`lst -t -d 1`):

```
╭──────────────┬──────┬──────────┬──────────────────┬──────────────────╮
│     Name     │ Type │   Size   │     Created      │  Last Modified   │
├──────────────┼──────┼──────────┼──────────────────┼──────────────────┤
│ Cargo.lock   │ file │ 11.87 KB │ 2026-11-01 06:24 │ 2026-11-15 10:49 │
│ Cargo.toml   │ file │   298  B │ 2026-11-15 10:49 │ 2026-11-22 08:30 │
│ README.md    │ file │  1.94 KB │ 2026-11-14 09:42 │ 2026-11-22 09:04 │
│ src/         │ dir  │        - │ -                │ -                │
╰──────────────┴──────┴──────────┴──────────────────┴──────────────────╯
```

Everything, including dotfiles and gitignored files, but not lock files:

```bash
lst -a --no-ignore --ignore "*.lock"
```

P.S. Please don't roast me, it's my first Rust project XD.
