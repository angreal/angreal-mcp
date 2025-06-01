use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug)]
pub enum AngrealError {
    NotInstalled,
    NotInProject,
    ExecutionFailed(String),
}

impl std::fmt::Display for AngrealError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AngrealError::NotInstalled => write!(
                f,
                "The 'angreal' command is not available. Please install angreal first."
            ),
            AngrealError::NotInProject => write!(
                f,
                "Not in an angreal project. Please run this command from within an angreal project directory."
            ),
            AngrealError::ExecutionFailed(msg) => write!(f, "Angreal execution failed: {}", msg),
        }
    }
}

impl std::error::Error for AngrealError {}

pub async fn get_angreal_tree(format: &str) -> Result<String> {
    validate_format(format)?;
    
    let output = match format {
        "json" => run_angreal_command(&["tree", "--json"]).await?,
        "human" => run_angreal_command(&["tree"]).await?,
        _ => unreachable!("Format already validated"),
    };
    
    Ok(output)
}

fn validate_format(format: &str) -> Result<()> {
    match format {
        "json" | "human" => Ok(()),
        _ => anyhow::bail!("Invalid format '{}'. Must be 'json' or 'human'", format),
    }
}

async fn run_angreal_command(args: &[&str]) -> Result<String> {
    let output = Command::new("angreal")
        .args(args)
        .output()
        .context("Failed to execute angreal command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if stderr.contains("command not found") || output.status.code() == Some(127) {
            return Err(AngrealError::NotInstalled.into());
        }
        
        if stderr.contains("No angreal.toml") || stderr.contains("not an angreal project") {
            return Err(AngrealError::NotInProject.into());
        }
        
        return Err(AngrealError::ExecutionFailed(stderr.to_string()).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn check_angreal_available() -> Result<bool> {
    match Command::new("angreal").arg("--version").output() {
        Ok(output) => Ok(output.status.success()),
        Err(_) => Ok(false),
    }
}

pub async fn check_angreal_project_status() -> Result<String> {
    let mut status_parts = Vec::new();
    
    // Check if angreal is installed
    let angreal_available = match Command::new("angreal").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            status_parts.push(format!("✓ Angreal is installed: {}", version));
            true
        },
        _ => {
            status_parts.push("✗ Angreal is not installed or not available in PATH".to_string());
            status_parts.push("  Install angreal first: pip install angreal".to_string());
            false
        }
    };
    
    // Check if current directory has .angreal folder
    let angreal_folder_exists = std::path::Path::new(".angreal").exists();
    if angreal_folder_exists {
        status_parts.push("✓ Found .angreal/ directory - this is an angreal project".to_string());
    } else {
        status_parts.push("✗ No .angreal/ directory found - this is not an angreal project".to_string());
    }
    
    // If both are available, check project initialization status
    if angreal_available && angreal_folder_exists {
        match Command::new("angreal").arg("tree").output() {
            Ok(output) if output.status.success() => {
                let tree_output = String::from_utf8_lossy(&output.stdout);
                if tree_output.trim().is_empty() || tree_output.contains("No commands") {
                    status_parts.push("⚠ Project appears to be initialized but has no commands defined".to_string());
                    status_parts.push("  You may need to add tasks in the .angreal/ directory".to_string());
                } else {
                    status_parts.push("✓ Project is properly initialized with available commands".to_string());
                    status_parts.push("  Use 'angreal_tree' tool to see available commands".to_string());
                }
            },
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.contains("No angreal.toml") || stderr.contains("not an angreal project") {
                    status_parts.push("✗ Project folder exists but may not be properly initialized".to_string());
                    status_parts.push("  Try running 'angreal init' to initialize the project".to_string());
                } else {
                    status_parts.push(format!("⚠ Angreal tree command failed: {}", stderr.trim()));
                }
            },
            Err(e) => {
                status_parts.push(format!("⚠ Could not check project status: {}", e));
            }
        }
    } else if !angreal_available && angreal_folder_exists {
        status_parts.push("  Install angreal to work with this project".to_string());
    } else if angreal_available && !angreal_folder_exists {
        status_parts.push("  This directory is not an angreal project".to_string());
        status_parts.push("  To create an angreal project: angreal init <template-url>".to_string());
    }
    
    // Add working directory info
    if let Ok(current_dir) = std::env::current_dir() {
        status_parts.push(format!("\nCurrent directory: {}", current_dir.display()));
    }
    
    Ok(status_parts.join("\n"))
}