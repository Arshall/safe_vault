// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use config_file_handler::{self, FileHandler};
use error::InternalError;
use xor_name::XorName;

#[derive(PartialEq, Eq, Debug, RustcDecodable, RustcEncodable, Clone)]
pub struct Config {
    pub account_size: Option<u64>,  // measured by unit
    pub wallet_address: Option<XorName>,
    pub entries_limit: Option<u64>,
    pub chunk_store_limit: Option<u64>,  // measured by MB
}

impl Default for Config {
    fn default() -> Config {
        Config {
            account_size: None,
            wallet_address: None,
            entries_limit: None,
            chunk_store_limit: None,
        }
    }
}

/// Reads the default vault config file.
pub fn read_config_file() -> Result<Config, InternalError> {
    // if the config file is not present, a default one will be generated
    let file_handler = try!(FileHandler::new(&try!(get_file_name())));
    let cfg = try!(file_handler.read_file());
    Ok(cfg)
}

/// Writes a Vault config file **for use by tests and examples**.
///
/// The file is written to the [`current_bin_dir()`](file_handler/fn.current_bin_dir.html)
/// with the appropriate file name.
///
/// N.B. This method should only be used as a utility for test and examples.  In normal use cases,
/// the config file should be created by the installer for the dependent application.
#[allow(dead_code)]
pub fn write_config_file(config: Config)
                         -> Result<::std::path::PathBuf, InternalError> {
    use std::io::Write;
    let mut config_path = try!(config_file_handler::current_bin_dir());
    config_path.push(try!(get_file_name()));
    let mut file = try!(::std::fs::File::create(&config_path));
    try!(write!(&mut file,
                "{}",
                ::rustc_serialize::json::as_pretty_json(&config)));
    try!(file.sync_all());
    Ok(config_path)
}

fn get_file_name() -> Result<::std::ffi::OsString, InternalError> {
    let mut name = try!(config_file_handler::exe_file_stem());
    name.push(".vault.config");
    Ok(name)
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_sample_config_file() {
        use std::path::Path;
        use std::io::Read;
        use super::Config;
        use rustc_serialize::json;

        let path = Path::new("installer/sample.config").to_path_buf();

        let mut file = match ::std::fs::File::open(path) {
            Ok(file) => file,
            Err(what) => {
                panic!(format!("Error opening sample.config: {:?}", what));
            }
        };

        let mut encoded_contents = String::new();

        if let Err(what) = file.read_to_string(&mut encoded_contents) {
            panic!(format!("Error reading sample.config: {:?}", what));
        }

        if let Err(what) = json::decode::<Config>(&encoded_contents) {
            panic!(format!("Error parsing sample.config: {:?}", what));
        }
    }
}
