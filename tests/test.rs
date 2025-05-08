const TEST_INVENTORY: &str = "./tests/hosts.toml";

#[cfg(test)]
mod tests {
    use super::*;
    use ansible::{Inventory, Load};
    use std::path::PathBuf;

    #[test]
    fn test_inventory_load() {
        let inventory = Inventory::load(PathBuf::from(TEST_INVENTORY));
        assert!(inventory.is_ok());
        println!("INVENTORY: {}", inventory.unwrap().data);
    }

    #[test]
    fn test_inventory_get_host() {
        let inventory = Inventory::load(PathBuf::from(TEST_INVENTORY)).unwrap();
        let host = inventory.get_host("nycpi001");
        assert!(host.is_ok());
        println!("HOST: {:?}", host);
        let host = host.unwrap();
        println!("HOST: {:?}, VARS: {:?}", host, host.data);
    }

    #[test]
    fn test_inventory_get_hosts() {
        let inventory = Inventory::load(PathBuf::from(TEST_INVENTORY)).unwrap();
        let hosts = inventory.get_hosts();
        assert!(hosts.is_ok());
        println!("HOSTS: {:?}", hosts);
        assert!(!hosts.unwrap().is_empty());
    }

    #[test]
    fn test_get_vars() {
        let inventory = Inventory::load(PathBuf::from(TEST_INVENTORY)).unwrap();
        let hosts = inventory.get_hosts().unwrap();
        for host in hosts {
            assert!(host.clone().get_vars().is_ok());
            println!("HOST: {}", host.name);
            println!("VARS: {}", host.get_vars().unwrap());
        }
    }
}
