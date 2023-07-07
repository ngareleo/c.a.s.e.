use serde::{ Deserialize, Serialize };

use super::{ photo_size_model::PhotoSize, file_model::File };

#[derive(Debug, Deserialize, Serialize)]
pub struct Sticker {
    file_id: String,
    file_unique_id: String,
    _type: String,
    width: i32,
    height: i32,
    is_animated: bool,
    is_video: bool,
    thumbnail: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    premium_animation: Option<File>,
    mask_position: Option<MaskPosition>,
    custom_emoji_id: Option<String>,
    needs_repainting: Option<bool>,
    file_size: Option<i32>, // in bytes
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaskPosition {}
