//! This is a small library to manage an Oracle Cloud Infrastructure (OCI) config file. 
//! The library checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory.
//! It also checks the permissions before adding content.
//! 
//! More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>
//! # Example
//! ```rust
//! use oci_config::{defaults, admin, report};
//! 
//! fn main() {
//!    defaults(
//!     "ocid1.user.oc1..aaaaaaaaxxxxxx",
//!     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
//!     "path/to/private/key",
//!     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
//!     "IAD"
//!    );
//!    admin(
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
use account::{profile, credentials};
use file::{create, permissions, read};

static DIR: &str = ".ocloud";
static NAME: &str = "config";

/// The defaults function writes the account profile to the config file. It is used grant access to a tenancy.
/// # Example
/// ```rust
/// fn main() {
///    defaults(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
///     "IAD"
///    );
/// }
/// ```
pub fn defaults(user: &str, fingerprint: &str, key_file: &str, tenancy: &str, region: &str) {
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    if !path.exists() {
        create(DIR, NAME);
        profile(
            user, 
            fingerprint, 
            key_file, 
            tenancy, 
            region
        );
    } else {
        permissions(path.to_str().unwrap());
        profile(
            user, 
            fingerprint, 
            key_file, 
            tenancy, 
            region
        );
    }
}

/// The admin function adds user credentials for a user to the config file and provides access to a defined tenancy.
/// # Example
/// ```rust
/// 
/// fn main() {
///    admin(
///     "ocid1.user.oc1..aaaaaaaaxxxxxx",
///     "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
///     "path/to/private/key",
///     "passphrase"
///    );
/// }
/// ```
pub fn admin(user: &str, fingerprint: &str, key_file: &str, pass_phrase: &str) {
    let mut path = PathBuf::from(DIR);
    path.push(NAME);

    permissions(path.to_str().unwrap());
    credentials(
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
///   report();
/// }
/// ```
pub fn report() {
    let mut path = PathBuf::from(DIR);
    path.push(NAME);
    read(path.to_str().unwrap());
}