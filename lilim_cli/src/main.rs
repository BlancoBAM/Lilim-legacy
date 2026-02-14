use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const API_URL: &str = "http://localhost:8080/chat";
const SESSION_FILE: &str = "/tmp/lilim_session.txt";

#[derive(Parser)]
#[command(name = "lilim")]
#[command(about = "Lilim AI Assistant CLI - Lilith Linux", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ask Lilim a question
    Ask {
        /// The question to ask
        question: Vec<String>,
    },
    /// Search the knowledge base
    Search {
        /// Search term
        term: Vec<String>,
    },
    /// Show conversation history
    History,
    /// Clear session memory
    Clear,
    /// Check server status
    Status,
}

#[derive(Serialize)]
struct LilimQuery {
    text: String,
    session_id: String,
    tools_enabled: bool,
    yolo_mode: bool,
}

#[derive(Deserialize)]
struct LilimResponse {
    response: String,
    source: String,
    domain: String,
}

fn get_session_id() -> Result<String> {
    if let Ok(session) = fs::read_to_string(SESSION_FILE) {
        Ok(session.trim().to_string())
    } else {
        let session = format!("cli_{}", chrono::Utc::now().timestamp());
        fs::write(SESSION_FILE, &session)?;
        Ok(session)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ask { question } => {
            let query_text = question.join(" ");
            
            if query_text.trim().is_empty() {
                eprintln!("{}", "Error: Question cannot be empty".red());
                std::process::exit(1);
            }

            println!("{}", "🔥 Querying Lilim...".yellow());
            
            let client = Client::new();
            let session_id = get_session_id()?;
            
            let payload = LilimQuery {
                text: query_text.clone(),
                session_id,
                tools_enabled: true,
                yolo_mode: false,
            };

            match client.post(API_URL).json(&payload).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<LilimResponse>() {
                            Ok(data) => {
                                println!("\n{}", "╔═══════════════════════════════════════╗".bright_red());
                                println!("{}", "║   Lilim's Response                    ║".bright_red());
                                println!("{}", "╚═══════════════════════════════════════╝".bright_red());
                                println!();
                                println!("{}", data.response.bright_white());
                                println!();
                                println!("{} {} | {} {}", 
                                    "Source:".dimmed(),
                                    data.source.cyan(),
                                    "Domain:".dimmed(),
                                    data.domain.green()
                                );
                                println!();
                            }
                            Err(e) => {
                                eprintln!("{} {}", "Failed to parse response:".red(), e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        eprintln!("{} {}", "API error:".red(), response.status());
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("{}", "Connection Error:".red().bold());
                    eprintln!("  {}", e);
                    eprintln!();
                    eprintln!("{}", "Is the Lilim service running?".yellow());
                    eprintln!("  {}", "sudo systemctl start lilith-ai".cyan());
                    std::process::exit(1);
                }
            }
        }
        Commands::Search { term } => {
            let search_text = term.join(" ");
            println!("{} {}", "🔍 Searching for:".yellow(), search_text.bright_white());
            
            let client = Client::new();
            let session_id = get_session_id()?;
            
            let payload = LilimQuery {
                text: format!("search: {}", search_text),
                session_id,
                tools_enabled: false,
                yolo_mode: false,
            };

            match client.post(API_URL).json(&payload).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<LilimResponse>() {
                            Ok(data) => {
                                println!("\n{}", data.response.bright_white());
                            }
                            Err(e) => {
                                eprintln!("{} {}", "Failed to parse response:".red(), e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Connection error:".red(), e);
                    std::process::exit(1);
                }
            }
        }
        Commands::History => {
            println!("{}", "📜 Conversation History".yellow());
            println!();
            if let Ok(session) = fs::read_to_string(SESSION_FILE) {
                println!("{} {}", "Current session:".dimmed(), session.cyan());
            } else {
                println!("{}", "No active session".dimmed());
            }
            println!();
            println!("{}", "Note: Full history is stored in /var/lib/lilith/memory.db".dimmed());
        }
        Commands::Clear => {
            match fs::remove_file(SESSION_FILE) {
                Ok(_) => println!("{}", "✓ Session cleared".green()),
                Err(_) => println!("{}", "No active session to clear".yellow()),
            }
        }
        Commands::Status => {
            println!("{}", "🔥 Checking Lilim status...".yellow());
            
            let client = Client::new();
            match client.get("http://localhost:8080/health").send() {
                Ok(_) => {
                    println!("{}", "✓ Lilim server is running".green().bold());
                    println!("  {} http://localhost:8080", "URL:".dimmed());
                }
                Err(_) => {
                    println!("{}", "✗ Lilim server is not responding".red().bold());
                    println!();
                    println!("{}", "Start the service:".yellow());
                    println!("  {}", "sudo systemctl start lilith-ai".cyan());
                    println!();
                    println!("{}", "Check logs:".yellow());
                    println!("  {}", "sudo journalctl -u lilith-ai -f".cyan());
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
