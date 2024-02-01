use std::io::{BufReader, BufRead};
use std::io::Write;
use std::fs;
use std::fs::File;
use serde::{Serialize, Deserialize};
use clap::Parser;

#[derive(Parser)]
#[command(name = "ocloud")]
#[command(version = "0.1.0")]
#[command(author = "Torsten Boettjer")]
#[command(about = "A tool to connect rust with oracle cloud infrastructure.")]

pub struct CLI {
    #[arg(short('f'))]
    field: usize,
    #[arg(short('d'))]
    delimiter: String,
    #[arg(long)]
    debug: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    tenancy_id: String,
    user_id: String,
    fingerprint: String,
    private_key_path: String,
    region: String,
    pass_phrase: String,
}

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line.");
    line.trim().to_string()
}

pub fn split_line(s: String, cli: &CLI) -> String {
    let parts: Vec<&str> = s.split(&cli.delimiter).collect();
    if cli.debug {
        println!("Parts: {:?}", parts);
        println!("Indexes available starting at 0: {}", parts.len());
    }
    parts.get(cli.field).unwrap_or(&"").to_string()
}

pub fn home() {
    // Specify the path for the new directory
    let dir_path = "$HOME/.ocloud";

    // Attempt to create the directory
    match fs::create_dir(dir_path) {
        Ok(_) => println!("Directory created successfully"),
        Err(e) => eprintln!("Error creating home directory: {}", e),
    }
}

pub fn create() {
    // Create an instance of the data structure
    let account = Account {
        tenancy_id: "ocid1.tenancy.oc1..aaaaaaaaxxxxxxxx".to_string(),
        user_id: "ocid1.user.oc1..aaaaaaaa".to_string(),
        fingerprint: "aa:bb:cc:dd:ee:ff:gg:hh:ii:jj:kk:ll:mm:nn:oo:pp".to_string(),
        private_key_path: "/home/user/.oci/oci_api_key.pem".to_string(),
        region: "us-ashburn-1".to_string(),
        pass_phrase: "passphrase".to_string(),
    };

    // Serialize the data structure to a JSON string
    let json_string = serde_json::to_string(&account).expect("Failed to serialize to JSON");

    // Specify the path for the JSON file
    let file_path = "$HOME/.ocloud/account.json";

    // Attempt to create or open the file
    let mut file = File::create(file_path).expect("Failed to create file");

    // Write the JSON string to the file
    file.write_all(json_string.as_bytes()).expect("Failed to write to file");

    println!("account.json created successfully");
}
