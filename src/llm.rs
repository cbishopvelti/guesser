use rig::prelude::*;
use std::env;
use rig::{
    providers::openai::{self, CompletionModel}
};
use rig::completion::Prompt;
use rig::prelude::*;

use rig::agent::Agent;
use regex::Regex;

pub struct Llm {
    pub agent: Agent<CompletionModel>
}

pub fn remove_think_tags(output: &str) -> String {
    // (?s) enables "dot matches newline" mode
    let re = Regex::new(r"(?s)<think>.*?</think>").unwrap();
    let cleaned = re.replace_all(output, "");
    cleaned.trim().to_string()
}

impl Llm {
    pub fn init(history: &str) -> Llm {

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

        let agent = client
            .completion_model("qwen/qwen3-14b")
            .completions_api()
            .into_agent_builder()
            .preamble(&format!("/no_think Only return one valid bash command. Try and predict the users next command.\n\n{}", history))
            .build();

        Llm{
            agent: agent
        }
    }
}
