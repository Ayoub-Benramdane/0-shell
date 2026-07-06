use rustyline::{Editor, error::ReadlineError};
use shell::lexer::token::{has_unclosed_quotes, tokenize_input};
use shell::shell_core::*;
use std::collections::HashMap;

fn main() -> rustyline::Result<()> {
    let mut editor = Editor::<(), _>::new()?;
    let user = whoami::username();
    let his_path = format!("/home/{}/.zero-history.txt", user);
    let _ = editor.load_history(&his_path);
    let mut state_map = HashMap::new();

    loop {
        let last_place = state_map.get(&BuiltinCommand::Pwd);
        let current_place = std::env::current_dir();
        let mut path = if let Ok(place) = current_place && !state_map.contains_key(&BuiltinCommand::Pwd) {
            place.display().to_string()
        } else {
            last_place.unwrap_or(&"Unknown error".to_string()).to_string()
        };
        path = path.replace(&("/home/".to_owned() + &user), "~");

        let mut input_line = match editor.readline(&format!("{}:{} $ ", col_user(), col_path(path))) {
            Ok(line) => {
                let _ = editor.add_history_entry(line.as_str());
                let _ = editor.append_history(&his_path);
                line
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(_) => {
                break;
            }
        };

        while has_unclosed_quotes(&input_line) {
            match editor.readline("dquote> ") {
                Ok(additional_input) => {
                    input_line.push('\n');
                    input_line.push_str(&additional_input);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    input_line = String::new();
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    input_line = String::new();
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }

        let mut tokenized_commands = tokenize_input(&input_line, &user);

        for command_tokens in tokenized_commands.iter_mut() {
            match BuiltinCommand::from_str(&command_tokens[0]) {
                Some(cmd) => {
                    execute_command(cmd, &mut command_tokens[1..].to_owned(), &mut state_map);
                }
                None => {
                    println!("Command '{}' not found", command_tokens[0]);
                }
            }
        }
    }
    Ok(())
}

pub fn col_user() -> String {
    let user = whoami::username();
    format!("\x1b[1;32m{}\x1b[0m", user)
}

pub fn col_path(path: String) -> String {
    format!(
        "\x1b[1;33m[<\x1b[0m\x1b[1;1m{}\x1b[0m\x1b[1;33m>]\x1b[0m",
        path
    )
}
