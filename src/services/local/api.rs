use crate::KakaoCred;
use super::model::*;
use crate::services::KakaoService;

pub struct KakaoLocal {
    service: Box<KakaoService>
}

impl KakaoLocal {
    pub fn new (cred: &KakaoCred) -> Self {
        KakaoLocal {
            service: Box::new(KakaoService::new(cred))
        }
    }

    pub fn search_address(&self, input: &LocalSearchAddressInput) -> Result<LocalSearchAddressOutput, failure::Error> {
        let api = "v2/local/search/address.json";
        self.service.call_api(api, input)
    }

    pub fn geo_coord_to_region_code(&self, input: &LocalGeoCoordToRegionCodeInput) -> Result<LocalGeoCoordToRegionCodeOutput, failure::Error> {
        let api = "v2/local/geo/coord2regioncode";
        self.service.call_api(api, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_address() -> Result<(), failure::Error> {
        let api_key = dotenv::var("KAKAO_REST_API_KEY")?;
        let client = KakaoLocal::new(&KakaoCred::new(&api_key));
        let res = client.search_address(&LocalSearchAddressInput {
            query: "전북 삼성동 100".to_string(),
            ..Default::default()
        })?;

        println!("{:#?}", res);

        Ok(())
    }

    #[test]
    fn test_geo_coord_to_region_code() -> Result<(), failure::Error> {
        let api_key = dotenv::var("KAKAO_REST_API_KEY")?;
        let client = KakaoLocal::new(&KakaoCred::new(&api_key));
        let res = client.geo_coord_to_region_code(&LocalGeoCoordToRegionCodeInput {
            x: 127.1086228,
            y: 37.4012191,
            ..Default::default()
        })?;

        println!("{:#?}", res);

        Ok(())
    }

}