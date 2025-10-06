#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde::Deserialize;
    use validator::{Validate, ValidationError};

    #[derive(Debug, Validate, Deserialize)]
    struct SignupData {
        #[validate(email)]
        mail: String,
        #[validate(url)]
        site: String,
        #[validate(
            length(min = 1, max = 30),
            custom(function = "validate_unique_username")
        )]
        #[serde(rename = "firstName")]
        first_name: String,
        #[validate(range(min = 18, max = 20))]
        age: u32,
        #[validate(range(exclusive_min = 0.0, max = 100.0))]
        height: f32,
    }

    fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
        if username.starts_with("xxx") && username.ends_with("xxx") {
            Err(ValidationError::new("terrible_username"))
        } else {
            Ok(())
        }
    }

    #[test]
    fn validate() {
        let s = SignupData {
            mail: "test@gmail.com".to_string(),
            site: "https://www.site.com".to_string(),
            first_name: "first".to_string(),
            age: 19,
            height: 50.0,
        };

        // check for no errors
        assert_eq!(s.validate().err(), None);
    }
}
