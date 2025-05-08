# ansible
Rust API for Ansible

## Progress Tracker
- [X] Ansible inventory command
- [X] Load trait for inventory to json
- [X] API to get hosts from inventory
- [X] API to get vars for host
- [ ] Add Ansible test data to repo for use in test suite
- [ ] Ansible adhoc command
- [ ] Ansible playbook command

## Features
- Load Ansible inventory data into json object
- Load Ansible host variables into json object

## Tests
- Execute tests with `cargo test -- --nocapture` to see stdout