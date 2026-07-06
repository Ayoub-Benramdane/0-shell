pub mod commands;
pub mod lexer;

pub mod shell_core {
    use crate::commands::cd::run_cd;
    use crate::commands::rm::run_rm;
    use crate::commands::pwd::run_pwd;
    use crate::commands::mkdir::run_mkdir;
    use crate::commands::exit::run_exit;
    use crate::commands::cp::run_cp;
    use crate::commands::mv::run_mv;
    use crate::commands::cat::run_cat;
    use crate::commands::history::run_history;
    use crate::commands::clear::run_clear;
    use crate::commands::ls::run_ls;
    use crate::commands::touch::run_touch;
    use crate::commands::echo::run_echo;




    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum BuiltinCommand {
        Ls,
        Cd,
        Pwd,
        Mkdir,
        Rm,
        Cp,
        Mv,
        Echo,
        Cat,
        Exit,
        History,
        Clear,
        Touch,
    }

    impl BuiltinCommand {
        pub fn from_str(cmd: &str) -> Option<BuiltinCommand> {
            match cmd {
                "ls" => Some(BuiltinCommand::Ls),
                "cd" => Some(BuiltinCommand::Cd),
                "pwd" => Some(BuiltinCommand::Pwd),
                "mkdir" => Some(BuiltinCommand::Mkdir),
                "rm" => Some(BuiltinCommand::Rm),
                "cp" => Some(BuiltinCommand::Cp),
                "mv" => Some(BuiltinCommand::Mv),
                "echo" => Some(BuiltinCommand::Echo),
                "cat" => Some(BuiltinCommand::Cat),
                "exit" => Some(BuiltinCommand::Exit),
                "history" => Some(BuiltinCommand::History),
                "clear" => Some(BuiltinCommand::Clear),
                "touch" => Some(BuiltinCommand::Touch),
                _ => None,
            }
        }
    }

    pub fn execute_command(
        cmd: BuiltinCommand,
        args: &mut Vec<String>,
        flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
    ) {
        let mut cat_count = 1;
        match cmd {
            BuiltinCommand::Rm => {
                if let Err(_) = run_rm(cmd, args, flag_map) {
                    return;
                }
                flag_map.remove(&BuiltinCommand::Rm);
            }
            BuiltinCommand::Cd => run_cd(cmd, args, flag_map),
            BuiltinCommand::Mv => {
                run_mv(cmd, args, flag_map);
                flag_map.remove(&BuiltinCommand::Mv);
            }
            BuiltinCommand::Pwd => {
                run_pwd(cmd, args, flag_map)
            }
            BuiltinCommand::Mkdir => {
                run_mkdir(cmd, args, flag_map);
                flag_map.remove(&BuiltinCommand::Mkdir);
            }
            BuiltinCommand::Cp => {
                run_cp(cmd, args, flag_map);
                flag_map.remove(&BuiltinCommand::Cp);
            }
            BuiltinCommand::Exit => {
                run_exit(args);
                flag_map.remove(&BuiltinCommand::Exit);
            }
            BuiltinCommand::Cat => {
                let _ = run_cat(&mut cat_count, cmd, args, flag_map);
                flag_map.remove(&BuiltinCommand::Cat);
            }
            BuiltinCommand::Clear => {
                run_clear(cmd, args, flag_map);
                flag_map.remove(&BuiltinCommand::Clear);
            }
            BuiltinCommand::History => {
                run_history(cmd, args, flag_map);
                flag_map.remove(&BuiltinCommand::History);
            }
            BuiltinCommand::Ls => {
                run_ls(cmd, args, flag_map);
                flag_map.remove(&BuiltinCommand::Ls);
            }
            BuiltinCommand::Touch => {
                run_touch(cmd, args, flag_map);
                flag_map.remove(&BuiltinCommand::Touch);
            }
            BuiltinCommand::Echo => {
                run_echo(cmd, args, flag_map)
            }
        }
    }

    pub fn parse_flags(
        cmd: BuiltinCommand,
        args: &mut Vec<String>,
        flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
    ) {
        for arg in args.clone() {
            if arg.starts_with('-') && arg.len() > 1 {
                for flag in arg[1..].chars() {
                    let value = flag_map.entry(cmd.clone()).or_insert(flag.to_string());
                    if !value.contains(flag) {
                        value.push(flag);
                    }
                }
            }
        }
        args.retain(|arg| !arg.starts_with('-') || arg == "-");
    }

    pub fn validate_flags(
        cmd: BuiltinCommand,
        flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
    ) -> bool {
        return match cmd {
            BuiltinCommand::Rm => verify_allowed_flags(cmd.clone(), flag_map, "r".to_string()),
            BuiltinCommand::Mkdir => verify_allowed_flags(cmd.clone(), flag_map, String::new()),
            BuiltinCommand::Cp => verify_allowed_flags(cmd.clone(), flag_map, "r".to_string()),
            BuiltinCommand::Cat => verify_allowed_flags(cmd.clone(), flag_map, "n".to_string()),
            BuiltinCommand::Ls => verify_allowed_flags(cmd.clone(), flag_map, "alFr".to_string()),
            BuiltinCommand::Echo => verify_allowed_flags(cmd.clone(), flag_map, String::new()),
            BuiltinCommand::Clear => verify_allowed_flags(cmd.clone(), flag_map, String::new()),
            BuiltinCommand::Pwd => verify_allowed_flags(cmd.clone(), flag_map, String::new()),
            BuiltinCommand::Cd => verify_allowed_flags(cmd.clone(), flag_map, String::new()),
            BuiltinCommand::Mv => verify_allowed_flags(cmd.clone(), flag_map, String::new()),
            BuiltinCommand::History => verify_allowed_flags(cmd.clone(), flag_map, String::from("c")),
            BuiltinCommand::Touch => verify_allowed_flags(cmd.clone(), flag_map, String::new()),
            _ => true,
        };
    }

    pub fn report_invalid_flag(
        cmd: BuiltinCommand,
        flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
        flag: char,
    ) -> bool {
        flag_map.remove(&cmd);
        println!("{:?}: invalid option -- '{}'", cmd, flag);
        false
    }

    pub fn verify_allowed_flags(
        cmd: BuiltinCommand,
        flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
        allowed_flags: String,
    ) -> bool {
        if let Some(found_flags) = flag_map.get(&cmd) {
            for ch in found_flags.chars() {
                if !allowed_flags.contains(ch) {
                    return report_invalid_flag(cmd.clone(), flag_map, ch);
                }
            }
        }
        true
    }

}
