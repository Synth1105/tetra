use anyhow::{Result, anyhow, Context};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use flate2::read::GzDecoder;
use std::io::Read;
use colored::Colorize;
use crate::config::GlobalSettings;
use std::time::Duration;

const MASTER_KEY: u8 = 42;

pub fn decrypt_and_process(data: &str, settings: &GlobalSettings) -> Result<String> {
    let mut current = STANDARD.decode(data.trim()).context("Base64 Decode Error")?;

    // 글로벌 설정에 따른 암호화 해제 여부
    if settings.use_encryption {
        current = current.iter().map(|&b| b ^ MASTER_KEY).collect();
    }

    // 글로벌 설정에 따른 압축 해제 여부
    if settings.use_compression {
        let mut decoder = GzDecoder::new(&current[..]);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed).context("Decompression Failed")?;
        Ok(decompressed)
    } else {
        Ok(String::from_utf8(current).context("UTF8 Conversion Error")?)
    }
}

pub fn handle_webtoken(method: &str, url: &str, h_json: &str, body: &str, timeout: u64, settings: &GlobalSettings) -> Result<()> {
    let tag = if settings.use_color { "[WebToken]".on_cyan().black().bold().to_string() } else { "[WebToken]".to_string() };
    let m_text = if settings.use_color { method.bright_magenta().bold().to_string() } else { method.to_string() };
    let u_text = if settings.use_color { url.cyan().underline().to_string() } else { url.to_string() };

    println!("{} {} -> {}", tag, m_text, u_text);

    let agent = ureq::AgentBuilder::new().timeout(Duration::from_millis(timeout)).build();
    let mut req = agent.request(method.to_uppercase().as_str(), url);

    if let Ok(headers) = serde_json::from_str::<serde_json::Value>(h_json) {
        if let Some(obj) = headers.as_object() {
            for (k, v) in obj {
                req = req.set(k, v.as_str().unwrap_or(""));
            }
        }
    }

    let resp = if !body.is_empty() && (method == "POST" || method == "PUT" || method == "PATCH") {
        req.send_string(body)
    } else {
        req.call()
    }.map_err(|e| anyhow!("HTTP Error: {}", e))?;

    let res_body = resp.into_string()?;
    let separator = if settings.use_color { "--- RESPONSE CONTENT ---".bright_black().to_string() } else { "--- RESPONSE CONTENT ---".to_string() };
    println!("{}", separator);
    
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&res_body) {
        let pretty = serde_json::to_string_pretty(&json)?;
        if settings.use_color {
            println!("{}", pretty.yellow());
        } else {
            println!("{}", pretty);
        }
    } else {
        println!("{}", res_body);
    }
    Ok(())
}

pub fn handle_codetoken(path: &str, code: &str, settings: &GlobalSettings) -> Result<()> {
    let tag = if settings.use_color { "[CodeToken]".on_green().black().bold().to_string() } else { "[CodeToken]".to_string() };
    let path_display = if settings.use_color { path.bright_white().bold().to_string() } else { path.to_string() };
    println!("{} Executing at: {}", tag, path_display);
    
    let status = std::process::Command::new(path)
        .arg("-c")
        .arg(code)
        .status()
        .context("Failed to execute process")?;

    if !status.success() {
        anyhow::bail!("Process exited with error status");
    }
    Ok(())
}

pub fn handle_pointertoken(path: &str, settings: &GlobalSettings) -> Result<()> {
    let tag = if settings.use_color { "[PointerToken]".on_magenta().black().bold().to_string() } else { "[PointerToken]".to_string() };
    let path_display = if settings.use_color { path.bright_white().bold().to_string() } else { path.to_string() };
    println!("{} Reading: {}", tag, path_display);
    
    std::process::Command::new("cat")
        .arg(path)
        .status()
        .context("Failed to run cat command")?;
    Ok(())
}
