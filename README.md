# OCI Config Parser

This small library enables engineers to create an Oracle Cloud Infrastructure (OCI) config file in Rust. It checks, whether a file already exists, before it writes the config into the sub-directory within the user's home. It checks the permissions before adding content. To instantiate the library it should be addressed as `oci_cfg`, this name is used within the modules. Documentation created using the `make doc` command. 

More information about the config file itself can be found in the official documentation under: <https://docs.oracle.com/en-us/iaas/Content/API/Concepts/sdkconfig.htm>

### Example

```rust
use oci_cfg::{profile, credentials, report};

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
   content();
}
```
## Modules

| Module    | Description |
| :--------: | :------- |
| file  | 	The file module contains helper functions to create, read the content and check the permissions of the config file. It stores the API configuration in a file located in a hidden sub-directory of a user’s home. |
| log  | 	The log module provides options for logging configurations. It contains the ‘LogLevel’ and ‘LogOutput’ enums, and the ‘Logging’ struct. The ‘LogLevel’ enum represents different levels for logging.The ‘LogOutput’ enum represents optional destinations for logging. The ‘Logging’ struct contains the configuration options for controlling logging. |
| region  | 		The region modules provides functions to convert region codes into the corresponding identifier. Regions are represented as an enum and a given code is translated to a string that represents the home region in the tenancy profile. The home region is the region where the user’s tenancy is located, it is used to set the default region for the user’s resources. |

## Structs

| Profile  | 	represents a tenancy profile with the user’s OCID, fingerprint, path to the private key, OCID of the tenancy and the region. |
| Credentials  | 	represents an admin profile with the user’s OCID, fingerprint, path to the private key and the passphrase. |

## Functions

| profile  | 	writes an account profile to the config file, the values are used as defaults for admin users. |
| credentials  | 	adds user credentials to the config file to authenticate the user and to provide access to a defined tenancy. |
| content  | 		reads and returns the content of a config file as a string. |
