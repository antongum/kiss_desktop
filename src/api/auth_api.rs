use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    login: String,
    pwd: String,
}

pub async fn auth_admin(login: String, pwd: String) -> Result<String, reqwest::Error> {
    let payload = Payload { login, pwd };
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3000/auth")
        .json(&payload)
        .send()
        .await?;
    println!("{:?}", res);
    Ok("String".to_string())
}
