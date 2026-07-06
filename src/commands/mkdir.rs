use crate::shell_core::BuiltinCommand;
use crate::shell_core::{parse_flags, validate_flags};
use std::path::Path;

pub fn run_mkdir(
    _cmd: BuiltinCommand,
    args: &mut Vec<String>,
    flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
) {
    if args.len() < 1 {
        println!("mkdir: missing operand");
        return;
    }

    parse_flags(BuiltinCommand::Mkdir, args, flag_map);
    if !validate_flags(BuiltinCommand::Mkdir, flag_map) {
        return;
    }

    // println!("{:?}", args);

    for dir in args.iter() {
        let path = Path::new(dir);
        if path.exists() {
            eprintln!("mkdir: cannot create directory '{}': File exists", dir);
            continue;
        }
        if let Err(_) = std::fs::create_dir(dir) {
            continue;
        }
    }
}
