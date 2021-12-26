use once_cell::sync::Lazy;
use regex::Regex;

/// Returns true if value is None or empty.
pub fn is_blank(value: &Option<String>) -> bool {
    value.as_ref().unwrap_or(&"".to_string()).is_empty()
}

/// Arn regex
pub static ARN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"arn:[\w+=/,.@-]+:[\w+=/,.@-]+:([\w+=/,.@-]*)?:[0-9]+:[\w+=/,.@-]+(:[\w+=/,.@-]+)?(:[\w+=/,.@-]+)?").unwrap()
});

/// Application ID regex
pub static APPLICATION_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-fA-F]+$").unwrap());

/// Client ID regex
pub static CLIENT_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w+]+").unwrap());

/// Email regex
pub static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}]+@[\p{L}\p{M}\p{S}\p{N}\p{P}]+").unwrap());

/// User pool id regex
pub static USER_POOL_ID_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\w-]+_[0-9a-zA-Z]+").unwrap());

/// Device key regex
pub static DEVICE_KEY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\w-]+_[0-9a-zA-Z]+").unwrap());

/// Name regex (e.g. Username, GroupName, ...)
pub static NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}]+").unwrap());

/// Password regex
pub static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\S]+").unwrap());

/// Event ID regex
pub static EVENT_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w+-]+").unwrap());

/// Token regex
pub static TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Za-z0-9-_=.]+").unwrap());

/// Code regex
pub static CODE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\S]+").unwrap());

/// Hash regex
pub static HASH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w+=/]+").unwrap());

pub static URL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}]+").unwrap());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_is_blank() {
        assert!(is_blank(&None));
    }

    #[test]
    fn empty_is_blank() {
        assert!(is_blank(&Some("".to_string())));
    }

    #[test]
    fn string_is_not_blank() {
        assert!(!is_blank(&Some("some".to_string())));
    }
}
