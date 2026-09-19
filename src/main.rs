

use std::io::{self, Write};

use reqwest::Client;
use serde::{Deserialize, Serialize};

const OLLAMA_URL: &str = "http://localhost:11434/v1/chat/completions";
const MODEL: &str = "qwen3:8b";

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a helpful AI assistant.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "What is an ERP?".to_string(),
        },
    ];

    loop {
        print!("You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        messages.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

          let response = ask_ai(messages.clone()).await?;

          println!("AI:{}", response.choices[0].message.content);

          messages.push(Message {
            role: "assistant".to_string(),
            content: response.choices[0].message.content.clone(),
        });
    }
  
    Ok(())
}


async fn ask_ai(messages: Vec<Message>) -> Result<ChatResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let request = ChatRequest {
        model: MODEL.to_string(),
        messages
    };

    let response = client
        .post(OLLAMA_URL)
        .header("Authorization", "Bearer ollama")
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json::<ChatResponse>()
        .await?;
    Ok(response)
}