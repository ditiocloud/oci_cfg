//! This is a small library that writes and reads to an Oracle Cloud Infrastructure (OCI) config file in the user's home directory.
//! The library checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory.
//! It also checks the permissions before adding content. The library is used by the oci_config crate.
//! More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>

use directories::UserDirs;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*; // for write_all
use std::io::{BufRead, BufReader}; // for BufReader
use std::path::PathBuf;

/// The Default struct represents the DEFAULT section of the config file.
#[derive(Debug)]
pub struct Default {
    user: String,
    fingerprint: String,
    key_file: String,
    tenancy: String,
    region: Regions, // selection of active regions
}

/// The Admin struct represents the ADMIN_USER section of the config file.
#[derive(Debug)]
pub struct Admin {
    user: String,
    fingerprint: String,
    key_file: String,
    pass_phrase: String,
}

/// The Regions enum represents a selection of regions in Oracle Cloud Infrastructure (OCI), made available for the developer.
#[derive(Debug)]
pub enum Regions {
    IAD,
    PHX,
    FRA,
    LON,
}

impl Default {
    fn new(
        user: String,
        fingerprint: String,
        key_file: String,
        tenancy: String,
        region: Regions,
    ) -> Default {
        Self {
            user,
            fingerprint,
            key_file,
            tenancy,
            region,
        }
    }
    // translate region to string
    fn home_region(&self) -> &'static str {
        match self.region {
            Regions::FRA => "eu-frankfurt-1",
            Regions::PHX => "us-phoenix-1",
            Regions::LON => "eu-london-1",
            Regions::IAD => "us-ashburn-1",
        }
    }
}

impl Admin {
    fn new(user: String, fingerprint: String, key_file: String, pass_phrase: String) -> Admin {
        Self {
            user,
            fingerprint,
            key_file,
            pass_phrase,
        }
    }
}

/// The check_permissions function checks the permissions of an existing config file.
pub fn permissions(config_file: &str) {
    // test file permissions
    let config = File::open(config_file);
    match config {
        Ok(config) => {
            let reader = BufReader::new(config);
            for line in reader.lines() {
                match line {
                    Ok(_line) => print!("."),
                    Err(error) => {
                        panic!("Error reading line: {}", error)
                    }
                }
            }
            println!("\nFile read successfully")
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("Opening the file is not allowed: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    }
}

/// The create_file function creates a sub-directory in the user's home directory and a config file in the sub-directory.
/// # Example
/// This example will create a file in the user's home directory and return the file path as a String. The file path is then used to write the tenancy data to the file. At this point, the file is empty.
/// ```rust
/// use oci_config::file;
/// use oci_config::readf;
/// 
/// fn main() {
///     _ = file();
///     let config = readf();
///     println!("{:?}", config);
/// }
/// ```
pub fn file() -> String {
    // Get the user's home directory
    if let Some(user_dirs) = UserDirs::new() {
        println!("Home directory: {:?}", user_dirs.home_dir());

        // Convert &Path to PathBuf and add your sub-directory to the path.
        let mut home_dir_pathbuf = PathBuf::from(user_dirs.home_dir());
        home_dir_pathbuf.push(".ocloud");

        // Check if the sub-directory exists.
        if home_dir_pathbuf.exists() {
            println!("{:?} already exists", home_dir_pathbuf);
        } else {
            // Create the sub-directory.
            match fs::create_dir_all(&home_dir_pathbuf) {
                Err(why) => panic!("! {:?}", why.kind()),
                Ok(_) => println!("Successfully created {:?}", home_dir_pathbuf),
            }
        }

        // Create the config file in the sub-directory.
        let file_path = home_dir_pathbuf.join("config");
        match File::create(&file_path) {
            Err(why) => panic!("! {:?}", why.kind()),
            Ok(_) => println!("Successfully created {:?}", file_path),
        }

        // convert file path to String
        // let path_buf = PathBuf::from(file_path);
        let config_file = file_path
            .to_str()
            .expect("Failed to convert path to str")
            .to_owned();

        // return the file path as String
        config_file
    } else {
        panic!("Failed to get user's home directory")
    }
}

/// The set_tenancy function writes the DEFAULT tenancy data to the config file.
pub fn default(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: Regions) {
    // write to file
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    // let path_buf = PathBuf::from(config_path);
    let config_file = config_path.to_str().expect("Failed to convert path to str");
    permissions(config_file);

    let default = Default::new(
        user.to_string(),
        fingerprint.to_string(),
        key_file.to_string(),
        tenancy.to_string(),
        region,
    );
    let region = default.home_region();

    let config = OpenOptions::new()
        .write(true)
        .append(true)
        .open(config_file);
    match config {
        Ok(mut config) => {
            match config.write_all(
                format!(
                    "[DEFAULT]\nuser={}\nfingerprint={}\nkey_file={}\ntenancy={}\nregion={}\n\n",
                    user, fingerprint, key_file, tenancy, region
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
pub fn admin(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    // write to config file
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    // let path_buf = PathBuf::from(config_path);
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

/// The read_file function reads the content of an existing config file.
pub fn readf() {
    // read from file
    let config_file = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let config = File::open(config_file);
    match config {
        Ok(config) => {
            let reader = BufReader::new(config);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => {
                        panic!("Error reading line: {}", error)
                    }
                }
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("Opening the file is not allowed: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    }
}
