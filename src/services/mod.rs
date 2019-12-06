use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::KakaoCred;

pub mod local;
pub mod common_model;

struct KakaoService {
    cred: KakaoCred,
    host: &'static str
}

impl KakaoService {
    fn new(cred: &KakaoCred) -> Self {
        const HOST: &'static str = "https://dapi.kakao.com";

        KakaoService {
            cred: cred.clone(),
            host: HOST
        }
    }

    pub fn call_api<Input: Serialize, Output: DeserializeOwned>(&self, api_path: &str, input: &Input) -> Result<Output, failure::Error>{
        let url = format!("{}/{}", self.host, api_path);

        let client = reqwest::Client::new();
        let response = client.get(&url)
            .headers(self.cred.authorization_header()?)
            .query(input)
            .send()?;

        let output = response.error_for_status()
            .and_then(|mut r| r.json::<Output>())?;

        Ok(output)
    }
}

