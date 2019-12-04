use crate::KakaoCred;
use super::model::*;

pub struct KakaoLocal {
    cred: KakaoCred,
    host: String
}

impl KakaoLocal {
    pub fn new (cred: &KakaoCred) -> Self {
        const HOST: &str = "https://dapi.kakao.com";

        KakaoLocal {
            cred: cred.clone(),
            host: HOST.to_string()
        }
    }

    pub fn search_address(&self, input: &LocalSearchAddressInput) -> Result<LocalSearchAddressOutput, failure::Error>{
        let url = format!("{}/v2/local/search/address.json", self.host);

        let client = reqwest::Client::new();
        let output = client.get(&url)
            .headers(self.cred.authorization_header()?)
            .query(input)
            .send()?
            .json::<LocalSearchAddressOutput>()?;

        println!("{:#?}", output);

        Ok(output)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_search_address() -> Result<(), failure::Error> {
//        let client = KakaoLocal::new(&KakaoCred::new(""));
//        let res = client.search_address(&LocalSearchAddressInput {
//            query: "전북 삼성동 100".to_string(),
//            ..Default::default()
//        })?;
//
//        Ok(())
//    }
//}