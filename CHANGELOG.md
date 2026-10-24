# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-10-24
Hardening release. **Breaking** API changes; see Migration below.

### Changed
- Errors are a typed `ansible::Error` enum (via `thiserror`) instead of `anyhow::Error`, with
  `ansible::Result<T>` as shorthand. Variants: `InventoryNotFound`, `CommandNotFound`,
  `CommandFailed` (with Ansible's stderr), `InvalidOutput`, `Io`. The enum is `#[non_exhaustive]`.
- `Inventory::load` takes `impl AsRef<Path>` (e.g. `&str`, `Path`, `PathBuf`) and no longer panics:
  a missing `ansible-inventory`, bad output or a non-UTF-8 path are errors (non-UTF-8 paths now work).
- An inventory Ansible can't parse is now an error (`CommandFailed`) instead of an empty inventory.
- Methods borrow instead of consuming the inventory, so they can be called repeatedly.
- Inventory and host fields are private; use the accessors.

### Removed
- The `Load` trait: call `Inventory::load` directly.
- The `anyhow` and `serde` dependencies.

### Added
- Crate documentation with runnable examples; error-path tests; CI; `rust-version = "1.85"`.
- Release workflow: merging a version bump to `main` publishes to crates.io (trusted publishing)
  and tags `vX.Y.Z`.

### Migration
| 0.2 | 0.3 |
|---|---|
| `use ansible::{Inventory, Load};` | `use ansible::Inventory;` |
| `Inventory::load(PathBuf::from(p))?` | `Inventory::load(p)?` |
| `inventory.data` | `inventory.data()` |
| `inventory.get_host(name)?` (`Ok` with `null` vars if unknown) | `inventory.host(name)` → `Option<Host>` |
| `inventory.get_hosts()?` → `Vec<Host>` | `inventory.hosts()` → iterator of `Host` |
| `host.name`, `host.get_vars()?` | `host.name()`, `host.vars()` |

## [0.2.0] - 2025.05.07
- Structs for Ansible `Inventory` and `Host` objects
- `Load` trait for `Inventory` struct
- `get_host()` method for `Inventory` struct
- `get_hosts()` method for `Inventory` struct
- `get_vars()` method for `Host` struct
- Basic test coverage

## [0.1.0] - 2025.05.07
- Initial commit containing boilerplate Rust library code