use log::error;
use regex::Regex;
pub fn auth_validate(value: String, old_value: String) -> String {
    match Regex::new(r"[^a-zA-Z0-9_]") {
        Ok(rex) => rex.replace_all(value.as_str(), "$1").to_string(),
        Err(err) => {
            error!("auth_validate() >> Regex::new: {}", err);
            old_value
        }
    }
}
