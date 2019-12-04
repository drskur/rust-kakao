use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    total_count: i32,
    pageable_count: Option<i32>,
    is_end: Option<bool>
}