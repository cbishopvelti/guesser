use std::io::{BufRead, self};
use std::fs::File;


pub fn get_history() -> Option<String> {
    // 1. Find the user's home directory safely
    let home_dir = dirs::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
    }).ok()?;

    // 2. Construct path to .bash_history
    // Note: Some systems use $HISTFILE, but this is the standard default
    let history_path = home_dir.join(".bash_history");

    if !history_path.exists() {
        return None;
    }

    // 3. Open and read the file
    let file = File::open(history_path).ok()?;
    let reader = io::BufReader::new(file);

    // Collect lines into a vector (be careful with very large history files)
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().ok()?;

    // Print the last 5 commands
    let mut commands: Vec<String> = Vec::new();
    for command in lines.iter().rev().take(200).rev() {
        commands.push(command.to_owned());
    }

    Some(commands.join("\n"))
}
