use crate::api::input::request_result_error::RequestResultError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub enum RequestResult {
    Ok(Value),
    Err(RequestResultError),
}
