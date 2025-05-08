use anyhow::{Result, anyhow};
use serde_json::Value;
use std::{path::PathBuf, process::Command};

const ANSIBLE_INVENTORY_COMMAND: &str = "ansible-inventory";

pub struct Inventory {
    pub path: PathBuf,
    pub data: Value,
}

#[derive(Debug, Clone)]
pub struct Host {
    pub name: String,
    pub data: Value,
}

pub trait Load
where
    Self: std::marker::Sized,
{
    fn load(source: PathBuf) -> Result<Self>;
}

impl Load for Inventory {
    fn load(source: PathBuf) -> Result<Inventory> {
        assert!(
            Command::new("which")
                .arg(ANSIBLE_INVENTORY_COMMAND)
                .output()
                .unwrap()
                .status
                .success()
        );
        if !source.exists() {
            return Err(anyhow!("Invalid path to Ansible inventory: {:?}", source));
        }
        let command = Command::new(ANSIBLE_INVENTORY_COMMAND)
            .args([
                "--inventory",
                source
                    .to_str()
                    .expect("Inventory source is not valid unicode"),
                "--list",
            ])
            .output()?;
        if command.status.success() {
            let raw_data = String::from_utf8_lossy(&command.stdout);
            let data: Value = serde_json::from_str(&raw_data).unwrap();
            Ok(Inventory { path: source, data })
        } else {
            Err(anyhow!(
                "Ansible command failed: stdin: {:?}\nstderr: {:?}",
                command.stdout,
                command.stderr
            ))
        }
    }
}

impl Inventory {
    pub fn get_hosts(self) -> Result<Vec<Host>> {
        let mut hosts = Vec::new();
        for (hostname, hostvars) in self.data["_meta"]["hostvars"].as_object().unwrap() {
            let host = Host {
                name: hostname.to_string(),
                data: hostvars.to_owned(),
            };
            hosts.push(host);
        }
        Ok(hosts)
    }
}

impl Host {
    pub fn get_vars(self) -> Result<Value> {
        Ok(self.data)
    }
}
