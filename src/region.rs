
//! The region modul presents the active regions to the developer. Regions are represented as an enum, and the active region is selected by the user.
//! The active region is then translated to a string, and used to set the home region for the user. The home region is the region where the user's tenancy is located.
//! The home region is also used to set the default region for the user's resources.
/// # Example:
/// ```rust
/// use oci_config::region::Regions;
/// use oci_config::region::home;
/// let home = home("IAD");
/// ```
#[derive(Debug)]
pub enum Regions {
    SYD,
    MEL,
    GRU,
    VCP,
    YUL,
    YYZ,
    SCL,
    VAP,
    BOG,
    CDG,
    MRS,
    FRA,
    HYD,
    BOM,
    MTZ,
    LIN,
    KIX,
    NRT,
    QRO,
    MTY,
    AMS,
    JED,
    BEG,
    SIN,
    JNB,
    ICN,
    YNY,
    MAD,
    ARN,
    ZRH,
    AUH,
    DXB,
    LHR,
    CWL,
    IAD,
    ORD,
    PHX,
    SJC,
}

/// Implement the ToString trait for the Regions enum.
impl ToString for Regions {
    fn to_string(&self) -> String {
        match self {
            Regions::IAD => String::from("us-ashburn-1"),
            Regions::LHR => String::from("uk-london-1"),
            Regions::PHX => String::from("us-phoenix-1"),
            Regions::FRA => String::from("eu-frankfurt-1"),
            Regions::SYD => String::from("ap-sydney-1"),
            Regions::MEL => String::from("ap-melbourne-1"),
            Regions::GRU => String::from("sa-saopaulo-1"),
            Regions::VCP => String::from("sa-vinhedo-1"),
            Regions::YUL => String::from("ca-montreal-1"),
            Regions::YYZ => String::from("ca-toronto-1"),
            Regions::SCL => String::from("sa-santiago-1"),
            Regions::VAP => String::from("sa-valparaiso-1"),
            Regions::BOG => String::from("sa-bogota-1"),
            Regions::CDG => String::from("eu-paris-1"),
            Regions::MRS => String::from("eu-marseille-1"),
            Regions::HYD => String::from("ap-hyderabad-1"),
            Regions::BOM => String::from("ap-mumbai-1"),
            Regions::MTZ => String::from("il-jerusalem-1"),
            Regions::LIN => String::from("eu-milan-1"),
            Regions::KIX => String::from("ap-osaka-1"),
            Regions::NRT => String::from("ap-tokyo-1"),
            Regions::QRO => String::from("mx-queretaro-1"),
            Regions::MTY => String::from("mx-monterrey-1"),
            Regions::AMS => String::from("eu-amsterdam-1"),
            Regions::JED => String::from("me-jeddah-1"),
            Regions::BEG => String::from("eu-jovanovac-1"),
            Regions::SIN => String::from("ap-singapore-1"),
            Regions::JNB => String::from("af-johannesburg-1"),
            Regions::ICN => String::from("ap-seoul-1"),
            Regions::YNY => String::from("ap-chuncheon-1"),
            Regions::MAD => String::from("eu-madrid-1"),
            Regions::ARN => String::from("eu-stockholm-1"),
            Regions::ZRH => String::from("eu-zurich-1"),
            Regions::AUH => String::from("me-abudhabi-1"),
            Regions::DXB => String::from("me-dubai-1"),
            Regions::CWL => String::from("uk-cardiff-1"),
            Regions::ORD => String::from("us-chicago-1"),
            Regions::SJC => String::from("us-sanjose-1"),
        }
    }
}

/// The home function takes a region code as input and returns the corresponding region as a string.
/// # Example
/// ```rust
/// use oci_config::home;
/// let home = home("IAD");
/// ``` 
pub fn home(code: &str) -> String { // translate region code to string
    // Convert input to lowercase for case-insensitivity
    let input_lowercase = code.to_string().trim().to_lowercase();

    // Match the input with the Coin enum
    let home = match input_lowercase.as_str() {
        "iad" => Regions::IAD,
        "fra" => Regions::FRA,
        "phx" => Regions::PHX,
        "lon" => Regions::LHR,
        _ => {
            println!("Invalid code.");
            std::process::exit(1);
        }
    };
    // Convert the enum variant to a string and print
    let home_string = home.to_string();
    home_string
}
