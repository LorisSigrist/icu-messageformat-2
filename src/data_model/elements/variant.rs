use std::string::ToString;
use crate::data_model::elements;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    pub keys: Vec<elements::VariantKey>,
    pub value: Vec<elements::PatternElement>,
}

impl ToString for Variant {
    fn to_string(&self) -> String {
        let serialized_keys = self
            .keys
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        let serialized_pattern = self
            .value
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("");

        format!("{} {{{{{}}}}}", serialized_keys, serialized_pattern)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VariantKey {
    Literal(elements::Literal),
    Catchall,
}

impl ToString for VariantKey {
    fn to_string(&self) -> String {
        match self {
            VariantKey::Literal(l) => l.to_string(),
            VariantKey::Catchall => "*".into(),
        }
    }
}
