// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed error hierarchy for `spore-validate`.
//!
//! Every fallible operation returns a domain-specific variant rather than
//! stringly-typed messages. This enables structured output, machine-readable
//! diagnostics, and composable error propagation without `process::exit`.

use std::path::PathBuf;

/// Top-level error type encompassing all failure modes.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error on {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML edit parse error: {0}")]
    TomlEditParse(#[from] toml_edit::TomlError),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("config error: {0}")]
    Config(String),

    #[error("validation failed: {error_count} error(s), {warning_count} warning(s)")]
    ValidationFailed {
        error_count: usize,
        warning_count: usize,
    },

    #[error("git operation failed: {0}")]
    Git(String),
}

impl Error {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

/// Severity level for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

/// Validation diagnostic — either an error (must fix) or warning (advisory).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    severity: Severity,
    message: String,
}

impl Diagnostic {
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            message: msg.into(),
        }
    }

    pub fn warning(msg: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            message: msg.into(),
        }
    }

    pub const fn is_error(&self) -> bool {
        matches!(self.severity, Severity::Error)
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    /// Promote warning to error (for --strict mode).
    pub fn promote_to_error(&mut self) {
        if self.severity == Severity::Warning {
            self.severity = Severity::Error;
            self.message = format!("(strict) {}", self.message);
        }
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.severity {
            Severity::Error => write!(f, "ERROR: {}", self.message),
            Severity::Warning => write!(f, "WARN:  {}", self.message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_io_displays_path() {
        let err = Error::io(
            "/tmp/foo.toml",
            std::io::Error::new(std::io::ErrorKind::NotFound, "gone"),
        );
        let msg = err.to_string();
        assert!(msg.contains("/tmp/foo.toml"));
        assert!(msg.contains("gone"));
    }

    #[test]
    fn error_config_displays_message() {
        let err = Error::Config("bad value".into());
        assert_eq!(err.to_string(), "config error: bad value");
    }

    #[test]
    fn error_validation_failed_displays_counts() {
        let err = Error::ValidationFailed {
            error_count: 3,
            warning_count: 2,
        };
        assert!(err.to_string().contains("3 error(s)"));
        assert!(err.to_string().contains("2 warning(s)"));
    }

    #[test]
    fn error_git_displays_message() {
        let err = Error::Git("auth failed".into());
        assert!(err.to_string().contains("auth failed"));
    }

    #[test]
    fn diagnostic_error_is_error() {
        let d = Diagnostic::error("test");
        assert!(d.is_error());
        assert_eq!(d.message(), "test");
        assert!(d.to_string().contains("ERROR"));
    }

    #[test]
    fn diagnostic_warning_is_not_error() {
        let d = Diagnostic::warning("warn");
        assert!(!d.is_error());
        assert_eq!(d.message(), "warn");
        assert!(d.to_string().contains("WARN"));
    }

    #[test]
    fn promote_to_error_works() {
        let mut d = Diagnostic::warning("soft issue");
        assert!(!d.is_error());
        d.promote_to_error();
        assert!(d.is_error());
        assert!(d.message().contains("(strict)"));
    }
}
