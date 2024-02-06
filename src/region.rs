
/// The Regions modul presents the active regions to the developer. Regions are represented as an enum, and the active region is selected by the user.
/// The active region is then translated to a string, and used to set the home region for the user. The home region is the region where the user's tenancy is located.
/// The home region is also used to set the default region for the user's resources.
/// # Example:
/// ```rust
/// use oci_config::Regions
/// let home = home(IAD);
/// ```

/// The Regions enum represents a selection of regions in Oracle Cloud Infrastructure (OCI), made available for the developer.
#[derive(Debug)]
pub enum Regions {
    IAD,
    PHX,
    FRA,
    LON,
}

// Implement the ToString trait for the Regions enum
impl ToString for Regions {
    fn to_string(&self) -> String {
        match self {
            Regions::IAD => String::from("us-ashburn-1"),
            Regions::LON => String::from("uk-london-1"),
            Regions::PHX => String::from("us-phoenix-1"),
            Regions::FRA => String::from("eu-frankfurt-1"),
        }
    }
}


pub fn home(code: &str) -> String { // translate region code to string
    // Convert input to lowercase for case-insensitivity
    let input_lowercase = code.to_string().trim().to_lowercase();

    // Match the input with the Coin enum
    let home = match input_lowercase.as_str() {
        "iad" => Regions::IAD,
        "fra" => Regions::FRA,
        "phx" => Regions::PHX,
        "lon" => Regions::LON,
        _ => {
            println!("Invalid code.");
            std::process::exit(1);
        }
    };
    // Convert the enum variant to a string and print
    let home_string = home.to_string();
    home_string
}
