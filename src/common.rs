/// Returns true if value is None or empty.
pub fn is_blank(value: &Option<String>) -> bool {
    value.as_ref().unwrap_or(&"".to_string()).is_empty()
}

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
