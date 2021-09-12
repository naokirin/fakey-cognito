/// Returns true if value is None or empty.
pub fn is_blank(value: &Option<String>) -> bool {
    value.as_ref().unwrap_or(&"".to_string()).is_empty()
}

pub fn is_none_or_empty_vec<T>(value: &Option<Vec<T>>) -> bool {
    value.is_none() || value.as_ref().unwrap().is_empty()
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

    #[test]
    fn none_is_none_or_empty() {
        let v: Option<Vec<String>> = None;
        assert!(is_none_or_empty_vec(&v));
    }

    #[test]
    fn empty_vec_is_none_or_empty() {
        let v = Some(Vec::<String>::new());
        assert!(is_none_or_empty_vec(&v));
    }

    #[test]
    fn sized_vec_is_not_none_or_empty() {
        let v = Some(vec![1]);
        assert!(!is_none_or_empty_vec(&v));
    }
}
