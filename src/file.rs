
/// The file module creates a sub-directory in the user's home directory and a config file in the sub-directory. It returns the path of the config file as a String.
/// # Example
/// ```rust
/// use oci_config::file;
/// 
/// fn main() {
///    let config_dir = ".ocloud";
///    let config_file = "config";
///    create(config_dir, config_file);
/// }
/// ```
use directories::UserDirs;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn create(config_dir: &str, config_file: &str) -> String {
    // Get the user's home directory
    if let Some(user_dirs) = UserDirs::new() {
        println!("Home directory: {:?}", user_dirs.home_dir());

        // Convert &Path to PathBuf and add your sub-directory to the path.
        let mut home_dir_pathbuf = PathBuf::from(user_dirs.home_dir());
        home_dir_pathbuf.push(config_dir);

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
        let file_path = home_dir_pathbuf.join(config_file);
        match File::create(&file_path) {
            Err(why) => panic!("! {:?}", why.kind()),
            Ok(_) => println!("Successfully created {:?}", file_path),
        }

        // convert file path to String
        // let path_buf = PathBuf::from(file_path);
        let config_path = file_path
            .to_str()
            .expect("Failed to convert path to str")
            .to_owned();

        // return the file path as String
        config_path
    } else {
        panic!("Failed to get user's home directory")
    }
}

/// The permissions function checks the permissions of an existing config file. It returns a message indicating whether the file can be read.
/// # Example
/// ```rust
/// use oci_config::permissions;
/// use directories::UserDirs;
/// use std::fs;
/// use std::fs::File;
/// use std::io::prelude::*;
/// use std::io::{BufRead, BufReader};
/// use std::path::PathBuf;
/// 
/// fn main() {
///   let config_file = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
///   permissions(config_file.to_str().unwrap());
/// }
/// ```
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

/// The read function reads and returns the content of an existing config file.
/// # Example
/// ```rust
/// use oci_config::read;
/// use directories::UserDirs;
/// use std::fs;
/// use std::fs::File;
/// use std::io::prelude::*;
/// use std::io::{BufRead, BufReader};
/// use std::path::PathBuf;
/// 
/// fn main() {
///     let config_file = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
///     read(config_file.to_str().unwrap());
/// }
/// ```
pub fn read(config_file: &str) {
    // read from file
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

