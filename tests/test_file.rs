use directories::UserDirs;

#[test]
fn config_exists() {
    let file_path = UserDirs::new().unwrap().home_dir().join(".oci/config");
    let created = file_path.exists();
    assert_eq!(created, true);
}