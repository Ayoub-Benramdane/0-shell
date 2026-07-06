use crate::shell_core::BuiltinCommand;
use crate::shell_core::{parse_flags, validate_flags};
use crate::commands::rm::run_rm;
use crate::commands::cat::run_cat;

pub fn run_history(
    cmd: BuiltinCommand,
    args: &mut Vec<String>,
    flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
) {
    parse_flags(cmd.clone(), args, flag_map);
    if !validate_flags(cmd.clone(), flag_map) {
        return;
    }
    if args.len() > 0 {
        println!("history: too many arguments");
        return;
    }
    let user = whoami::username();
    let his_path = format!("/home/{}/.zero-history.txt", user) ;
    if flag_map.contains_key(&cmd) && flag_map.get(&cmd) == Some(&"c".to_string()) {
        let _ = run_rm(cmd.clone(), &mut vec![his_path.to_string()], flag_map);
        flag_map.remove(&cmd);
    } else {
        let mut cat_count = 1;
        let _ = run_cat(&mut cat_count, BuiltinCommand::Cat, &mut vec![his_path.to_string(), "-n".to_string()], flag_map);
    }
    
}