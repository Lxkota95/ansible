# ansible
Rust API for Ansible. Wraps Ansible's command-line tools, so it needs Ansible installed
(`ansible-inventory` on `PATH`).

## Progress Tracker
- [X] Ansible inventory command
- [X] API to get hosts from inventory
- [X] API to get vars for host
- [X] Add Ansible test data to repo for use in test suite
- [X] Typed errors, no panics (0.3.0)
- [X] Official Rust docs (doctests run with `cargo test`)
- [ ] Inventory groups
- [ ] Ansible adhoc command
- [ ] Ansible playbook command

## Usage
```rust
use ansible::Inventory;

fn main() -> ansible::Result<()> {
    let inventory = Inventory::load("/path/to/inventory")?;

    // Full `ansible-inventory --list` output (a serde_json::Value), including groups
    println!("{:#}", inventory.data());

    // One host (None if the inventory doesn't have it)
    if let Some(host) = inventory.host("<hostname>") {
        println!("{}", host.vars());
    }

    // Every host, sorted by name
    for host in inventory.hosts() {
        println!("{}: {}", host.name(), host.vars());
    }
    Ok(())
}
```

Errors are `ansible::Error` values you can match on: `InventoryNotFound`, `CommandNotFound`
(Ansible not installed), `CommandFailed` (e.g. an unparseable inventory, with Ansible's stderr), and
`InvalidOutput`. See the [docs](https://docs.rs/ansible) and [CHANGELOG](CHANGELOG.md).

## Tests
`cargo test` runs the unit, integration and doc tests (needs Ansible installed).

## Releasing
Bump `version` in `Cargo.toml` and add a `CHANGELOG.md` entry in the PR. Merging it to `main` runs
`.github/workflows/release.yml`, which publishes to crates.io (if that version isn't there yet) and
pushes the `vX.Y.Z` tag. Merges that don't change the version publish nothing.
