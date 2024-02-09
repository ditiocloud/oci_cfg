//! The account module captures the basic account data, it writes the user and tenancy information to the config file.
//! #Example
//! ```rust
//! use oci_config::account::{tenancy, admin};
//! 
//! tenancy(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
//!     "IAD"
//! );
//! admin(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "passphrase"
//! );
//! ```
use directories::UserDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;

/// Represents the DEFAULT section of the config file.
#[derive(Debug)]
pub struct Tenancy {
    user: String,
    fingerprint: String,
    key_file: String,
    tenancy: String,
    region: String, // selection of active regions
}

impl Tenancy {
    fn new(
        user: String,
        fingerprint: String,
        key_file: String,
        tenancy: String,
        region: String,
    ) -> Tenancy {
        Self {
            user,
            fingerprint,
            key_file,
            tenancy,
            region,
        }
    }
}

/// The set_tenancy function writes the DEFAULT tenancy data to the config file.
pub fn tenancy(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: &str) {
    // write to file
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let config_file = config_path.to_str().expect("Failed to convert path to str");

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

/// Represents the ADMIN_USER section of the config file.
#[derive(Debug)]
pub struct Admin {
    user: String,
    fingerprint: String,
    key_file: String,
    pass_phrase: String,
}

impl Admin {
    fn new(user: String, fingerprint: String, key_file: String, pass_phrase: String) -> Usr {
        Self {
            user,
            fingerprint,
            key_file,
            pass_phrase,
        }
    }
}

/// The add_user function writes the ADMIN_USER data to the config file.
pub fn user(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    // write to config file
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let config_file = config_path.to_str().expect("Failed to convert path to str");
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