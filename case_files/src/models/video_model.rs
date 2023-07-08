use serde::{ Deserialize, Serialize };

use super::photo_size_model::PhotoSize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
    file_id: String,
    file_unique_id: String,
    width: i32,
    height: i32,
    duration: i32, // in seconds
    thumbnail: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i32>,
}
