use chrono::Utc;

pub struct ChatMessage {
    pub room_id: String,
    pub username: String,
    pub text: String,
    pub kind: ChatMessageKind,
    pub sent_at: i64,
}

pub enum ChatMessageKind {
    Normal,
    Join,
    Left,
}

impl ChatMessage {
    pub fn new(room_id: &str, username: &str, text: &str, kind: ChatMessageKind) -> Self {
        let mut modified_text = String::from(text);

        match kind {
            ChatMessageKind::Join => modified_text = format!("{} joined the chat.", username),
            ChatMessageKind::Left => modified_text = format!("{} left the chat.", username),
            ChatMessageKind::Normal => {}
        }

        ChatMessage {
            room_id: String::from(room_id),
            username: String::from(username),
            text: modified_text,
            kind,
            sent_at: Utc::now().timestamp(),
        }
    }
}
