use ansible::{Error, Inventory};
use serde_json::json;

const TEST_INVENTORY: &str = "./tests/hosts.toml";

#[test]
fn inventory_load() {
    let inventory = Inventory::load(TEST_INVENTORY).unwrap();
    assert!(inventory.data()["_meta"]["hostvars"].is_object());
}

#[test]
fn inventory_host() {
    let inventory = Inventory::load(TEST_INVENTORY).unwrap();
    let host = inventory.host("nycpi001").unwrap();
    assert_eq!(host.name(), "nycpi001");
}

#[test]
fn inventory_hosts() {
    let inventory = Inventory::load(TEST_INVENTORY).unwrap();
    let names: Vec<_> = inventory.hosts().map(|host| host.name()).collect();
    assert_eq!(names, ["nycpi001"]);
}

#[test]
fn host_vars() {
    let inventory = Inventory::load(TEST_INVENTORY).unwrap();
    // From tests/host_vars/nycpi001.yaml
    assert_eq!(
        inventory.host("nycpi001").unwrap().vars(),
        &json!({ "foo": "bar" })
    );
}

#[test]
fn unknown_host_is_none() {
    let inventory = Inventory::load(TEST_INVENTORY).unwrap();
    assert!(inventory.host("no-such-host").is_none());
}

#[test]
fn missing_inventory_is_inventory_not_found() {
    let err = Inventory::load("./tests/does-not-exist.toml").unwrap_err();
    assert!(matches!(err, Error::InventoryNotFound(_)), "{err:?}");
}

#[test]
fn unparseable_inventory_is_command_failed() {
    let err = Inventory::load("./tests/invalid.toml").unwrap_err();
    let Error::CommandFailed { stderr, .. } = &err else {
        panic!("expected CommandFailed, got {err:?}");
    };
    assert!(
        stderr.contains("parse"),
        "stderr should explain the failure: {stderr}"
    );
}
