use crate::commands::ls::create_path;
use crate::shell_core::BuiltinCommand;
use std::{env::*};
use std::path::*;

pub fn run_cd(
    _cmd: BuiltinCommand,
    args: &mut Vec<String>,
    flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
) {
    let user = whoami::username();

    if args.is_empty() {
        *args = vec![String::from("~")];
    }

    if args.len() > 1 {
        println!("cd: too many arguments");
        return;
    }

    if args[0] == "--" {
        args[0] = String::from("~");
    }

    let target_path = args[0].replace("~", &format!("/home/{}", user));

    if target_path == "-" {
        match flag_map.get(&BuiltinCommand::Cd).cloned() {
            Some(old_path) => {
                if !Path::new(&old_path).exists() {
                    println!(
                        "cd: {}: No such file or directory",
                        old_path.replace(&format!("/home/{}", user), "~")
                    );
                    return;
                }
                let current_before = flag_map.get(&BuiltinCommand::Pwd).cloned().or_else(|| get_current_dir());
                println!("{}", old_path.replace(&format!("/home/{}", user), "~"));
                if let Err(e) = set_current_dir(&old_path) {
                    println!("cd: {}: {}", old_path.replace(&format!("/home/{}", user), "~"), e);
                } else if let Some(prev) = current_before {
                    flag_map.insert(BuiltinCommand::Cd, prev);
                    flag_map.insert(BuiltinCommand::Pwd, old_path);
                }
            }
            None => {
                let current = get_current_dir().unwrap_or_else(|| format!("/home/{}", user));
                flag_map.insert(BuiltinCommand::Cd, current);
                flag_map.insert(BuiltinCommand::Pwd, format!("/home/{}", user));
                let _ = set_current_dir(&format!("/home/{}", user));
            }
        }
        return;
    }

    if !Path::new(&target_path).exists() {
        println!(
            "cd: {}: No such file or directory",
            target_path.replace(&format!("/home/{}", user), "~")
        );
        return;
    }

    if !Path::new(&target_path).is_dir() {
        println!("cd: {}: Not a directory", target_path.replace(&format!("/home/{}", user), "~"));
        return;
    }

    if !flag_map.contains_key(&BuiltinCommand::Pwd) {
        flag_map.insert(BuiltinCommand::Pwd, get_current_dir().unwrap_or_else(|| format!("/home/{}", user)));
    }

    let current_before = flag_map.get(&BuiltinCommand::Pwd).cloned();
    if let Err(e) = set_current_dir(&target_path) {
        println!("cd: {}: {}", target_path.replace(&format!("/home/{}", user), "~"), e);
        return;
    } else if let Some(prev) = current_before {
        flag_map.insert(BuiltinCommand::Cd, prev);
    }

    let pwd_path = if Path::new(&target_path).is_absolute() {
        target_path.clone()
    } else {
        let current = flag_map.get(&BuiltinCommand::Pwd).cloned().unwrap_or_else(|| "/".to_string());
        create_path(current, target_path.clone()).to_string_lossy().to_string()
    };

    let _current_pwd = flag_map.get(&BuiltinCommand::Pwd).cloned().unwrap_or_else(|| "/".to_string());

    let Ok(metadata) = Path::new(&pwd_path).symlink_metadata() else {
        flag_map.insert(BuiltinCommand::Pwd, get_current_dir().unwrap_or_else(|| "Unknown".to_string()));
        return;
    };

    if metadata.file_type().is_symlink() {
        flag_map.insert(BuiltinCommand::Pwd, pwd_path);
    } else {
        flag_map.insert(BuiltinCommand::Pwd, get_current_dir().unwrap_or_else(|| "Unknown".to_string()));
    }
}

fn get_current_dir() -> Option<String> {
    current_dir()
        .ok()
        .and_then(|p| {
            let path_str = p.to_string_lossy().to_string();
            if Path::new(&path_str).exists() {
                Some(path_str)
            } else {
                None
            }
        })
}