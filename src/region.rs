
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
/// let home = identifier("IAD");
/// ``` 
pub fn identifier(code: &str) -> String { // translate region code to string
    // Convert input to lowercase for case-insensitivity
    let input_lowercase = code.to_string().trim().to_lowercase();

    // Match the input with the Coin enum
    let code = match input_lowercase.as_str() {
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
    let ident = code.to_string();
    ident
}

pub fn list_region_options() {
    match Regions::IAD {
        Regions::IAD => println!("IAD - Ashburn, US"),
        _ => unreachable!(), // This line is required to make the match exhaustive
    };
    match Regions::LHR {
        Regions::LHR => println!("LHR - London, UK"),
        _ => unreachable!(),
    };
    match Regions::PHX {
        Regions::PHX => println!("PHX - Phoenix, US"),
        _ => unreachable!(),
    };
    match Regions::FRA {
        Regions::FRA => println!("FRA - Frankfurt, DE"),
        _ => unreachable!(),
    };
    match Regions::SYD {
        Regions::SYD => println!("SYD - Sydney, AU"),
        _ => unreachable!(),
    };
    match Regions::MEL {
        Regions::MEL => println!("MEL - Melbourne, AU"),
        _ => unreachable!(),
    };
    match Regions::GRU {
        Regions::GRU => println!("GRU - Sao Paulo, BR"),
        _ => unreachable!(),
    };
    match Regions::VCP {
        Regions::VCP => println!("VCP - Vinhedo, BR"),
        _ => unreachable!(),
    };
    match Regions::YUL {
        Regions::YUL => println!("YUL - Montreal, CA"),
        _ => unreachable!(),
    };
    match Regions::YYZ {
        Regions::YYZ => println!("YYZ - Toronto, CA"),
        _ => unreachable!(),
    };
    match Regions::SCL {
        Regions::SCL => println!("SCL - Santiago, CL"),
        _ => unreachable!(),
    };
    match Regions::VAP {
        Regions::VAP => println!("VAP - Valparaiso, CL"),
        _ => unreachable!(),
    };
    match Regions::BOG {
        Regions::BOG => println!("BOG - Bogota, CO"),
        _ => unreachable!(),
    };
    match Regions::CDG {
        Regions::CDG => println!("CDG - Paris, FR"),
        _ => unreachable!(),
    };
    match Regions::MRS {
        Regions::MRS => println!("MRS - Marseille, FR"),
        _ => unreachable!(),
    };
    match Regions::HYD {
        Regions::HYD => println!("HYD - Hyderabad, IN"),
        _ => unreachable!(),
    };
    match Regions::BOM {
        Regions::BOM => println!("BOM - Mumbai, IN"),
        _ => unreachable!(),
    };
    match Regions::MTZ {
        Regions::MTZ => println!("MTZ - Jerusalem, IL"),
        _ => unreachable!(),
    };
    match Regions::LIN {
        Regions::LIN => println!("LIN - Milan, IT"),
        _ => unreachable!(),
    };
    match Regions::KIX {
        Regions::KIX => println!("KIX - Osaka, JP"),
        _ => unreachable!(),
    };
    match Regions::NRT {
        Regions::NRT => println!("NRT - Tokyo, JP"),
        _ => unreachable!(),
    };
    match Regions::QRO {
        Regions::QRO => println!("QRO - Queretaro, MX"),
        _ => unreachable!(),
    };
    match Regions::MTY {
        Regions::MTY => println!("MTY - Monterrey, MX"),
        _ => unreachable!(),
    };
    match Regions::AMS {
        Regions::AMS => println!("AMS - Amsterdam, NL"),
        _ => unreachable!(),
    };
    match Regions::JED {
        Regions::JED => println!("JED - Jeddah, SA"),
        _ => unreachable!(),
    };
    match Regions::BEG {
        Regions::BEG => println!("BEG - Jovanovac, RS"),
        _ => unreachable!(),
    };
    match Regions::SIN {
        Regions::SIN => println!("SIN - Singapore, SG"),
        _ => unreachable!(),
    };
    match Regions::JNB {
        Regions::JNB => println!("JNB - Johannesburg, ZA"),
        _ => unreachable!(),
    };
    match Regions::ICN {
        Regions::ICN => println!("ICN - Seoul, KR"),
        _ => unreachable!(),
    };
    match Regions::YNY {
        Regions::YNY => println!("YNY - Chuncheon, KR"),
        _ => unreachable!(),
    };
    match Regions::MAD {
        Regions::MAD => println!("MAD - Madrid, ES"),
        _ => unreachable!(),
    };
    match Regions::ARN {
        Regions::ARN => println!("ARN - Stockholm, SE"),
        _ => unreachable!(),
    };
    match Regions::ZRH {
        Regions::ZRH => println!("ZRH - Zurich, CH"),
        _ => unreachable!(),
    };
    match Regions::AUH {
        Regions::AUH => println!("AUH - Abu Dhabi, AE"),
        _ => unreachable!(),
    };
    match Regions::DXB {
        Regions::DXB => println!("DXB - Dubai, AE"),
        _ => unreachable!(),
    };
    match Regions::CWL {
        Regions::CWL => println!("CWL - Cardiff, UK"),
        _ => unreachable!(),
    };
    match Regions::ORD {
        Regions::ORD => println!("ORD - Chicago, US"),
        _ => unreachable!(),
    };
    match Regions::SJC {
        Regions::SJC => println!("SJC - San Jose, US"),
        _ => unreachable!(),
    };
}
