# ansible
Rust API for Ansible

## Progress Tracker
- [X] Ansible inventory command
- [X] Load trait for inventory to json
- [X] API to get hosts from inventory
- [X] API to get vars for host
- [X] Add Ansible test data to repo for use in test suite
- [ ] Official Rust docs
- [ ] Ansible adhoc command
- [ ] Ansible playbook command

## Features
- Load Ansible inventory data into serde json object
```rust
use ansible::{Inventory, Load};

let inventory = Inventory::load(PathBuf::from('/path/to/inventory'))?;
let data = inventory.data
```

- Get specific host from Ansible inventory
```rust
use ansible::{Inventory, Load};

let inventory = Inventory::load(PathBuf::from('/path/to/inventory'))?;
let host = inventory.get_host("<hostname>")?;
hostvars = host.get_vars()?;
```

- Get all hosts from Ansible inventory
```rust
use ansible::{Inventory, Load};

let inventory = Inventory::load(PathBuf::from('/path/to/inventory'))?;
let hosts = inventory.get_hosts()?;
for host in hosts {
    hostvars = host.get_vars()?;
}
```

## Tests
- Execute tests with `cargo test -- --nocapture` to see stdout