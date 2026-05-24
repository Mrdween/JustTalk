use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;

// ========== Типы сообщений ==========
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum MessageContent {
    Text(TextContent),
    Audio(AudioContent),
    Video(VideoContent),
    Sticker(String),        // просто ID стикера
    Gif(String),            // URL гифки
    Emoji(String),          // обычно кластером графом
    File(FileContent),
    Vote(VoteContent),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub from_user: String,
    pub to_chat: u64,
    pub content: MessageContent,
    pub is_read: bool,
    pub is_replied: bool,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new_text(from_user: String, to_chat: u64, text: String) -> Self {
        Self {
            id: 0,
            from_user,
            to_chat,
            content: MessageContent::Text(TextContent::new(text)),
            is_read: false,
            is_replied: false,
            created_at: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextContent {
    pub text: String,
    pub font_name: String,
    pub font_size: u16,
    pub color: String,
}

impl TextContent {
    pub fn new(text: String) -> Self {
        Self {
            text,
            font_name: "default".to_string(),
            font_size: 14,
            color: "#000000".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioContent {
    pub url: String,
    pub duration_secs: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoContent {
    pub url: String,
    pub thumbnail_url: String,
    pub duration_secs: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileContent {
    pub name: String,
    pub url: String,
    pub size_bytes: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoteContent {
    pub question: String,
    pub options: Vec<String>,
    pub results: Vec<u32>,
}

fn main() {
    
}