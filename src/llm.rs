use rig::prelude::*;
use std::env;
use rig::{
    providers::openai::{self, CompletionModel}
};
use rig::completion::Prompt;
use rig::prelude::*;

use rig::agent::Agent;
use regex::Regex;
use lancedb::query::{QueryBase, ExecutableQuery};
use futures::TryStreamExt;
use arrow_array::{StringArray, Array};

pub struct Llm {
    pub agent: Agent<CompletionModel>
}

pub fn remove_think_tags(output: &str) -> String {
    // (?s) enables "dot matches newline" mode
    let re = Regex::new(r"(?s)<think>.*?</think>").unwrap();
    let cleaned = re.replace_all(output, "");
    cleaned.trim().to_string()
}

async fn get_previous_command() -> Option<String> {
    let table = guesser::db::get_table().await.ok()?;
    let results = table.query()
    .only_if("id = 1") // SQL syntax: works with strings, numbers, etc.
    .limit(1)           // Optimization: stop after finding one
    .execute()
    .await.ok()?
    .try_collect::<Vec<_>>() // Collect stream of batches into a Vec
    .await.ok()?;

    // 3. Extract the data from the Arrow Batch
    let batch = results.first()?; // Returns None if empty

    // Get the specific column
    let col = batch.column_by_name("command_context")?;

    // Cast it to a String Array (Arrow is strongly typed)
    let strings = col.as_any().downcast_ref::<StringArray>()?;

    // Return the value (handling potential nulls)
    if strings.is_valid(0) {
        Some(strings.value(0).to_string())
    } else {
        None
    }
}

impl Llm {


    pub async fn init(history: &str) -> Llm {

        unsafe {
            if env::var("OPENAI_API_KEY").is_err() {
                env::set_var("OPENAI_API_KEY", "lm-studio");
            }

            if env::var("OPENAI_BASE_URL").is_err() {
                env::set_var("OPENAI_BASE_URL", "http://localhost:1234/v1");
            }
        }

        let client = openai::Client::builder("stuff").base_url(
            &env::var("OPENAI_BASE_URL").unwrap_or("http://localhost:1234/v1".to_string())
        ).build();


        let mut agent = client
            .completion_model("qwen/qwen3-14b")
            .completions_api()
            .into_agent_builder()
            .context(&format!("/no_think Only return one valid bash command. Try and predict the users next command.\n\n{}", history));

        agent = match (get_previous_command().await) {
            Some(prev_command) => {
                agent.context(&prev_command)
            }
            _ => {
                agent
            }
        };

        Llm {
            agent: agent.build()
        }
    }
}
