use crate::shell_core::BuiltinCommand;
use crate::shell_core::{parse_flags, validate_flags};

pub fn run_clear(
    _cmd: BuiltinCommand,
    args: &mut Vec<String>,
    _flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
) {
    parse_flags(BuiltinCommand::Clear, args, _flag_map);
    if !validate_flags(BuiltinCommand::Clear, _flag_map) {
        return;
    }
    clearscreen::clear().expect("Failed to clear terminal");
}
