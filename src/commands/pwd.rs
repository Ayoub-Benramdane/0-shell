use std::env::current_dir;
use crate::shell_core::BuiltinCommand;
use crate::shell_core::{parse_flags, validate_flags};

pub fn run_pwd(
    cmd: BuiltinCommand,
    args: &mut Vec<String>,
    flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
) {
    parse_flags(cmd.clone(), args, flag_map);
    if validate_flags(cmd.clone(), flag_map) == false {
        return;
    }
    let current_dir = current_dir();
    let curr = flag_map.get(&BuiltinCommand::Pwd);
    if curr.is_none() {
        if let Ok(path) = current_dir {
            let path_str = path.to_string_lossy().to_string();
            println!("{}", path_str);
            flag_map.insert(BuiltinCommand::Pwd, path_str);
        } else {
            println!("pwd: error retrieving current directory");
        }
    } else {
        println!("{}", curr.unwrap());
    }
    
}
