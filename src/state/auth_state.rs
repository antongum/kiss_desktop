use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthState {
    pub name: String,
    pub pin: String,
    pub location: String,
    pub is_auth: bool,
}

impl AuthState {
    pub fn default() -> AuthState {
        AuthState {
            name: "".to_string(),
            pin: "".to_string(),
            location: "".to_string(),
            is_auth: false,
        }
    }

    pub fn set(&mut self, name: String, pin: String, location: String, is_auth: bool) {
        self.name = name;
        self.pin = pin;
        self.location = location;
        self.is_auth = is_auth;
    }
}
