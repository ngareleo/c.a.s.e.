use serde::{ Serialize, Deserialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    file_id: String,
    file_unique_id: String,
    file_size: Option<i32>, // in bytes
    file_path: Option<String>,
}
