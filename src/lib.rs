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

use std::path;
use std::path::PathBuf;

use directories::UserDirs;

pub mod file;
pub mod region;
pub mod config;
pub mod account;

use account::{root_compartment, admin};
use file::{create, access, read};
use region::{active_regions, identifier};

static DIR: &str = ".ocloud";
static NAME: &str = "config";

/// The write function creates a sub-directory in the user's home and writes the required default into the config file.
/// # Example
/// ```rust
/// use oci_config::write;
/// 
/// fn main() {
///     let _ = init();
/// }
/// ```
pub fn init() {
    create(
        DIR,
        NAME
    );
}

/// The tenancy function adds the entries to the config file that allow the user to access another tenancy. It is also used for the initial setup of the user's tenancy.
/// # Example
/// ```rust
/// use oci_config::tenancy;
/// 
/// fn main() {
///    tenancyadd(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
///     "IAD"
///    );
/// }
/// ```
pub fn tenancyadd(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: &str) {
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    if access(&path) {
        root_compartment(
            user, 
            fingerprint, 
            key_file, 
            tenancy, 
            region
        );
    } else {
        create(
            DIR,
            NAME
        );
        root_compartment(
            user, 
            fingerprint, 
            key_file, 
            tenancy, 
            region
        );
    }
}

/// The user function writes user data for a defined tenancy into the config file.
/// # Example
/// ```rust
/// use oci_config::user;
/// 
/// fn main() {
///    useradd(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "passphrase"
///    );
/// }
/// ```
pub fn useradd(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    access(".ocloud/config");
    admin(
        user, 
        fingerprint, 
        key_file, 
        pass_phrase
    );
}

/// The read function reads the content of the config file and returns the content as a String.
/// # Example
/// ```rust
/// use oci_config::content;
/// content();
/// ```
pub fn content() {
    read(".ocloud/config");
}

/// The check_permissions function checks whether rust can write data into an existing config file. It returns a message indicating whether the file can be opened.
/// # Example
/// ```rust
/// use oci_config::permissions;
/// permissions();
/// ```
pub fn permissions() {
    access(".ocloud/config");
}

/// The region_identifier function takes a region code as input and returns the corresponding region as a string.
/// # Example
/// ```rust
/// use oci_config::region_identifier;
/// let home_region = home("IAD");
/// ```
pub fn home(region: &str) {
    identifier(region);
}

/// The list_regions function lists all the active regions.
/// # Example
/// ```rust
/// use oci_config::list_regions;
/// regionlist();
/// ```
pub fn regionlist() {
    active_regions();
}