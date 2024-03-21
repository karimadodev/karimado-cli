use fastrand::alphanumeric;
use std::iter;

pub(crate) fn uuid() -> String {
    let str: String = iter::repeat_with(alphanumeric).take(16).collect();
    format!("kari{}", str)
}
