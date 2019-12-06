use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub total_count: i32,
    pub pageable_count: Option<i32>,
    pub is_end: Option<bool>
}