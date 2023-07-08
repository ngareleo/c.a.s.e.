use serde::{ Deserialize, Serialize };

use super::photo_size_model::PhotoSize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    file_id: String,
    file_unique_id: String,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<String>,
    thumbnail: Option<PhotoSize>,
}
