//! This is a small library to manage an Oracle Cloud Infrastructure (OCI) config file. 
//! The library checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory.
//! It also checks the permissions before adding content.
//! 
//! More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>
//! # Example
//! ```rust
//! use oci_cfg::{profile, credentials, report};
//! 
//! fn main() {
//!    profile(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
//!     "IAD"
//!    );
//!    credentials(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "passphrase"
//!    );
//!    report();
//! }
//! ```
pub mod file;
pub mod region;
pub mod log;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use directories::UserDirs;
use file::{create, permissions, read};
use region::identifier;

static DIR: &str = ".oci";
static NAME: &str = "config";

/// represents a tenancy profile with the user's OCID, fingerprint, path to the private key, OCID of the tenancy and the region.
#[derive(Debug)]
pub struct Profile {
    user: String,
    fingerprint: String,
    key_file: String,
    tenancy: String,
    region: String, // selection of active regions
}

impl Profile {
    // Function to format the Profile struct as a string
    fn profile_entry(&self) -> String {
        format!("[DEFAULT]\nuser: {}\nfingerprint: {}\nkey_file: {}\ntenancy: {}\nregion: {}\n\n", 
        self.user, self.fingerprint, self.key_file, self.tenancy, self.region)
    }
    
    // Function to write the struct to the config file
    fn write_to_config(&self, path: &str) -> io::Result<()> {
        // define directory directory
        let config_path = UserDirs::new().unwrap().home_dir().join(path);
        let path_to_str = config_path.to_str().expect("Failed to convert path to str");

        // set modification properties
        let config = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path_to_str);
        match config {
            Ok(mut config) => {
                match config.write_all(
                    self.profile_entry().as_bytes(),
                ) {
                    Ok(_) => println!("Tenancy data written to file successfully"),
                    Err(e) => println!("Failed to write tenancy data to file: {}", e),
                }
            }
            Err(e) => println!("Failed to create file: {}", e),
        }
    
        Ok(())
    }
}

/// writes an account profile to the config file, the values are used as defaults for admin users.
/// # Example
/// ```rust
/// use oci_cfg::profile;
/// 
/// fn main() {
///    profile(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
///     "IAD"
///    );
/// }
/// ```
pub fn profile(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, home: &str) {
    let default_profile = Profile {
        user: String::from(user),
        fingerprint: String::from(fingerprint),
        key_file: String::from(key_file),
        tenancy: String::from(tenancy),
        region: identifier(home)
    };
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    if !path.exists() {
        create(DIR, NAME);
        // Call the write_to_config method to write the struct to the file
        if let Err(err) = default_profile.write_to_config(path.to_str().unwrap()) {
            eprintln!("Error writing to file: {}", err);
        } else {
            println!("Profile successfully written to {}", path.to_str().unwrap());
        }
    } else {
        permissions(path.to_str().unwrap());
        // Call the write_to_config method to write the struct to the file
        if let Err(err) = default_profile.write_to_config(path.to_str().unwrap()) {
            eprintln!("Error writing to file: {}", err);
        } else {
            println!("Profile successfully written to {}", path.to_str().unwrap());
        }
    }
}

/// represents an admin profile with the user's OCID, fingerprint, path to the private key and the passphrase.
#[derive(Debug)]
pub struct Credentials {
    user: String,
    fingerprint: String,
    key_file: String,
    pass_phrase: String
}

impl Credentials {
    // Function to format the Profile struct as a string
    fn admin_entry(&self) -> String {
        format!("[ADMIN_USER]\nuser: {}\nfingerprint: {}\nkey_file: {}\npass_phrase: {}\n\n", 
        self.user, self.fingerprint, self.key_file, self.pass_phrase)
    }
    
    // Function to write the struct to the config file
    fn write_to_config(&self, path: &str) -> io::Result<()> {
        // define directory directory
        let config_path = UserDirs::new().unwrap().home_dir().join(path);
        let path_to_str = config_path.to_str().expect("Failed to convert path to str");

        // set modification properties
        let config = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path_to_str);
        match config {
            Ok(mut config) => {
                match config.write_all(
                    self.admin_entry().as_bytes(),
                ) {
                    Ok(_) => println!("Admin credentials written to file successfully"),
                    Err(e) => println!("Failed to write admin credentials to file: {}", e),
                }
            }
            Err(e) => println!("Failed to open file: {}", e),
        }
    
        Ok(())
    }
}

/// adds user credentials to the config file to authenticate the user and to provide access to a defined tenancy.
/// # Example
/// ```rust
/// use oci_cfg::credentials;
/// 
/// fn main() {
///    credentials(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "passphrase"
///    );
/// }
/// ```
pub fn credentials(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    let admin = Credentials {
        user: String::from(user),
        fingerprint: String::from(fingerprint),
        key_file: String::from(key_file),
        pass_phrase: String::from(pass_phrase)
    };

    let path: String = format!("{}/{}", DIR, NAME); 

    permissions(path.as_str());
    // Call the write_to_config method to write the struct to the file
    if let Err(err) = admin.write_to_config(path.as_str()) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("Profile successfully written to {}", path.as_str());
    }
}

/// reads and returns the content of a config file as a string.
/// # Example
/// ```rust
/// use oci_cfg::report;
/// 
/// fn main() {
///   content();
/// }
/// ```
pub fn content() {
    let path: String = format!("{}/{}", DIR, NAME); 
    read(path.as_str());
}