mod api;
mod auth;
mod cli;
mod config;

use clap::Parser;
use std::io::{self, Read};

use crate::api::NoteApi;
use crate::auth::authenticate;
use crate::cli::{Args, Operation};
use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut config = Config::load()?;

    match args.operation {
        Operation::SetToken => {
            if let Some(token) = args.token {
                config.token = token;
                config.save()?;
                println!("Token saved successfully!");
                return Ok(());
            } else {
                return Err("Token is required for set-token operation".into());
            }
        }
        Operation::Login => {
            authenticate(&mut config, args.username, args.password).await?;
            println!("Authentication successful!");
            return Ok(());
        }
        _ => {
            let api = NoteApi::new(config);
            let mut content = String::new();
            io::stdin().read_to_string(&mut content)?;
            let content = content.trim();

            match args.operation {
                Operation::Create => {
                    api.create_note(content).await?;
                }
                Operation::Update => {
                    let id = args.id.ok_or("ID is required for update operation")?;
                    api.update_note(&id, content).await?;
                }
                Operation::Delete => {
                    let id = args.id.ok_or("ID is required for delete operation")?;
                    api.delete_note(&id).await?;
                }
                Operation::SetToken | Operation::Login => unreachable!(),
            }
        }
    }

    Ok(())
} 