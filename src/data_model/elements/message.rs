use std::string::ToString;
use crate::data_model::elements;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Message {
    Pattern(elements::PatternMessage),
    Select(elements::SelectMessage),
}

impl ToString for Message {
    fn to_string(&self) -> String {
        match self {
            Message::Pattern(p) => p.to_string(),
            Message::Select(s) => s.to_string(),
        }
    }
}