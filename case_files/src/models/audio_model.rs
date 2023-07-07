use serde::{ Serialize, Deserialize };

use super::photo_size_model::PhotoSize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    file_id: String,
    file_unique_id: String,
    duration: f32,
    performer: Option<String>,
    title: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i32>,
    thumbnail: Option<PhotoSize>,
}
