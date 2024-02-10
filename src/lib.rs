//! This is a small library to manage an Oracle Cloud Infrastructure (OCI) config file. 
//! The library checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory.
//! It also checks the permissions before adding content.
//! 
//! More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>
//! # Example
//! ```rust
//! use oci_config::{tenancy, user, content};
//! 
//! fn main() {
//!    tenancy(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
//!     "IAD"
//!    );
//!    user(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "passphrase"
//!    );
//!     content();
//! }
//! ```
pub mod file;
pub mod region;
pub mod config;
pub mod account;

use std::path::PathBuf;
use account::{root_compartment, admin};
use file::{create, access, read};

static DIR: &str = ".ocloud";
static NAME: &str = "config";

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
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    if !path.exists() {
        create(DIR, NAME);
        root_compartment(
            user, 
            fingerprint, 
            key_file, 
            tenancy, 
            region
        );
    } else {
        access(path.to_str().unwrap());
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
///    user(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "passphrase"
///    );
/// }
/// ```
pub fn user(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    access(path.to_str().unwrap());
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
/// fn main() {
///   content();
/// }
/// ```
pub fn content() {
    let mut path = PathBuf::from(DIR);
    path.push(NAME);
    read(path.to_str().unwrap());
}