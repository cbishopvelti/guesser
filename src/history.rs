use std::io::{BufRead, self};
use std::fs::File;


pub fn get_history() -> Option<String> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
    }).ok()?;

    let shell = std::env::var("SHELL").unwrap_or_default();
    let history_path = if shell.contains("zsh") {
        home_dir.join(".zsh_history")
    } else if shell.contains("bash") {
        home_dir.join(".bash_history")
    } else {
        home_dir.join(".bash_history")
    };

    if !history_path.exists() {
        return None;
    }

    let file = File::open(history_path).ok()?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().ok()?;

    let mut commands: Vec<String> = Vec::new();
    for command in lines.iter().rev().take(200).rev() {
        let clean_command = match command.find(';') {
            Some(index) => &command[index + 1..], // Slice the string after the ';'
            None => command,
        };

        commands.push(
            //: 1765108497:0;aws-azure-login --mode gui --profile prod
            clean_command.to_owned()
        );
    }

    Some(commands.join("\n"))
}
