use std::io::{BufRead, self};
use std::fs::File;


pub fn get_history() -> Option<String> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
    }).ok()?;

    let history_path = home_dir.join(".bash_history");

    if !history_path.exists() {
        return None;
    }

    let file = File::open(history_path).ok()?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().ok()?;

    let mut commands: Vec<String> = Vec::new();
    for command in lines.iter().rev().take(200).rev() {
        commands.push(command.to_owned());
    }

    Some(commands.join("\n"))
}
