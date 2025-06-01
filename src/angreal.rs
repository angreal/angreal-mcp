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

    let args = match format {
        "json" => vec!["--json".to_string()],
        "human" => vec![],
        _ => unreachable!("Format already validated"),
    };

    run_angreal_command("tree", &args).await
}

fn validate_format(format: &str) -> Result<()> {
    match format {
        "json" | "human" => Ok(()),
        _ => anyhow::bail!("Invalid format '{}'. Must be 'json' or 'human'", format),
    }
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
        }
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
        status_parts
            .push("✗ No .angreal/ directory found - this is not an angreal project".to_string());
    }

    // If both are available, check project initialization status
    if angreal_available && angreal_folder_exists {
        match Command::new("angreal").arg("tree").output() {
            Ok(output) if output.status.success() => {
                let tree_output = String::from_utf8_lossy(&output.stdout);
                if tree_output.trim().is_empty() || tree_output.contains("No commands") {
                    status_parts.push(
                        "⚠ Project appears to be initialized but has no commands defined"
                            .to_string(),
                    );
                    status_parts
                        .push("  You may need to add tasks in the .angreal/ directory".to_string());
                } else {
                    status_parts.push(
                        "✓ Project is properly initialized with available commands".to_string(),
                    );
                    status_parts
                        .push("  Use 'angreal_tree' tool to see available commands".to_string());
                }
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.contains("No angreal.toml") || stderr.contains("not an angreal project") {
                    status_parts.push(
                        "✗ Project folder exists but may not be properly initialized".to_string(),
                    );
                    status_parts
                        .push("  Try running 'angreal init' to initialize the project".to_string());
                } else {
                    status_parts.push(format!("⚠ Angreal tree command failed: {}", stderr.trim()));
                }
            }
            Err(e) => {
                status_parts.push(format!("⚠ Could not check project status: {}", e));
            }
        }
    } else if !angreal_available && angreal_folder_exists {
        status_parts.push("  Install angreal to work with this project".to_string());
    } else if angreal_available && !angreal_folder_exists {
        status_parts.push("  This directory is not an angreal project".to_string());
        status_parts
            .push("  To create an angreal project: angreal init <template-url>".to_string());
    }

    // Add working directory info
    if let Ok(current_dir) = std::env::current_dir() {
        status_parts.push(format!("\nCurrent directory: {}", current_dir.display()));
    }

    Ok(status_parts.join("\n"))
}

pub async fn run_angreal_command(command: &str, args: &[String]) -> Result<String> {
    // Enhanced validation: allow more complex command structures
    validate_angreal_command(command)?;

    // Parse command to handle potential subcommands
    let all_args = parse_command_and_args(command, args)?;

    // Log the full command for debugging
    eprintln!("Executing: angreal {}", all_args.join(" "));

    let output = Command::new("angreal")
        .args(&all_args)
        .output()
        .context("Failed to execute angreal command")?;

    // Handle both success and failure cases
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Combine stdout and stderr for complete output
        if stderr.trim().is_empty() {
            Ok(stdout.to_string())
        } else {
            Ok(format!("{}\n\nStderr:\n{}", stdout, stderr))
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Check for common error patterns
        if stderr.contains("command not found") || output.status.code() == Some(127) {
            return Err(AngrealError::NotInstalled.into());
        }

        if stderr.contains("No angreal.toml") || stderr.contains("not an angreal project") {
            return Err(AngrealError::NotInProject.into());
        }

        // Provide helpful error with both stdout and stderr
        let error_output = if stdout.trim().is_empty() {
            stderr.to_string()
        } else {
            format!("Output:\n{}\n\nError:\n{}", stdout, stderr)
        };

        Err(AngrealError::ExecutionFailed(error_output).into())
    }
}

fn validate_angreal_command(command: &str) -> Result<()> {
    // Allow more flexible command structures including subcommands
    let parts: Vec<&str> = command.split_whitespace().collect();

    for part in parts {
        // Allow alphanumeric, hyphens, underscores, and common patterns
        if !part.chars().all(|c| {
            c.is_alphanumeric()
            || c == '-'
            || c == '_'
            || c == '.'  // for version specifiers
            || c == '/' // for paths in template names
        }) {
            return Err(anyhow::anyhow!(
                "Invalid command component '{}': contains disallowed characters",
                part
            ));
        }

        // Prevent obvious injection attempts
        if part.contains("&&") || part.contains("||") || part.contains(";") || part.contains("|") {
            return Err(anyhow::anyhow!(
                "Command injection attempt detected in '{}'",
                part
            ));
        }
    }

    Ok(())
}

fn parse_command_and_args(command: &str, args: &[String]) -> Result<Vec<String>> {
    let mut all_args = Vec::new();

    // Split command by whitespace to handle subcommands
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    all_args.extend(command_parts.iter().map(|s| s.to_string()));

    // Add additional arguments
    all_args.extend(args.iter().cloned());

    // Basic validation on final argument list
    for arg in &all_args {
        if arg.len() > 1000 {
            // Prevent extremely long arguments
            return Err(anyhow::anyhow!("Argument too long: {}", arg.len()));
        }
    }

    Ok(all_args)
}
