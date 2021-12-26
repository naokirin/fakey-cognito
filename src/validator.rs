use once_cell::sync::Lazy;
use regex::Regex;
use validator::ValidationError;

/// Custom validator for string included in a given string set
pub fn includes(value: &str, set: Vec<&str>) -> Result<(), ValidationError> {
    let matched = set.into_iter().any(|p| value == p);
    if matched {
        return Ok(());
    }
    Err(ValidationError::new("is not included in a given set"))
}

/// Custom validator for array included in given string set
pub fn includes_in_array(value: &[String], set: Vec<&str>) -> Result<(), ValidationError> {
    for v in value.iter() {
        if let err @ Err(_) = includes(v, set.clone()) {
            return err;
        }
    }
    Ok(())
}

pub fn regex_in_array(value: &[String], re: &Lazy<Regex>) -> Result<(), ValidationError> {
    for v in value.iter() {
        if !re.is_match(v) {
            return Err(ValidationError::new("is not match regex"));
        }
    }
    Ok(())
}

pub fn includes_lambda_version(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["V1_0"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_includes() {
        assert!(includes("abc", vec!["a", "ab", "abc"]).is_ok());
    }

    #[test]
    fn failure_includes() {
        assert!(includes("bc", vec!["a", "ab", "abc"]).is_err());
        assert!(includes("", vec!["a", "ab", "abc"]).is_err());
    }

    #[test]
    fn success_includes_in_array() {
        assert!(includes_in_array(
            &["abc".to_string(), "a".to_string()],
            vec!["a", "ab", "abc"]
        )
        .is_ok());
        assert!(includes_in_array(&[], vec!["a", "ab", "abc"]).is_ok());
    }

    #[test]
    fn failure_includes_in_array() {
        assert!(includes_in_array(
            &["abc".to_string(), "c".to_string()],
            vec!["a", "ab", "abc"]
        )
        .is_err());
        assert!(includes_in_array(&["".to_string()], vec!["a", "ab", "abc"]).is_err());
    }
}
