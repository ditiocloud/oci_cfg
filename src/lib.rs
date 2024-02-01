use std::io::{BufReader, BufRead};
use std::io::Write;
use std::fs;
use std::fs::File;
use std::env::{self, args};
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
struct Account {
    tenancy_id: String,
    user_id: String,
    fingerprint: String,
    private_key_path: String,
    home_region: Regions, // selection of active regions
    pass_phrase: String,
}

#[derive(Debug)]
enum Regions {
    ASH,
    PHX,
    FRA,
    LON,
}

impl Account {
    fn new(tenancy_id: String, user_id: String, fingerprint: String, private_key_path: String, home_region: Regions, pass_phrase: String,) -> Account {
        Self {
            tenancy_id,
            user_id,
            fingerprint,
            private_key_path,
            home_region,
            pass_phrase,
        } 
    }
    // translate popularity flag to meaningful string
    fn region(&self) -> &'static str {
        match self.home_region {
            Regions::FRA => "eu-frankfurt-1",
            Regions::PHX => "us-Phoenix-1",
            Regions::LON => "eu-london-1",
            Regions::ASH => "us-ashburn-1",
        }
    }
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

fn home() {
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
    let account = Account::new(
        String::from("ocid1.tenancy.oc1..aaaaaaaaxxxxxxxx"),
        String::from("ocid1.user.oc1..aaaaaaaa"),
        String::from("aa:bb:cc:dd:ee:ff:gg:hh:ii:jj:kk:ll:mm:nn:oo:pp"),
        String::from("/home/user/.oci/oci_api_key.pem"),
        Regions::FRA,
        String::from("passphrase"),
    );

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

pub fn collect() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        panic!("Please provide six arguments: tenancy and user id, fingerprint, private key path, home region and pass phrase.");
    }

    let account = Account::new(
        String::from(&args[1]),
        String::from(&args[2]),
        String::from(&args[3]),
        String::from(&args[4]),
        Regions::String::from(&args[5]),
        String::from(&args[6]),
    );



    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);

    // Print all variantes
    println!("Account: {:?}", account);
}
