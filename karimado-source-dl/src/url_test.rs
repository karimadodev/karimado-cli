use super::*;

#[test]
fn ok_file() {
    let u = format!("file://{}", "/home/root");
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::File));
    assert_eq!(r.to_file_path().unwrap().to_str().unwrap(), "/home/root");
}

#[test]
fn ok_file_absolute_path() {
    let u = "/home/root";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::File));
    assert_eq!(r.to_file_path().unwrap().to_str().unwrap(), "/home/root");
}

#[test]
fn ok_file_relative_path() {
    let u = "home/root";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::File));

    let actual = r.to_file_path().unwrap();
    let expected = env::current_dir().unwrap().join("home/root");
    assert_eq!(actual, expected);
}

#[test]
fn ok_git_https() {
    let u = "git+https://github.com/karimadodev/karimado-cli";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::GitHttps));
    assert_eq!(r.to_string(), u);
}

#[test]
fn ok_https() {
    let u = "https://github.com/karimadodev/karimado-cli";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::Https));
    assert_eq!(r.to_string(), u);
}

#[test]
fn ok_http() {
    let u = "http://github.com/karimadodev/karimado-cli";
    let r = Url::parse_with_quirks_mode(&u, None);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::Http));
    assert_eq!(r.to_string(), u);
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
