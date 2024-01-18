use chrono::Utc;

pub struct Message {
    pub room_id: String,
    pub username: String,
    pub text: String,
    pub kind: MessageKind,
    pub sent_at: i64,
}

pub enum MessageKind {
    Normal,
    Join,
    Left,
}

impl Message {
    pub fn new(room_id: &str, username: &str, text: &str, kind: MessageKind) -> Self {
        let mut modified_text = String::from(text);

        match kind {
            MessageKind::Join => modified_text = format!("{} joined the chat.", username),
            MessageKind::Left => modified_text = format!("{} left the chat.", username),
            MessageKind::Normal => {}
        }

        Message {
            room_id: String::from(room_id),
            username: String::from(username),
            text: modified_text,
            kind,
            sent_at: Utc::now().timestamp(),
        }
    }
}
