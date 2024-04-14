use std::string::ToString;
use crate::data_model::elements;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternElement {
    Literal(String),
    Expression(elements::Expression),
    Markup(elements::Markup),
}

impl ToString for PatternElement {
    fn to_string(&self) -> String {
        match self {
            PatternElement::Literal(l) => l.to_string(),
            PatternElement::Expression(e) => e.to_string(),
            PatternElement::Markup(m) => m.to_string(),
        }
    }
}
