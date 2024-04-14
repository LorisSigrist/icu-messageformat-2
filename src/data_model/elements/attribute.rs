use std::string::ToString;
use crate::data_model::elements;

/// Attributes are reserved for future standardization
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attribute {
    pub name: String,
    pub value: Option<elements::AttributeValue>,
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        match &self.value {
            Some(v) => format!("@{}={}", self.name, v.to_string()),
            None => self.name.clone(),
        }
    }
}
/// Attributes are reserved for future standardization
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValue {
    Literal(elements::Literal),
    Variable(elements::VariableRef),
}

impl ToString for AttributeValue {
    fn to_string(&self) -> String {
        match self {
            Self::Literal(l) => l.to_string(),
            Self::Variable(v) => v.to_string(),
        }
    }
}