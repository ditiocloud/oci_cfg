use directories::UserDirs;

#[test]
fn default_tenancy() {
    let snippet = "DEFAULT";
    let file_path = UserDirs::new().unwrap().home_dir().join(".oci/config");
    let config = std::fs::read_to_string(file_path).unwrap();
    let contains = config.contains(snippet);
    assert_eq!(contains, true, "The config file does not contain the section: {}", snippet);
}