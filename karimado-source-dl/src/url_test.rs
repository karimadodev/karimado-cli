use super::*;

#[test]
fn ok_file() {
    let u = format!("file://{}", "/home/root");
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let url = r.unwrap();
    assert!(matches!(url.scheme(), Scheme::File));

    let path = url.to_file_path().unwrap();
    assert_eq!(path.to_str().unwrap(), "/home/root");
}

#[test]
fn ok_file_absolute_path() {
    let u = "/home/root";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let url = r.unwrap();
    assert!(matches!(url.scheme(), Scheme::File));

    let path = url.to_file_path().unwrap();
    #[cfg(unix)]
    assert_eq!(path.to_str().unwrap(), "/home/root");
    #[cfg(windows)]
    assert_eq!(path.to_str().unwrap(), "D:\\home\\root");
}

#[test]
fn ok_file_relative_path() {
    let u = "home/root";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let url = r.unwrap();
    assert!(matches!(url.scheme(), Scheme::File));

    let actual = url.to_file_path().unwrap();
    let expected = env::current_dir().unwrap().join("home/root");
    assert_eq!(actual, expected);
}

#[test]
fn ok_git_https() {
    let u = "git+https://github.com/karimadodev/karimado-cli";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let url = r.unwrap();
    assert!(matches!(url.scheme(), Scheme::GitHttps));
    assert_eq!(url.to_string(), u);
}

#[test]
fn ok_https() {
    let u = "https://github.com/karimadodev/karimado-cli";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let url = r.unwrap();
    assert!(matches!(url.scheme(), Scheme::Https));
    assert_eq!(url.to_string(), u);
}

#[test]
fn ok_http() {
    let u = "http://github.com/karimadodev/karimado-cli";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let url = r.unwrap();
    assert!(matches!(url.scheme(), Scheme::Http));
    assert_eq!(url.to_string(), u);
}

#[test]
fn err_unknown_scheme() {
    let u = "un://github.com/karimadodev/karimado-cli";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::UrlParseError(UnknownScheme(_)))));

    let e = r.unwrap_err().to_string();
    assert!(e.contains("the scheme was expected one of"));
    assert!(e.contains("but got \"un\""));
}
