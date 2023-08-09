use derive_more::{ Display, Error };

use crate::models::{ message_model::Message, user_model::User, chat_model::Chat };

#[derive(Debug, Display, Error)]
pub enum CaseErrors {
    #[display(fmt = "Greeting response not sent")]
    GreetingNotSent,
}

#[derive(Clone)]
pub struct Case {
    pub tg_api_key: String,
}

impl Case {
    pub fn new(tg_api_key: String) -> Self {
        Case {
            tg_api_key,
        }
    }

    pub async fn respond(
        self: &Case,
        message: String,
        user: User,
        chat: Chat
    ) -> Result<(), CaseErrors> {
        Ok(())
    }
}
