//! Rust API for [Ansible](https://docs.ansible.com/).
//!
//! Wraps Ansible's command-line tools; it needs Ansible installed (`ansible-inventory` on `PATH`).
//!
//! # Inventory
//!
//! [`Inventory::load`] reads any inventory Ansible understands and gives typed access to its hosts
//! and their variables (merged from the inventory, `host_vars/` and `group_vars/`):
//!
//! ```
//! use ansible::Inventory;
//!
//! # fn main() -> ansible::Result<()> {
//! let inventory = Inventory::load("tests/hosts.toml")?;
//!
//! // One host; `None` if the inventory doesn't have it
//! let host = inventory.host("nycpi001").expect("host is in the test inventory");
//! assert_eq!(host.vars()["foo"], "bar");
//!
//! // Every host, sorted by name
//! for host in inventory.hosts() {
//!     println!("{}: {}", host.name(), host.vars());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Failures are [`Error`] values you can match on, for example a missing inventory:
//!
//! ```
//! use ansible::{Error, Inventory};
//!
//! match Inventory::load("does/not/exist") {
//!     Err(Error::InventoryNotFound(path)) => println!("no inventory at {}", path.display()),
//!     other => panic!("unexpected: {other:?}"),
//! }
//! ```

mod error;

pub use error::{Error, Result};
use serde_json::{Map, Value};
use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
};

const ANSIBLE_INVENTORY_COMMAND: &str = "ansible-inventory";

/// An Ansible inventory, loaded with `ansible-inventory --list`.
#[derive(Debug, Clone)]
pub struct Inventory {
    path: PathBuf,
    data: Value,
}

/// A host in an [`Inventory`], borrowed from it: `name` and `vars` point into the inventory's data,
/// so looking hosts up copies nothing.
#[derive(Debug, Clone, Copy)]
pub struct Host<'a> {
    name: &'a str,
    vars: &'a Value,
}

impl Inventory {
    /// Loads an inventory by running `ansible-inventory --inventory <path> --list`.
    ///
    /// Any inventory format Ansible understands works (INI, YAML, TOML, a directory, a script).
    /// An inventory Ansible cannot parse is an error, not an empty inventory.
    ///
    /// # Errors
    ///
    /// - [`Error::InventoryNotFound`] if `path` does not exist
    /// - [`Error::CommandNotFound`] if `ansible-inventory` is not on `PATH`
    /// - [`Error::CommandFailed`] if `ansible-inventory` fails, e.g. on an unparseable inventory
    /// - [`Error::InvalidOutput`] if its output is not the expected JSON
    /// - [`Error::Io`] if the command could not be started for another reason
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        Self::load_with(ANSIBLE_INVENTORY_COMMAND, path.as_ref())
    }

    /// [`Inventory::load`] with the command name as a parameter, so tests can use a missing one.
    fn load_with(command: &'static str, path: &Path) -> Result<Self> {
        // ansible-inventory exits 0 with an empty inventory for a missing path, so check first
        if !path.exists() {
            return Err(Error::InventoryNotFound(path.to_path_buf()));
        }

        let output = Command::new(command)
            .arg("--inventory")
            .arg(path)
            .arg("--list")
            // Without this, an unparseable inventory is only a warning and the output is empty
            .env("ANSIBLE_INVENTORY_ANY_UNPARSED_IS_FAILED", "true")
            .output()
            .map_err(|source| match source.kind() {
                io::ErrorKind::NotFound => Error::CommandNotFound { command },
                _ => Error::Io { command, source },
            })?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command,
                status: output.status,
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            });
        }

        let data: Value =
            serde_json::from_slice(&output.stdout).map_err(|e| Error::InvalidOutput {
                command,
                reason: e.to_string(),
            })?;
        if hostvars(&data).is_none() {
            return Err(Error::InvalidOutput {
                command,
                reason: "no `_meta.hostvars` object".to_owned(),
            });
        }

        Ok(Self {
            path: path.to_path_buf(),
            data,
        })
    }

    /// The path the inventory was loaded from.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The full `ansible-inventory --list` output, including groups.
    #[must_use]
    pub fn data(&self) -> &Value {
        &self.data
    }

    /// Looks up a host by name; `None` if the inventory has no such host.
    #[must_use]
    pub fn host(&self, name: &str) -> Option<Host<'_>> {
        let (name, vars) = hostvars(&self.data)?.get_key_value(name)?;
        Some(Host { name, vars })
    }

    /// All hosts in the inventory, sorted by name.
    pub fn hosts(&self) -> impl Iterator<Item = Host<'_>> {
        hostvars(&self.data)
            .into_iter()
            .flatten()
            .map(|(name, vars)| Host { name, vars })
    }
}

impl<'a> Host<'a> {
    /// The host's inventory name.
    #[must_use]
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// The host's variables (from the inventory, `host_vars/` and `group_vars/`), as a JSON object.
    #[must_use]
    pub fn vars(&self) -> &'a Value {
        self.vars
    }
}

/// The `_meta.hostvars` object of `ansible-inventory --list` output.
fn hostvars(data: &Value) -> Option<&Map<String, Value>> {
    data.pointer("/_meta/hostvars")?.as_object()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_command_is_command_not_found() {
        let err =
            Inventory::load_with("ansible-inventory-does-not-exist", Path::new(".")).unwrap_err();
        assert!(matches!(err, Error::CommandNotFound { .. }), "{err:?}");
    }
}
