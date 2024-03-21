use strum::{Display, EnumIter, EnumString};

#[derive(Clone, Display, EnumIter, EnumString)]
pub(crate) enum Scheme {
    #[strum(serialize = "git+https")]
    GitHttps,
    #[strum(serialize = "https")]
    Https,
    #[strum(serialize = "http")]
    Http,
}
