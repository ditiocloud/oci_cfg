
//! The region modules provides functions to convert region codes into the corresponding identifier. Regions are represented as an enum and a given code is translated to a string that represents the home region in the tenancy profile. 
//! The home region is the region where the user's tenancy is located, it is used to set the default region for the user's resources.
//! # Example:
//! ```rust
//! use oci_config_writer::region::{identifier, identifiers};
//! 
//! let home = identifier("IAD");
//! let regions = list();
//! println!("The home region identifier is: {}", home);
//! println!("The following regions can be converted with this module: {:?}", regions);
//! ```
#[derive(Debug)]
enum Codes {
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
            Codes::IAD => String::from("us-ashburn-1"),
            Codes::LHR => String::from("uk-london-1"),
            Codes::PHX => String::from("us-phoenix-1"),
            Codes::FRA => String::from("eu-frankfurt-1"),
            Codes::SYD => String::from("ap-sydney-1"),
            Codes::MEL => String::from("ap-melbourne-1"),
            Codes::GRU => String::from("sa-saopaulo-1"),
            Codes::VCP => String::from("sa-vinhedo-1"),
            Codes::YUL => String::from("ca-montreal-1"),
            Codes::YYZ => String::from("ca-toronto-1"),
            Codes::SCL => String::from("sa-santiago-1"),
            Codes::VAP => String::from("sa-valparaiso-1"),
            Codes::BOG => String::from("sa-bogota-1"),
            Codes::CDG => String::from("eu-paris-1"),
            Codes::MRS => String::from("eu-marseille-1"),
            Codes::HYD => String::from("ap-hyderabad-1"),
            Codes::BOM => String::from("ap-mumbai-1"),
            Codes::MTZ => String::from("il-jerusalem-1"),
            Codes::LIN => String::from("eu-milan-1"),
            Codes::KIX => String::from("ap-osaka-1"),
            Codes::NRT => String::from("ap-tokyo-1"),
            Codes::QRO => String::from("mx-queretaro-1"),
            Codes::MTY => String::from("mx-monterrey-1"),
            Codes::AMS => String::from("eu-amsterdam-1"),
            Codes::JED => String::from("me-jeddah-1"),
            Codes::BEG => String::from("eu-jovanovac-1"),
            Codes::SIN => String::from("ap-singapore-1"),
            Codes::JNB => String::from("af-johannesburg-1"),
            Codes::ICN => String::from("ap-seoul-1"),
            Codes::YNY => String::from("ap-chuncheon-1"),
            Codes::MAD => String::from("eu-madrid-1"),
            Codes::ARN => String::from("eu-stockholm-1"),
            Codes::ZRH => String::from("eu-zurich-1"),
            Codes::AUH => String::from("me-abudhabi-1"),
            Codes::DXB => String::from("me-dubai-1"),
            Codes::CWL => String::from("uk-cardiff-1"),
            Codes::ORD => String::from("us-chicago-1"),
            Codes::SJC => String::from("us-sanjose-1"),
        }
    }
}

/// converts a given region code to the corresponding region identifier as a string.
/// # Example
/// ```rust
/// use oci_config_writer::region::identifier;
/// let home = identifier("IAD");
/// ```
pub fn identifier(code: &str) -> String { // translate region code to string
    // Convert input to lowercase for case-insensitivity
    let input_lowercase = code.trim().to_lowercase();

    // Match the input with the Coin enum
    let code = match input_lowercase.as_str() {
        "iad" => Codes::IAD,
        "lon" => Codes::LHR,
        "phx" => Codes::PHX,
        "fra" => Codes::FRA,
        "syd" => Codes::SYD,
        "mel" => Codes::MEL,
        "gru" => Codes::GRU,
        "vcp" => Codes::VCP,
        "yul" => Codes::YUL,
        "yyz" => Codes::YYZ,
        "scl" => Codes::SCL,
        "vap" => Codes::VAP,
        "bog" => Codes::BOG,
        "cdg" => Codes::CDG,
        "mrs" => Codes::MRS,
        "hyd" => Codes::HYD,
        "bom" => Codes::BOM,
        "mtz" => Codes::MTZ,
        "lin" => Codes::LIN,
        "kix" => Codes::KIX,
        "nrt" => Codes::NRT,
        "qro" => Codes::QRO,
        "mty" => Codes::MTY,
        "ams" => Codes::AMS,
        "jed" => Codes::JED,
        "beg" => Codes::BEG,
        "sin" => Codes::SIN,
        "jnb" => Codes::JNB,
        "icn" => Codes::ICN,
        "yny" => Codes::YNY,
        "mad" => Codes::MAD,
        "arn" => Codes::ARN,
        "zrh" => Codes::ZRH,
        "auh" => Codes::AUH,
        "dxb" => Codes::DXB,
        "cwl" => Codes::CWL,
        "ord" => Codes::ORD,
        "sjc" => Codes::SJC,
        _ => {
            println!("Invalid code.");
            std::process::exit(1);
        }
    };
    // Convert the enum variant to a string and print
    let id = code.to_string();
    id
}

/// lists all regions enabled in the module.
/// # Example
/// ```rust
/// use oci_config_writer::region::list;
/// list();
/// ```
pub fn list() {
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
