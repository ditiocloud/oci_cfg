use directories::UserDirs;

#[test]
fn admin_user() {
    let snippet = "ADMIN_USER";
    let file_path = UserDirs::new().unwrap().home_dir().join(".oci/config");
    let config = std::fs::read_to_string(file_path).unwrap();
    let contains = config.contains(snippet);
    assert_eq!(contains, true, "The config file does not contain the section: {}", snippet);
}