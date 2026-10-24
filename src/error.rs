use std::{io, path::PathBuf, process::ExitStatus};

/// Errors returned by this crate.
///
/// Marked `#[non_exhaustive]` so new variants can be added (for example when ad-hoc and playbook
/// commands arrive) without a breaking release; callers matching on it need a `_ =>` arm.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An Ansible command (such as `ansible-inventory`) is not installed or not on `PATH`.
    #[error("`{command}` not found on PATH (is Ansible installed?)")]
    CommandNotFound { command: &'static str },

    /// The inventory path does not exist.
    #[error("Ansible inventory not found: {}", .0.display())]
    InventoryNotFound(PathBuf),

    /// An Ansible command could not be started for a reason other than not being installed.
    #[error("failed to run `{command}`")]
    Io {
        command: &'static str,
        #[source]
        source: io::Error,
    },

    /// An Ansible command ran but exited unsuccessfully; `stderr` holds its error output.
    #[error("`{command}` failed ({status}): {stderr}")]
    CommandFailed {
        command: &'static str,
        status: ExitStatus,
        stderr: String,
    },

    /// An Ansible command's output was not the JSON this crate expects.
    #[error("unexpected output from `{command}`: {reason}")]
    InvalidOutput {
        command: &'static str,
        reason: String,
    },
}

/// Shorthand for `Result<T, ansible::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
