use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    file_id: String,
    file_unique_id: String,
    width: i32,
    height: i32,
    file_size: Option<i32>,
}
