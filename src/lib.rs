use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::{BufRead, BufReader}; // for BufReader
use std::io::prelude::*; // for write_all
use directories::UserDirs;

#[derive(Debug)]
struct Default {
    user: String,
    fingerprint: String,
    key_file: String,
    tenancy: String,
    region: Regions, // selection of active regions
}

#[derive(Debug)]
enum Regions {
    IAD,
    PHX,
    FRA,
    LON,
}

impl Default {
    fn new(user: String, fingerprint: String, key_file: String, tenancy: String, region: Regions) -> Default {
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

fn check_permissions(config_file: &str) { // test file permissions 
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
        }
    }
}

pub fn create_file() -> String {
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
        let path_buf = PathBuf::from(file_path);
        let config_file = path_buf.to_str().expect("Failed to convert path to str").to_owned();

        // return the file path as String
        config_file

    } else {
        panic!("Failed to get user's home directory")
    }
}

pub fn set_tenancy(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: Regions) { // write to file
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let path_buf = PathBuf::from(config_path);
    let config_file = path_buf.to_str().expect("Failed to convert path to str");
    check_permissions(config_file);

    let default = Default::new(user.to_string(), fingerprint.to_string(), key_file.to_string(), tenancy.to_string(), region);
    let region = default.home_region();

    let config = OpenOptions::new()
        .write(true)
        .append(true)
        .open(config_file);
        match config {
            Ok(mut config) => {
                match config.write_all(format!(
                    "[DEFAULT]\nuser={}\nfingerprint={}\nkey_file={}\ntenancy={}\nregion={}\n\n", user, fingerprint, key_file, tenancy, region).as_bytes()) {
                Ok(_) => println!("Tenancy data written to file successfully"),
                Err(e) => println!("Failed to write tenancy data to file: {}", e),
            }
        },
        Err(e) => println!("Failed to create file: {}", e),
    }
}

pub fn add_user(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) { // write to config file
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let path_buf = PathBuf::from(config_path);
    let config_file = path_buf.to_str().expect("Failed to convert path to str");
    check_permissions(config_file);
    
    let config = OpenOptions::new()
        .write(true)
        .append(true)
        .open(config_file);
    match config {
        Ok(mut config) => {match config.write_all(format!(
            "[ADMIN_USER]\nuser={}\nfingerprint={}\nkey_file={}\npass_phrase={}\n\n", user, fingerprint, key_file, pass_phrase).as_bytes()) {
                Ok(_) => println!("User data written to file successfully"),
                Err(e) => println!("Failed to write user data to file: {}", e),
            }
        },
        Err(e) => println!("Failed to create file: {}", e),
    }
}

pub fn read_file() { // read from file
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
        }
    }
}
