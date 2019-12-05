use reqwest::{Response, StatusCode};
use serde::Serialize;
use std::panic::resume_unwind;
use serde::de::DeserializeOwned;

pub mod local;
pub mod common_model;