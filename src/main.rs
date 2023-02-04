use std::{env, fs::{self}, process::Command};

static ENV_HINT: &str = "#!/usr/bin/env";

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
    let contents = match fs::read_to_string(file_path) {
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
    let commands: Vec<&str> = first_line
        .split(" ")
        .collect();
    if commands.is_empty() {
        eprintln!("not find exec program name");
        return;
    }
    if cfg!(target_os="windows") {
        Command::new("powershell")
            .arg("-NoProfile")
            //commands[1..].to_owned()
            .arg(first_line.replace(ENV_HINT, "").as_str())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(first_line.replace(ENV_HINT, "").as_str())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}