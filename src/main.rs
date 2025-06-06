use std::env;
use std::process;
use clap::{Arg, Command};
use reqwest;
use serde_json::{json, Value};
use tokio;
use colored::*;

#[derive(Debug)]
struct GeminiClient {
    api_key: String,
    base_url: String,
}

impl GeminiClient {
    fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent".to_string(),
        }
    }

    async fn search(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        // Concise prompt for direct answers
        let enhanced_prompt = format!(
            "User query: '{}'

            Rules:
            - Be extremely concise and direct
            - If asking for installation: Only provide the exact commands needed, nothing else
            - If asking how to do something: Only provide the minimal steps/commands
            - No explanations, no background info, no troubleshooting unless specifically asked
            - No introductory text or conclusions
            - Format: Just the essential commands or answer

            Respond with only what's necessary to answer the query.",
            query
        );

        let payload = json!({
            "contents": [{
                "parts": [{
                    "text": enhanced_prompt
                }]
            }],
            "generationConfig": {
                "temperature": 0.3,
                "maxOutputTokens": 512,
            }
        });

        let url = format!("{}?key={}", self.base_url, self.api_key);

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }

        let response_json: Value = response.json().await?;

        if let Some(candidates) = response_json["candidates"].as_array() {
            if let Some(first_candidate) = candidates.first() {
                if let Some(content) = first_candidate["content"]["parts"].as_array() {
                    if let Some(text_part) = content.first() {
                        if let Some(text) = text_part["text"].as_str() {
                            return Ok(text.to_string());
                        }
                    }
                }
            }
        }

        Err("No valid response found in API response".into())
    }
}

fn format_output(content: &str) -> String {
    let mut formatted = String::new();
    let lines: Vec<&str> = content.lines().collect();

    for line in lines {
        if line.starts_with("# ") {
            formatted.push_str(&format!("{}\n", line.replace("# ", "").bold().blue()));
        } else if line.starts_with("## ") {
            formatted.push_str(&format!("{}\n", line.replace("## ", "").bold().green()));
        } else if line.starts_with("### ") {
            formatted.push_str(&format!("{}\n", line.replace("### ", "").bold().yellow()));
        } else if line.starts_with("```") {
            if line == "```" {
                formatted.push_str(&format!("{}\n", "───────────────────────────────────".dimmed()));
            } else {
                formatted.push_str(&format!("{}\n", format!("Code ({})", line.replace("```", "")).cyan().bold()));
            }
        } else if line.trim().starts_with("$") || line.trim().starts_with("sudo") {
            formatted.push_str(&format!("  {}\n", line.green().bold()));
        } else if line.starts_with("- ") || line.starts_with("* ") {
            formatted.push_str(&format!("  • {}\n", line[2..].white()));
        } else if line.chars().all(|c| c.is_numeric() || c == '.' || c == ' ') && line.contains('.') {
            formatted.push_str(&format!("  {}. {}\n", line.split('.').next().unwrap_or("").cyan(), line.split('.').skip(1).collect::<Vec<_>>().join(".")));
        } else {
            formatted.push_str(&format!("{}\n", line));
        }
    }

    formatted
}

#[tokio::main]
async fn main() {
    let matches = Command::new("Gemini Search CLI")
        .version("1.0")
        .author("Your Name")
        .about("AI-powered CLI search engine using Google's Gemini API")
        .arg(
            Arg::new("query")
                .help("Search query")
                .required(true)
                .num_args(1..)
        )
        .arg(
            Arg::new("raw")
                .short('r')
                .long("raw")
                .help("Output raw response without formatting")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Get API key from environment
    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("{}", "Error: GEMINI_API_KEY environment variable not set".red().bold());
            eprintln!("Please set your Gemini API key:");
            eprintln!("export GEMINI_API_KEY=your_api_key_here");
            process::exit(1);
        }
    };

    // Get search query
    let query_parts: Vec<&str> = matches.get_many::<String>("query").unwrap().map(|s| s.as_str()).collect();
    let query = query_parts.join(" ");

    if query.trim().is_empty() {
        eprintln!("{}", "Error: Please provide a search query".red().bold());
        process::exit(1);
    }

    println!("{}", format!("🔍 Searching for: {}", query).blue().bold());
    println!("{}", "━".repeat(50).dimmed());

    let client = GeminiClient::new(api_key);

    match client.search(&query).await {
        Ok(response) => {
            if matches.get_flag("raw") {
                println!("{}", response);
            } else {
                println!("{}", format_output(&response));
            }
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Search failed: {}", e).red().bold());
            process::exit(1);
        }
    }
}
