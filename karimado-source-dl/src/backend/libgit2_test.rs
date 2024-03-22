use super::*;

#[test]
fn ok() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let downloads_path = tmpdir.path();
    let url = Url::parse("https://github.com/karimadodev/karimado-cli.git#8afbd6d").unwrap();
    let r = download(&url, downloads_path);
    assert!(r.is_ok());

    let target_path = r.unwrap();
    let file = target_path.join("README.md");
    let filedata = std::fs::read_to_string(file).unwrap();
    #[cfg(unix)]
    assert_eq!(filedata, "# Karimado CLI\n");
    #[cfg(windows)]
    assert_eq!(filedata, "# Karimado CLI\r\n");
}
