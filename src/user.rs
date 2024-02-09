//! The user module contains the user configuration options for the config writer.
use directories::UserDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;

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

/// The add_user function writes the ADMIN_USER data to the config file.
pub fn usr(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
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