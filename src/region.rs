
//! The region modul presents the active regions to the developer. Regions are represented as an enum, and the active region is selected by the user.
//! The active region is then translated to a string, and used to set the home region for the user. The home region is the region where the user's tenancy is located.
//! The home region is also used to set the default region for the user's resources.
/// # Example:
/// ```rust
/// use oci_config::region::Codes;
/// use oci_config::region::home;
/// let home = home("IAD");
/// ```
#[derive(Debug)]
pub enum Codes {
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
impl ToString for Codes {
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

/// The home identifier function takes a region code as input and returns the corresponding region as a string.
/// # Example
/// ```rust
/// use oci_config::region::identifier;
/// let home = identifier("IAD");
/// ```
pub fn identifier(code: &str) -> String { // translate region code to string
    // Convert input to lowercase for case-insensitivity
    let input_lowercase = code.to_string().trim().to_lowercase();

    // Match the input with the Coin enum
    let code = match input_lowercase.as_str() {
        "iad" => Codes::IAD,
        "fra" => Codes::FRA,
        "phx" => Codes::PHX,
        "lon" => Codes::LHR,
        _ => {
            println!("Invalid code.");
            std::process::exit(1);
        }
    };
    // Convert the enum variant to a string and print
    let id = code.to_string();
    id
}

/// The list_regions function lists all the active regions.
/// # Example
/// ```rust
/// use oci_config::active_regions;
/// regions();
/// ```
pub fn regions() {
    match Codes::IAD {
        Codes::IAD => println!("IAD - Ashburn, US"),
        _ => unreachable!(), // This line is required to make the match exhaustive
    };
    match Codes::LHR {
        Codes::LHR => println!("LHR - London, UK"),
        _ => unreachable!(),
    };
    match Codes::PHX {
        Codes::PHX => println!("PHX - Phoenix, US"),
        _ => unreachable!(),
    };
    match Codes::FRA {
        Codes::FRA => println!("FRA - Frankfurt, DE"),
        _ => unreachable!(),
    };
    match Codes::SYD {
        Codes::SYD => println!("SYD - Sydney, AU"),
        _ => unreachable!(),
    };
    match Codes::MEL {
        Codes::MEL => println!("MEL - Melbourne, AU"),
        _ => unreachable!(),
    };
    match Codes::GRU {
        Codes::GRU => println!("GRU - Sao Paulo, BR"),
        _ => unreachable!(),
    };
    match Codes::VCP {
        Codes::VCP => println!("VCP - Vinhedo, BR"),
        _ => unreachable!(),
    };
    match Codes::YUL {
        Codes::YUL => println!("YUL - Montreal, CA"),
        _ => unreachable!(),
    };
    match Codes::YYZ {
        Codes::YYZ => println!("YYZ - Toronto, CA"),
        _ => unreachable!(),
    };
    match Codes::SCL {
        Codes::SCL => println!("SCL - Santiago, CL"),
        _ => unreachable!(),
    };
    match Codes::VAP {
        Codes::VAP => println!("VAP - Valparaiso, CL"),
        _ => unreachable!(),
    };
    match Codes::BOG {
        Codes::BOG => println!("BOG - Bogota, CO"),
        _ => unreachable!(),
    };
    match Codes::CDG {
        Codes::CDG => println!("CDG - Paris, FR"),
        _ => unreachable!(),
    };
    match Codes::MRS {
        Codes::MRS => println!("MRS - Marseille, FR"),
        _ => unreachable!(),
    };
    match Codes::HYD {
        Codes::HYD => println!("HYD - Hyderabad, IN"),
        _ => unreachable!(),
    };
    match Codes::BOM {
        Codes::BOM => println!("BOM - Mumbai, IN"),
        _ => unreachable!(),
    };
    match Codes::MTZ {
        Codes::MTZ => println!("MTZ - Jerusalem, IL"),
        _ => unreachable!(),
    };
    match Codes::LIN {
        Codes::LIN => println!("LIN - Milan, IT"),
        _ => unreachable!(),
    };
    match Codes::KIX {
        Codes::KIX => println!("KIX - Osaka, JP"),
        _ => unreachable!(),
    };
    match Codes::NRT {
        Codes::NRT => println!("NRT - Tokyo, JP"),
        _ => unreachable!(),
    };
    match Codes::QRO {
        Codes::QRO => println!("QRO - Queretaro, MX"),
        _ => unreachable!(),
    };
    match Codes::MTY {
        Codes::MTY => println!("MTY - Monterrey, MX"),
        _ => unreachable!(),
    };
    match Codes::AMS {
        Codes::AMS => println!("AMS - Amsterdam, NL"),
        _ => unreachable!(),
    };
    match Codes::JED {
        Codes::JED => println!("JED - Jeddah, SA"),
        _ => unreachable!(),
    };
    match Codes::BEG {
        Codes::BEG => println!("BEG - Jovanovac, RS"),
        _ => unreachable!(),
    };
    match Codes::SIN {
        Codes::SIN => println!("SIN - Singapore, SG"),
        _ => unreachable!(),
    };
    match Codes::JNB {
        Codes::JNB => println!("JNB - Johannesburg, ZA"),
        _ => unreachable!(),
    };
    match Codes::ICN {
        Codes::ICN => println!("ICN - Seoul, KR"),
        _ => unreachable!(),
    };
    match Codes::YNY {
        Codes::YNY => println!("YNY - Chuncheon, KR"),
        _ => unreachable!(),
    };
    match Codes::MAD {
        Codes::MAD => println!("MAD - Madrid, ES"),
        _ => unreachable!(),
    };
    match Codes::ARN {
        Codes::ARN => println!("ARN - Stockholm, SE"),
        _ => unreachable!(),
    };
    match Codes::ZRH {
        Codes::ZRH => println!("ZRH - Zurich, CH"),
        _ => unreachable!(),
    };
    match Codes::AUH {
        Codes::AUH => println!("AUH - Abu Dhabi, AE"),
        _ => unreachable!(),
    };
    match Codes::DXB {
        Codes::DXB => println!("DXB - Dubai, AE"),
        _ => unreachable!(),
    };
    match Codes::CWL {
        Codes::CWL => println!("CWL - Cardiff, UK"),
        _ => unreachable!(),
    };
    match Codes::ORD {
        Codes::ORD => println!("ORD - Chicago, US"),
        _ => unreachable!(),
    };
    match Codes::SJC {
        Codes::SJC => println!("SJC - San Jose, US"),
        _ => unreachable!(),
    };
}
