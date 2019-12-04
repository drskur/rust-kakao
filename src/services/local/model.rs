use serde::{Serialize, Deserialize};
use crate::services::common_model::Meta;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LocalSearchAddressInput {
    pub query: String,
    pub page: Option<i32>,
    pub size: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSearchAddressDocumentAddress {
    address_name: String,
    region_1depth_name: String,
    region_2depth_name: String,
    region_3depth_name: String,
    region_3depth_h_name: String,
    h_code: String,
    b_code: String,
    mountain_yn: String,
    main_address_no: String,
    sub_address_no: String,
    zip_code: String,
    x: String,
    y: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSearchAddressDocumentRoadAddress {
    address_name: String,
    region_1depth_name: String,
    region_2depth_name: String,
    region_3depth_name: String,
    road_name: String,
    underground_yn: String,
    main_building_no: String,
    sub_building_no: String,
    building_name: String,
    zone_no: String,
    x: String,
    y: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSearchAddressDocument {
    pub address_name: String,
    pub y: String,
    pub x: String,
    pub address_type: String,
    pub address: Option<LocalSearchAddressDocumentAddress>,
    pub road_address: Option<LocalSearchAddressDocumentRoadAddress>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSearchAddressOutput {
    pub meta: Meta,
    pub documents: Vec<LocalSearchAddressDocument>
}