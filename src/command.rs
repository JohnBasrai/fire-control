use anyhow::{ensure, Context, Result};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum FireCommand {
    Fire(f64),
    Cancel,
}

impl FromStr for FireCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let trimmed = s.trim();

        if trimmed == "-1" {
            return Ok(FireCommand::Cancel);
        }

        let secs = trimmed
            .parse::<f64>()
            .with_context(|| format!("Failed to parse input as float: {:?}", trimmed))?;

        ensure!(secs > 0.0, "Delay must be positive or -1 to cancel");
        Ok(FireCommand::Fire(secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{ensure, Result};
    use std::str::FromStr;

    #[test]
    fn parses_valid_fire_commands() -> Result<()> {
        ensure!(FireCommand::from_str("10")? == FireCommand::Fire(10.0));
        ensure!(FireCommand::from_str("3.14")? == FireCommand::Fire(3.14));
        ensure!(FireCommand::from_str("  2.5 ")? == FireCommand::Fire(2.5));
        Ok(())
    }

    #[test]
    fn parses_cancel_command() -> Result<()> {
        ensure!(FireCommand::from_str("-1")? == FireCommand::Cancel);
        ensure!(FireCommand::from_str("  -1  ")? == FireCommand::Cancel);
        Ok(())
    }

    #[test]
    fn rejects_non_positive_numbers() -> Result<()> {
        assert!(FireCommand::from_str("0").is_err());
        assert!(FireCommand::from_str("-2").is_err());
        assert!(FireCommand::from_str("-100").is_err());
        Ok(())
    }

    #[test]
    fn rejects_invalid_input() -> Result<()> {
        assert!(FireCommand::from_str("hello").is_err());
        assert!(FireCommand::from_str(" ").is_err());
        assert!(FireCommand::from_str("").is_err());
        assert!(FireCommand::from_str("15s").is_err());
        Ok(())
    }
}
