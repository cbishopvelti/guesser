pub mod llm;
pub mod history;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, BufRead, BufReader, Write};
use std::env;
use rig::{
    message::Message, providers::openai
};
use rig::completion::Prompt;
use rig::prelude::*;
use regex::Regex;
use crate::llm::Llm;

fn remove_think_tags(output: &str) -> String {
    // (?s) enables "dot matches newline" mode
    let re = Regex::new(r"(?s)<think>.*?</think>").unwrap();
    let cleaned = re.replace_all(output, "");
    cleaned.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let history = format!("Historic bash commands:\n {}", history::get_history().unwrap());

    let llm = Llm::init(&history).await;

    let args: Vec<String> = env::args().collect();

    let mut query = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };

    let response = llm.agent
        .prompt(query)
        .await?;

    println!("{}", remove_think_tags(&response));

    Ok(())
}
