use clap::{Parser, Subcommand};
use flate2::{write::GzEncoder, Compression};
use std::io::Write;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use anyhow::{Result, Context};
use colored::Colorize;

mod config;
mod core;

#[derive(Parser)]
#[command(name = "tokenizer", about = "Smart Token System")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "global.init")] GlobalInit,
    #[command(name = "config.init")] ConfigInit,
    #[command(name = "token.ize")] Tokenize,
    #[command(name = "token.read")] TokenRead { file: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let settings = config::GlobalSettings::load();

    match cli.command.ok_or(anyhow::anyhow!("Please provide a command."))? {
        Commands::GlobalInit => {
            let mut p = dirs::home_dir().context("Could not find home directory")?;
            p.push(".config/tetra");
            std::fs::create_dir_all(&p)?;
            p.push("settings.toml");
            let default_set = "use_color = true\nuse_compression = true\ncompression_level = 6\nuse_encryption = true";
            std::fs::write(&p, default_set)?;
            
            let msg = if settings.use_color { "SUCCESS".on_bright_green().black().bold().to_string() } else { "SUCCESS".to_string() };
            println!("{} Settings created at {:?}", msg, p);
        }
        Commands::ConfigInit => {
            config::LocalConfig::init()?;
            let msg = if settings.use_color { "SUCCESS".on_bright_blue().black().bold().to_string() } else { "SUCCESS".to_string() };
            println!("{} config.toml initialized.", msg);
        }
        Commands::Tokenize => {
            let cfg = config::LocalConfig::load()?;
            let raw = format!("{}|{}|{}|{}|{}|{}|{}", 
                cfg.token_type, cfg.method, cfg.target, cfg.headers_json, cfg.body_data, cfg.timeout_ms, cfg.payload);
            
            let mut data = raw.into_bytes();

            if settings.use_compression {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::new(settings.compression_level));
                encoder.write_all(&data)?;
                data = encoder.finish()?;
            }

            if settings.use_encryption {
                data = data.iter().map(|&b| b ^ 42).collect();
            }

            std::fs::write(&cfg.output_file, STANDARD.encode(data))?;
            
            let msg = if settings.use_color { "SUCCESS".on_green().black().bold().to_string() } else { "SUCCESS".to_string() };
            let file_disp = if settings.use_color { cfg.output_file.bright_white().underline().to_string() } else { cfg.output_file };
            println!("{} Token created: {}", msg, file_disp);
        }
        Commands::TokenRead { file } => {
            let data = std::fs::read_to_string(&file).context("Failed to read token file")?;
            let decrypted = core::decrypt_and_process(&data, &settings)?;
            
            let tag = if settings.use_color { "[Read]".on_yellow().black().bold().to_string() } else { "[Read]".to_string() };
            let file_disp = if settings.use_color { file.bright_white().bold().to_string() } else { file };
            let line = if settings.use_color { "--------------------------------------------------".bright_black().to_string() } else { "--------------------------------------------------".to_string() };

            println!("{} {}", tag, file_disp);
            println!("{}", line);
            println!("{}", decrypted);
            println!("{}", line);
        }
    }
    Ok(())
}
