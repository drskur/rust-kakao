use serde::{Serialize, Deserialize};
use crate::services::common_model::Meta;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LocalSearchAddressInput {
    pub query: String,
    pub page: Option<i32>,
    pub size: Option<i32>
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LocalGeoCoordToRegionCodeInput {
    pub x: f64,
    pub y: f64,
    pub input_coord: Option<String>,
    pub output_coord: Option<String>,
    pub lang: Option<String>
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LocalGeoCoordToAddressInput {
    pub x: f64,
    pub y: f64,
    pub input_coord: Option<String>
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LocalGeoTranscoordInput {
    pub x: f64,
    pub y: f64,
    pub input_coord: Option<String>,
    pub output_coord: Option<String>
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LocalSearchKeywordInput {
    pub query: String,
    pub category_group_code: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
    pub radius: Option<i32>,
    pub rect: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub sort: Option<i32>
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LocalSearchCategoryInput {
    pub category_group_code: String,
    pub x: Option<String>,
    pub y: Option<String>,
    pub radius: Option<i32>,
    pub rect: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub sort: Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalSearchAddressDocumentAddress {
    pub address_name: String,
    pub region_1depth_name: String,
    pub region_2depth_name: String,
    pub region_3depth_name: String,
    pub region_3depth_h_name: String,
    pub h_code: String,
    pub b_code: String,
    pub mountain_yn: String,
    pub main_address_no: String,
    pub sub_address_no: String,
    pub zip_code: String,
    pub x: String,
    pub y: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalSearchAddressDocumentRoadAddress {
    pub address_name: String,
    pub region_1depth_name: String,
    pub region_2depth_name: String,
    pub region_3depth_name: String,
    pub road_name: String,
    pub underground_yn: String,
    pub main_building_no: String,
    pub sub_building_no: String,
    pub building_name: String,
    pub zone_no: String,
    pub x: String,
    pub y: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalSearchAddressDocument {
    pub address_name: String,
    pub y: String,
    pub x: String,
    pub address_type: String,
    pub address: Option<LocalSearchAddressDocumentAddress>,
    pub road_address: Option<LocalSearchAddressDocumentRoadAddress>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalSearchAddressOutput {
    pub meta: Meta,
    pub documents: Vec<LocalSearchAddressDocument>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalGeoCoordToRegionCodeOutput {
    pub meta: Meta,
    pub documents: Vec<LocalGeoCoordToRegionCodeDocument>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalGeoCoordToRegionCodeDocument {
    pub region_type: String,
    pub address_name: String,
    pub region_1depth_name: String,
    pub region_2depth_name: String,
    pub region_3depth_name: String,
    pub region_4depth_name: String,
    pub code: String,
    // KAKAO의 실수로 보임. API에 따라 자료형이 다른 경우가 있음.
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalGeoCoordToAddressDocumentAddress {
    pub address_name: String,
    pub region_1depth_name: String,
    pub region_2depth_name: String,
    pub region_3depth_name: String,
    pub main_address_no: String,
    pub sub_address_no: String,
    pub zip_code: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalGeoCoordToAddressDocumentRoadAddress {
    pub address_name: String,
    pub region_1depth_name: String,
    pub region_2depth_name: String,
    pub region_3depth_name: String,
    pub road_name: String,
    pub underground_yn: String,
    pub main_building_no: String,
    pub sub_building_no: String,
    pub building_name: String,
    pub zone_no: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalGeoCoordToAddressDocument {
    pub address: Option<LocalGeoCoordToAddressDocumentAddress>,
    pub road_address: Option<LocalGeoCoordToAddressDocumentRoadAddress>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalGeoCoordToAddressOutput {
    pub meta: Meta,
    pub documents: Vec<LocalGeoCoordToAddressDocument>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalGeoTranscoordDocument {
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalGeoTranscoordOutput {
    pub meta: Meta,
    pub documents: Vec<LocalGeoTranscoordDocument>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalSearchKeywordOutput {
    pub meta: Meta,
    pub documents: Vec<LocalSearchKeywordDocument>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalSearchKeywordDocument {
    pub id: String,
    pub place_name: String,
    pub category_name: String,
    pub category_group_code: String,
    pub category_group_name: String,
    pub phone: String,
    pub address_name: String,
    pub road_address_name: String,
    pub x: String,
    pub y: String,
    pub place_url: String,
    pub distance: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalSearchCategoryOutput {
    pub meta: Meta,
    pub documents: Vec<LocalSearchCategoryDocument>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalSearchCategoryDocument {
    pub id: String,
    pub place_name: String,
    pub category_name: String,
    pub category_group_code: String,
    pub category_group_name: String,
    pub phone: String,
    pub address_name: String,
    pub road_address_name: String,
    pub x: String,
    pub y: String,
    pub place_url: String,
    pub distance: String
}