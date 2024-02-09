//! The account module is responsible for managing the user's account data.
use directories::UserDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;

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

/// The set_tenancy function writes the DEFAULT tenancy data to the config file.
pub fn acc(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: &str) {
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