use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Clone, Debug)]
pub struct KakaoCred {
    pub rest_api_key: String
}

impl KakaoCred {
    pub fn new(rest_api_key: &str) -> Self {
        KakaoCred {
            rest_api_key: rest_api_key.to_string()
        }
    }

    pub fn authorization_header(&self) -> Result<HeaderMap, failure::Error> {
        let mut headers = HeaderMap::new();
        let value = HeaderValue::from_str(&format!("KakaoAK {}", self.rest_api_key))?;

        headers.insert("Authorization", value);

        Ok(headers)
    }
}