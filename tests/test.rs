use ansible::Inventory;
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
