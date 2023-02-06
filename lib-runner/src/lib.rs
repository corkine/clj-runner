use std::{env, fs::{self}, process::Command};

static ENV_HINT: &str = "#!/usr/bin/env";

static CLOJURE_COMMENT: &str = ";";

static VESION: &str = "0.0.1";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn abc_test() {
        let file = "hello/world.clj".split("/").last().unwrap_or("");
        dbg!(&file);
        dbg!("hello world agg 123".split_whitespace()
        .map(|x|x.to_string())
        .reduce(|mut agg,item| {
            agg.push_str(&item);
            agg.push_str("---");
            agg
        }));
    }
}

fn ends_with_suffix(file_path: &str, match_suffix: &[&str]) -> bool {
    for ele in match_suffix {
        if file_path.ends_with(ele) { return true }
    }
    false
}

fn format_suffix(match_suffix: &[&str]) -> String {
    let mut str = String::new();
    let end = match_suffix.len();
    for (i, ele) in match_suffix.iter().enumerate() {
        str.push_str(ele);
        if i != end {
            str.push_str(" or ");
        }
    }
    str
}

/// 从命令行读取特定文件路径，如果合适，则对其进行首行命令调用。
/// match_suffix: 匹配的文件名后缀，包括 . 号，比如 .clj, .cljw 等。
pub fn run(match_suffix: &[&str]) {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(file_path) if ends_with_suffix(&file_path, match_suffix) => {
            handle_file(file_path);
        },
        _ => {
            let format_suffix = format_suffix(match_suffix);
            println!("clj-runner by rust, v{VESION}\n
pass a {format_suffix} file to execute! program will
find the first comment line and try to run 
it by pass the file path at the end of line.\n
eg. #!/usr/bin/env bb
eg. #!/usr/bin/env clojure -Spath ..
eg. ; clojure -Spath ..
eg. ; bb");
        }
    };
}

/// 对传入的 .clj 文件进行解析，获取其首个以 #!/usr/bin/env 开头或 ; 开头的行
/// 并且根据平台调用 powershell 或 bash 执行拼接了文件路径到此命令后的命令
/// 在程序执行结束后立刻返回
pub fn handle_file(file_path: String) {
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
    let full_cmds_line = match match first_line {
        line if first_line.starts_with(ENV_HINT) =>
        Some(line.replacen(ENV_HINT, "", 1)),
        line if first_line.starts_with(CLOJURE_COMMENT) =>
        Some(line.replacen(CLOJURE_COMMENT, "", 1)),
        _ => None,
    }.map(|mut l| {
        l.push_str(" ");
        l.push_str(&file_path);
        l
    }) { None => return, Some(l) => l, };
    let (shell, arg) = if cfg!(target_os="windows") {
        ("powershell","-NoProfile")
    } else {
        ("bash","-c")
    };
    if let Err(e) = Command::new(shell)
        .arg(arg)
        .arg(full_cmds_line)
        .spawn()
        .and_then(|mut res|res.wait()) {
        eprintln!("error when execute command: {:?}", e);
    }
}