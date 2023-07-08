use serde::{ Deserialize, Serialize };

use super::photo_size_model::PhotoSize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Animation {
    file_id: String,
    file_unique_id: String,
    width: i32,
    height: i32,
    duration: i32,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<f64>,
    thumbnail: Option<PhotoSize>,
}
