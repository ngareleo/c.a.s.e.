use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    id: i64,
    #[serde(rename = "type")]
    _type: ChatType,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    is_forum: Option<String>,
    active_usernames: Option<Vec<String>>,
    emoji_status_custom_emoji_id: Option<String>,
    bio: Option<String>,
    has_private_forwards: Option<bool>,
    has_restricted_voice_and_video_messages: Option<bool>,
    join_to_send_messages: Option<bool>,
    join_by_request: Option<bool>,
    description: Option<String>,
    invite_link: Option<String>,
    slow_mode_delay: Option<i64>,
    message_auto_delete_time: Option<i64>,
    has_aggressive_anti_spam_enabled: Option<bool>,
    has_hidden_members: Option<String>,
    has_protected_content: Option<bool>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
    linked_chat_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}
