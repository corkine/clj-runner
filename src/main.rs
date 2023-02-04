use std::{env, fs::{self}, process::Command};

static ENV_HINT: &str = "#!/usr/bin/env";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn abc_test() {
        let file = "hello/world.clj".split("/").last().unwrap_or("");
        dbg!(&file);
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(file_path) if file_path.ends_with(".clj") => {
            handle_file(file_path);
        },
        _ => eprintln!("need input .clj file"),
    };
}

fn handle_file(file_path: String) {
    let file_name = match file_path.split("/").last() {
        None => &file_path,
        Some(file) => file,
    };
    let contents = match fs::read_to_string(&file_path) {
        Err(_) => {
            eprintln!("file can't open!");
            return;
        }
        Ok(content) => content,
    };
    let first_line = contents
        .lines()
        .filter(|line|!line.is_empty())
        .take(1)
        .next()
        .unwrap_or("");
    if !first_line.starts_with(ENV_HINT) { 
        eprintln!("not find exec command");
        return;
     }
    let mut full_cmds_line = first_line.replace(ENV_HINT, "");
    full_cmds_line.push_str(" ");
    full_cmds_line.push_str(&file_path);
    if cfg!(target_os="windows") {
        Command::new("powershell")
            .arg("-NoProfile")
            //commands[1..].to_owned()
            .arg(full_cmds_line)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(full_cmds_line)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}