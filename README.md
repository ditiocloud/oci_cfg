# OCI Config Writer

This small library enables engineers to create an Oracle Cloud Infrastructure (OCI) config file with Rust. It checks, whether a file already exists, before it writes the config into the sub-directory within the user's home directory. It also checks the permissions before adding content. To instantiate the library it should be addressed as `oci_config_writer`, this name is used within the modules. Documentation created using the `make doc` command. 

More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>

## Example
```rust
use oci_config_writer::{profile, credentials, report};

fn main() {
   profile(
    "ocid1.user.oc1..aaaaaaaaxxxxxx",
    "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
    "path/to/private/key",
    "ocid1.tenancy.oc1..aaaaaaaaxxxxxx",
    "IAD"
   );
   credentials(
    "ocid1.user.oc1..aaaaaaaaxxxxxx",
    "ocid1.fingerprint.oc1..aaaaaaaaxxxxxx",
    "path/to/private/key",
    "passphrase"
   );
   report();
}
```
