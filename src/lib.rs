//! This is a small library to manage an Oracle Cloud Infrastructure (OCI) config file. 
//! The library checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory.
//! It also checks the permissions before adding content.
//! 
//! More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>
//! # Example
//! ```rust
//! fn main() {
//!     let _ = write_config();
//! }
//! ```

use directories::UserDirs;

pub mod file;
pub mod user;
pub mod region;
pub mod config;
pub mod tenancy;

use file::permissions;
use file::create;
use file::read;
use region::Regions;
use region::home;

use tenancy::acc;
use user::usr;

/// The write function creates a sub-directory in the user's home and writes the required default into the config file.
/// # Example
/// ```rust
/// use oci_config::write;
/// 
/// fn main() {
///     let _ = write();
/// }
/// ```
pub fn write() {
    create(
        ".ocloud",
        "config"
    );
}

/// The tenancy function adds the entries to the config file that allow the user to access another tenancy. It is also used for the initial setup of the user's tenancy.
/// # Example
/// ```rust
/// use oci_config::tenancy;
/// 
/// fn main() {
///    tenancy(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
///     "IAD"
///    );
/// }
/// ```
pub fn tenancy(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: &str) {
    permissions();
    acc(
        user, 
        fingerprint, 
        key_file, 
        tenancy, 
        region
    );
}

/// The user function writes user data for a defined tenancy into the config file.
/// # Example
/// ```rust
/// use oci_config::user;
/// 
/// fn main() {
///    user(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "passphrase"
///    );
/// }
/// ```
pub fn user(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    permissions();
    user(
        user, 
        fingerprint, 
        key_file, 
        pass_phrase
    );
}

/// The read function reads the content of the config file and returns the content as a String.
pub fn read() {
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let config_file = config_path.to_str().expect("Failed to convert path to str");
    read(config_file);
}

/// The check_permissions function checks whether rust can write data into an existing config file. It returns a message indicating whether the file can be opened.
pub fn permissions() {
    let config_path = UserDirs::new().unwrap().home_dir().join(".ocloud/config");
    let config_file = config_path.to_str().expect("Failed to convert path to str");
    permissions(config_file);
}