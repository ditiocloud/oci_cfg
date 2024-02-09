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
pub mod region;
pub mod config;
pub mod account;

use account::{add_tenancy, add_user};
use file::{create, permissions, read};
use region::{Regions, home};

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
///    add_tenancy(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
///     "IAD"
///    );
/// }
/// ```
pub fn add_tenancy(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: &str) {
    permissions();
    tenancy(
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
///    add_user(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "passphrase"
///    );
/// }
/// ```
pub fn add_user(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    permissions();
    user(
        user, 
        fingerprint, 
        key_file, 
        pass_phrase
    );
}

/// The read function reads the content of the config file and returns the content as a String.
pub fn content() {
    read(".ocloud/config");
}

/// The check_permissions function checks whether rust can write data into an existing config file. It returns a message indicating whether the file can be opened.
pub fn permissions() {
    let config_path = UserDirs::new().unwrap().home_dir().join(config_file);
    let config_file = config_path.to_str().expect("Failed to convert path to str");
    permissions(".ocloud/config");
}