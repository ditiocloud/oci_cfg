//! This is a small library to manage an Oracle Cloud Infrastructure (OCI) config file. 
//! The library checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory.
//! It also checks the permissions before adding content.
//! 
//! More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>
//! # Example
//! ```rust
//! use oci_config_writer::{defaults, admin, report};
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
pub mod account;

use std::path::PathBuf;
use account::{default, admin};
use file::{create, permissions, read};

static DIR: &str = ".ocloud";
static NAME: &str = "config";

/// writes an account profile to the config file, the values are used as defaults for admin users.
/// # Example
/// ```rust
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
pub fn profile(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: &str) {
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    if !path.exists() {
        create(DIR, NAME);
        default(
            user, 
            fingerprint, 
            key_file, 
            tenancy, 
            region
        );
    } else {
        permissions(path.to_str().unwrap());
        default(
            user, 
            fingerprint, 
            key_file, 
            tenancy, 
            region
        );
    }
}

/// adds user credentials to the config file to authenticate the user and to provide access to a defined tenancy.
/// # Example
/// ```rust
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
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    permissions(path.to_str().unwrap());
    admin(
        user, 
        fingerprint, 
        key_file, 
        pass_phrase
    );
}

/// reads and returns the content of a config file as a string.
/// # Example
/// ```rust
/// use oci_config_writer::content;
/// fn main() {
///   report();
/// }
/// ```
pub fn report() {
    let mut path = PathBuf::from(DIR);
    path.push(NAME);
    read(path.to_str().unwrap());
}