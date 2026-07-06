# 0-shell (shell)

A compact, educational Unix-like command shell written in Rust.

This repository implements a minimal interactive shell with a small set of builtin commands (ls, cd, pwd, mkdir, rm, cp, mv, echo, cat, history, clear, touch, exit) and a simple lexer/tokenizer for command parsing. It focuses on implementing common shell behaviors (flags, quoting, basic path handling, and simple recursive copy/remove semantics) and is suitable as a learning/reference implementation.

**Table of contents**
- Project overview
- Features
- Project structure
- Installation
- Build
- Usage examples
- Configuration
- Dependencies
- Error handling
- Testing
- Example commands
- Contributing
- License

## Project overview

This project provides a small interactive shell implemented in Rust. It reads lines using `rustyline`, tokenizes input with a small `Lexer`, and dispatches recognized builtin commands from a `shell_core` module. The implementation aims to replicate common command behaviors and flag parsing for a subset of GNU coreutils-style commands.

## Features

- Interactive prompt with history (stored in `~/.zero-history.txt`).
- Builtin commands: `ls`, `cd`, `pwd`, `mkdir`, `rm`, `cp`, `mv`, `echo`, `cat`, `history`, `clear`, `touch`, `exit`.
- Basic flag parsing and validation for builtins.
- Quoting support and escaped characters in arguments.
- Recursive copy/remove semantics where appropriate (`-r`).
- Colored output for `ls` (directories, executables, devices, sockets, etc.).

## Project structure

- `Cargo.toml` — Cargo manifest and dependency list.
- `src/main.rs` — Program entry point and interactive prompt loop.
- `src/lib.rs` — Library root exposing `shell_core` and modules.
- `src/lexer/` — Tokenizer and lexer (`token.rs`) that produces tokens for parsing.
- `src/commands/` — Directory containing individual command implementations (one file per command):
  - `ls.rs`, `cd.rs`, `pwd.rs`, `mkdir.rs`, `rm.rs`, `cp.rs`, `mv.rs`, `echo.rs`, `cat.rs`, `history.rs`, `clear.rs`, `touch.rs`, `exit.rs`, etc.

Key modules and types
- `shell_core::BuiltinCommand` — enum of supported builtin commands.
- `shell_core::parse_flags` / `validate_flags` — flag parsing and validation utilities.
- `lexer::Lexer` / `lexer::Token` — tokenization primitives.

## Installation

Prerequisites:
- Rust toolchain (rustup recommended). Tested with recent stable Rust.

Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build

From the repository root:

```bash
# build in debug mode
cargo build

# or build a release binary
cargo build --release
```

## Run / Usage

Start the shell from the project root:

```bash
cargo run --quiet
```

The prompt shows a colored username and path. Commands are typed like a normal shell. Example interactions:

```text
$ ls -l
$ cd ~/projects
$ pwd
$ echo "hello world"
$ cat file.txt
$ cp -r src dest
$ rm -r some_directory
$ history -c
$ clear
$ exit
```

The lexer supports quoted strings and escaped characters. If the input has unclosed quotes the prompt will continue with `dquote>` to complete the quoted string.

## Configuration

- History file: `~/.zero-history.txt` (created and appended automatically).
- No additional config files are required.

## Dependencies

Dependencies are declared in `Cargo.toml`. Major runtime dependencies include:

- `rustyline` — line editing and history support.
- `whoami` — username discovery for home path expansion.
- `clearscreen` — clearing the terminal for `clear` builtin.
- `chrono`, `chrono-tz` — formatting file modification times for `ls -l`.
- `libc` and `users` — filesystem metadata, ACL checks and user/group mapping.
- `term_size` — terminal sizing utilities (used where applicable).

See `Cargo.toml` for exact versions.

## Error handling

- Tokenization errors are printed to stderr in `tokenize_input` (e.g., unclosed quotes). The REPL continues after tokenization errors.
- Invalid flags produce output similar to: `COMMAND: invalid option -- 'x'` and the flag state for that command is cleared.
- Commands that fail (I/O errors, permission denied) print descriptive errors (e.g., `ls: cannot open directory '...': Permission denied`).
- `exit` uses `std::process::exit` and accepts an optional numeric exit code; if the argument is not numeric the shell exits with status `255`.

## Testing

Run unit tests (if any) and run checks via cargo:

```bash
# run unit tests
cargo test

# fast compile-only check
cargo check
```

This repository contains a small codebase; any automated tests (if present) will be executed by `cargo test`.

## Example commands and notes

- `ls` supports flags like `-a`, `-l`, `-F`, `-r` (see `shell_core::validate_flags` for allowed flags).
- `rm -r` deletes directories recursively. Without `-r` it refuses to remove directories.
- `cp -r` copies directories recursively.
- `mv` attempts to rename and falls back to copy+remove when rename fails.
- `cat -n` prints line numbers.
- `history -c` clears history file.

## Contributing

Contributions are welcome. Suggested workflow:

- Fork the repository.
- Create a branch for your change: `git checkout -b feat/your-feature`.
- Keep changes focused and add tests where applicable.
- Open a pull request with a clear description of changes.

Coding conventions:
- Follow Rust naming conventions: `snake_case` for functions/variables/modules, `PascalCase` for types.
- Keep changes minimal and preserve existing behavior unless intentionally changing functionality.

## License

No license has been specified for this repository.

If you want to add a license, add a `LICENSE` file at the repository root and update `README.md` accordingly.

---

If you want, I can also:
- Expand the `Contributing` section into a CONTRIBUTING.md template.
- Add examples of `ls` output or include a short demo script to exercise the builtins.
- Add a small `Makefile` or convenience scripts for development tasks.
