use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct HelloResponse {
    pub message: String,
}

const API_BASE_URL: &str = "http://127.0.0.1:8080";

pub async fn get_hello_message() -> Result<HelloResponse, reqwest::Error> {
    reqwest::get(&format!("{}/api/hello", API_BASE_URL))
        .await?
        .json::<HelloResponse>()
        .await
}