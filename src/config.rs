/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::Deserialize;
use toml;

pub struct Config {
    pub data: ConfigToml,
    pub raw_toml: String
}
#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    pub destination_root: Option<String>,
    pub status_file: Option<String>,
    pub mirror: Vec<Mirror>,
}

#[derive(Debug, Deserialize)]
pub struct Mirror {
    pub source: String,
    pub destination: String
}

impl Config {
    pub fn new(unparsed_toml: &String) -> Self {
        // TODO: convert into pattern matching for error handling
        let parsed_toml = toml::from_str(unparsed_toml).unwrap();
        Self {
            data: parsed_toml,
            raw_toml: unparsed_toml.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use super::*;

    macro_rules! testdata {($fname:expr) => (
        concat!(env!("CARGO_MANIFEST_DIR"), "/testdata/", $fname)
      )}

    #[test]
    fn complete_config() {
        let s = read_to_string(testdata!("config/complete_config.toml")).unwrap();
        let c = Config::new(&s);
        assert_eq!(c.data.destination_root.unwrap(), "/tmp/mirrors/data");
        assert_eq!(c.data.status_file.unwrap(), "/tmp/mirrors/status");
        assert_eq!(c.data.mirror[0].source, "rsync://rsync.example.com/mirror1");
        assert_eq!(c.data.mirror[0].destination, "mirror1");
        assert_eq!(c.data.mirror[1].source, "rsync://rsync.example.com/mirror2");
        assert_eq!(c.data.mirror[1].destination, "/tmp/outside/mirror2");
    }
}
