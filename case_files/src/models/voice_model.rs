use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Voice {
    file_id: String,
    file_unique_id: String,
    duration: i32, // in seconds
    mime_type: Option<String>,
    file_size: Option<i32>,
}
