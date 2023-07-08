use serde::{ Deserialize, Serialize };

use super::photo_size_model::PhotoSize;

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoNote {
    file_id: String,
    file_unique_id: String,
    length: i32,
    duration: i32, // in secs
    thumbnail: Option<PhotoSize>,
    file_size: Option<i32>,
}
