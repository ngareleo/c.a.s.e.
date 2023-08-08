use serde::{Serialize, Deserialize};

use super::{message_model::Message, user_model::User, chat_model::{ChatType, Chat}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    update_id: i32,
    message: Option<Message>,
    edited_message: Option<Message>,
    channel_post: Option<Message>,
    edited_channel_post: Option<Message>,
    inline_query: Option<Message>,
    chosen_inline_result: Option<InlineQuery>,
    callback_query: Option<ChosenInlineResult>,
    shipping_query: Option<ShippingQuery>,
    pre_checkout_query: Option<PreCheckoutQuery>,
    poll: Option<Poll>,
    poll_answer: Option<PollAnswer>,
    #[serde(skip)]
    my_chat_member: Option<ChatMemberUpdated>,
    #[serde(skip)]
    chat_member: Option<ChatMemberUpdated>,
    chat_join_request: Option<ChatJoinRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMemberUpdated {
    chat: Chat,
    from: User,
    date: i32,
    invite_link: Option<ChatInviteLink>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Poll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    total_voter_count: i32,
    is_closed: bool,
    is_anonymous: bool,
    #[serde(rename="type")]
    _type: PollType,
    allows_multiple_answers: bool,
    correct_option_id: Option<i32>,
    explanation: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<i32>,
    close_date: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageEntity {
    _type: MessageEntityType,
    offset: i32,
    length: i32,
    url: Option<String>,
    user: Option<User>,
    language: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Code,
    Pre,
    TextLink,
    TextMention,
}

#[derive(Serialize, Deserialize, Debug)]
struct PollOption {
    text: String,
    voter_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PollAnswer {
    poll_id: String,
    user: User,
    option_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase")]
enum PollType {
    Regular,
    Quiz,
}

#[derive(Serialize, Deserialize, Debug)]
struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: i32,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatInviteLink {
    invite_link: String,
    creator: User,
    creates_join_request: bool,
    is_primary: bool,
    is_revoked: bool,
    name: Option<String>,
    expire_date: Option<i32>,
    member_limit: Option<u32>,
    pending_join_request_count: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatJoinRequest {
    user_chat_id: i32,
    from: User,
    chat: Chat,
    date: i32,
    invite_link: Option<ChatInviteLink>,
    bio: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct ShippingQuery {
    id: String,
    from: User,
    invoice_payload: String,
    shipping_address: ShippingAddress,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChosenInlineResult {
    result_id: String,
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InlineQuery {
    id: String,
    from: User,
    location: Option<Location>,
    query: String,
    offset: String,
    chat_type: Option<ChatType>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    longitude: f32,
    latitude: f32,
    horizontal_accuracy: Option<f32>,
    live_period: Option<i32>,
    heading: Option<i32>,
    proximity_alert_radius: Option<i32>,
}