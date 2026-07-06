use crate::shell_core::{parse_flags, validate_flags, BuiltinCommand};
use std::fs::* ;

pub fn run_touch(
    cmd: BuiltinCommand,
    args: &mut Vec<String>,
    flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
) {
    if args.is_empty() {
        println!("touch: missing file operand");
        return;
    }

    parse_flags(cmd.clone(), args, flag_map);
    if !validate_flags(cmd, flag_map) {
        return;
    }

    for filename in args.iter() {
        match OpenOptions::new()
            .create(true)
            .write(true)
            .open(filename)
        {
            Ok(_) => {}
            Err(e) => {
                println!("touch: cannot touch '{}': {}", filename, e);
            }
        }
    }
}