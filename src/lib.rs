//! This is a small library that writes and reads to an Oracle Cloud Infrastructure (OCI) config file in the user's home directory.
//! The library checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory.
//! It also checks the permissions before adding content.
//! 
//! More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>
//! # Example
//! ```rust
//! fn main() {
//! file()
//! acc(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "xxx",
//!     "path/to/key_file",
//!     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
//!     "IAD",
//! );
//! usr(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "xxx",
//!     "path/to/key_file",
//!     "xxx",
//! );
//! read();
//! }
//! ```

use directories::UserDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub mod file;
pub mod region;
pub mod config;

use file::permissions;
use file::create;
use file::read;
use region::Regions;
use region::home;

/// Represents the DEFAULT section of the config file.
#[derive(Debug)]
pub struct Acc {
    user: String,
    fingerprint: String,
    key_file: String,
    tenancy: String,
    region: String, // selection of active regions
}

impl Acc {
    fn new(
        user: String,
        fingerprint: String,
        key_file: String,
        tenancy: String,
        region: String,
    ) -> Acc {
        Self {
            user,
            fingerprint,
            key_file,
            tenancy,
            region,
        }
    }
}

/// Represents the ADMIN_USER section of the config file.
#[derive(Debug)]
pub struct Usr {
    user: String,
    fingerprint: String,
    key_file: String,
    pass_phrase: String,
}

impl Usr {
    fn new(user: String, fingerprint: String, key_file: String, pass_phrase: String) -> Usr {
        Self {
            user,
            fingerprint,
            key_file,
            pass_phrase,
        }
    }
}

/// The set_tenancy function writes the DEFAULT tenancy data to the config file.
pub fn acc(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: &str) {
    // write to file
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let config_file = config_path.to_str().expect("Failed to convert path to str");
    permissions(config_file);

    let config = OpenOptions::new()
        .write(true)
        .append(true)
        .open(config_file);
    match config {
        Ok(mut config) => {
            match config.write_all(
                format!(
                    "[DEFAULT]\nuser={}\nfingerprint={}\nkey_file={}\ntenancy={}\nregion={}\n\n",
                    user, fingerprint, key_file, tenancy, home(region)
                )
                .as_bytes(),
            ) {
                Ok(_) => println!("Tenancy data written to file successfully"),
                Err(e) => println!("Failed to write tenancy data to file: {}", e),
            }
        }
        Err(e) => println!("Failed to create file: {}", e),
    }
}

/// The add_user function writes the ADMIN_USER data to the config file.
pub fn usr(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    // write to config file
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let config_file = config_path.to_str().expect("Failed to convert path to str");
    permissions(config_file);

    let config = OpenOptions::new()
        .write(true)
        .append(true)
        .open(config_file);
    match config {
        Ok(mut config) => {
            match config.write_all(
                format!(
                    "[ADMIN_USER]\nuser={}\nfingerprint={}\nkey_file={}\npass_phrase={}\n\n",
                    user, fingerprint, key_file, pass_phrase
                )
                .as_bytes(),
            ) {
                Ok(_) => println!("User data written to file successfully"),
                Err(e) => println!("Failed to write user data to file: {}", e),
            }
        }
        Err(e) => println!("Failed to create file: {}", e),
    }
}