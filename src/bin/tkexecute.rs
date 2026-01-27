use anyhow::{Result, Context};
use colored::Colorize;

#[path = "../core.rs"] mod core;
#[path = "../config.rs"] mod config;

fn main() -> Result<()> {
    let settings = config::GlobalSettings::load();
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        let usage = if settings.use_color { "Usage: tkexecute <token_file>".bright_yellow().to_string() } else { "Usage: tkexecute <token_file>".to_string() };
        anyhow::bail!(usage);
    }

    let path = &args[1];
    let raw_encoded = std::fs::read_to_string(path).context("Failed to read token file")?;
    
    let dec = core::decrypt_and_process(&raw_encoded, &settings)?;
    let p: Vec<&str> = dec.splitn(7, '|').collect();

    if p.len() < 7 {
        let err_msg = if settings.use_color { "Error: Invalid token format.".on_red().white().to_string() } else { "Error: Invalid token format.".to_string() };
        anyhow::bail!(err_msg);
    }

    match p[0].to_lowercase().as_str() {
        "webtoken" => {
            let timeout = p[5].parse().unwrap_or(5000);
            core::handle_webtoken(p[1], p[2], p[3], p[4], timeout, &settings)?;
        }
        "codetoken" | "run" => {
            core::handle_codetoken(p[2], p[6], &settings)?;
        }
        "pointertoken" => {
            core::handle_pointertoken(p[2], &settings)?;
        }
        _ => {
            let err_tag = if settings.use_color { "[Error]".on_red().white().to_string() } else { "[Error]".to_string() };
            println!("{} Unknown type: {}", err_tag, p[0]);
        }
    }
    Ok(())
}
