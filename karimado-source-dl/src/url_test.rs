use super::*;

#[test]
fn ok_git_https() {
    let u = "git+https://github.com/karimadodev/karimado-cli";
    let r = Url::parse(u);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::GitHttps));
    assert_eq!(r.to_string(), u);
}

#[test]
fn ok_https() {
    let u = "https://github.com/karimadodev/karimado-cli";
    let r = Url::parse(u);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::Https));
    assert_eq!(r.to_string(), u);
}

#[test]
fn ok_http() {
    let u = "http://github.com/karimadodev/karimado-cli";
    let r = Url::parse(u);
    assert!(r.is_ok());

    let r = r.unwrap();
    assert!(matches!(r.scheme(), Scheme::Http));
    assert_eq!(r.to_string(), u);
}

#[test]
fn err_unknown() {
    let u = "un://github.com/karimadodev/karimado-cli";
    let r = Url::parse(u);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::UrlParseError(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.contains("the scheme was expected one of"));
    assert!(e.contains("but got \"un\""));
}
