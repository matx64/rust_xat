use chrono::Utc;
use core::fmt;
use std::collections::HashMap;

pub struct ChatMessage {
    pub room_id: String,
    pub username: String,
    pub text: String,
    pub kind: ChatMessageKind,
    pub sent_at: i64,
}

impl ChatMessage {
    pub fn new(
        room_id: &str,
        username: &str,
        text: &str,
        kind: ChatMessageKind,
        sent_at: Option<i64>,
    ) -> Self {
        let text = match kind {
            ChatMessageKind::Join => format!("{} joined the chat.", username),
            ChatMessageKind::Left => format!("{} left the chat.", username),
            ChatMessageKind::Normal => String::from(text),
        };

        let sent_at = match sent_at {
            Some(ts) => ts,
            None => Utc::now().timestamp(),
        };

        Self {
            room_id: String::from(room_id),
            username: String::from(username),
            text,
            kind,
            sent_at,
        }
    }

    pub fn parse(mut data: &str) -> Self {
        if data.ends_with(";") {
            data = &data[..data.len() - 1];
        }

        let mut attrs = HashMap::with_capacity(data.split(";").count());

        for element in data.split(";") {
            let parts: Vec<&str> = element.splitn(2, "=").collect();
            if parts.len() != 2 {
                panic!("Invalid Message format");
            }
            attrs.insert(parts[0], parts[1]);
        }

        let room_id = Self::get_attribute(&attrs, "room_id");
        let username = Self::get_attribute(&attrs, "username");
        let text = Self::get_attribute(&attrs, "text");
        let sent_at = Self::get_attribute(&attrs, "sent_at")
            .parse::<i64>()
            .expect("Invalid sent_at");
        let kind = ChatMessageKind::from_str(Self::get_attribute(&attrs, "kind"));

        Self::new(room_id, username, text, kind, Some(sent_at))
    }

    pub fn to_string(&self) -> String {
        format!(
            "room_id={};username={};text={};kind={};sent_at={}",
            self.room_id, self.username, self.text, self.kind, self.sent_at
        )
    }

    fn get_attribute<'a>(attrs: &'a HashMap<&str, &str>, name: &'a str) -> &'a str {
        attrs.get(name).expect(format!("Missing {}", name).as_str())
    }
}

pub enum ChatMessageKind {
    Normal,
    Join,
    Left,
}

impl ChatMessageKind {
    pub fn from_str(kind: &str) -> Self {
        match kind.to_lowercase().as_str() {
            "join" => ChatMessageKind::Join,
            "left" => ChatMessageKind::Left,
            _ => ChatMessageKind::Normal,
        }
    }
}

impl fmt::Display for ChatMessageKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatMessageKind::Normal => write!(f, "normal"),
            ChatMessageKind::Join => write!(f, "join"),
            ChatMessageKind::Left => write!(f, "left"),
        }
    }
}
