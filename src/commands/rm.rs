use crate::commands::ls::create_path;
use crate::shell_core::BuiltinCommand;
use crate::shell_core::{parse_flags, validate_flags};
use std::fs;
use std::io;

pub fn run_rm(
    cmd: BuiltinCommand,
    args: &mut Vec<String>,
    flag_map: &mut std::collections::HashMap<BuiltinCommand, String>,
) -> io::Result<()> {
    parse_flags(cmd.clone(), args, flag_map);
    if validate_flags(cmd.clone(), flag_map) == false {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid flags"));
    }

    for i in args {

        // if i == "." || i == ".." {
        //     println!("rm: refusing to remove '.' or '..' directory: skipping '..'");
        //     continue;
        // }

        let path = create_path(String::from(".") ,i.clone());

        if path.symlink_metadata().is_err() {
            println!("rm: cannot remove '{}': No such file or directory", i);
            continue;
        }

        let metadata = path.symlink_metadata()?;

        if metadata.is_dir() {
            // println!("{:?}  ++++++   {:?}", metadata.file_type(), mp);
            if flag_map.contains_key(&cmd) && flag_map.get(&cmd) == Some(&"r".to_string()) {
                fs::remove_dir_all(path)?;
            } else {
                println!("rm: cannot remove '{}': Is a directory", i);
            }
        } else {
            fs::remove_file(path)?;
        }
    }
    
    Ok(())
}