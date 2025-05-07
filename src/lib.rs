use std::{path::PathBuf, process::Command};
use serde_json::Value;
use anyhow::{anyhow, Result};

struct Inventory {
    path: PathBuf,
    data: Value,
}

struct Host {
    name: String,
    data: Value,
}

impl Inventory {
    fn load(self) -> Result<Value> {
        // configure the command
        // execute the command
        // save output to Ok(json)
        // return
        todo!();
        return Err(anyhow!("-1"))
    }
}

impl Inventory {
    fn get_hosts(self) -> Result<Vec<Host>> {
        todo!();
        return Err(anyhow!("-1"))
    }
}

impl Host {
    fn get_vars(self) -> Result<Value> {
        todo!();
        return Err(anyhow!("-1"))
    }
}