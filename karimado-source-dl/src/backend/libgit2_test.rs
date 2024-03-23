use super::*;

#[test]
fn ok() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let downloads_dir = tmpdir.path();
    let url = Url::parse_with_quirks_mode(
        "git+https://github.com/karimadodev/karimado-cli.git#8afbd6d",
        None,
    )
    .unwrap();
    let r = download(&url, downloads_dir);
    assert!(r.is_ok());

    let path = r.unwrap();
    let file = path.join("README.md");
    let filedata = std::fs::read_to_string(file).unwrap();
    #[cfg(unix)]
    assert_eq!(filedata, "# Karimado CLI\n");
    #[cfg(windows)]
    assert_eq!(filedata, "# Karimado CLI\r\n");
}
