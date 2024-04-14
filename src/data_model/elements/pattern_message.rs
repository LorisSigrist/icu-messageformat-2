use std::string::ToString;
use crate::data_model::elements;

/// A message without selectors and with a single pattern
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternMessage {
    pub declarations: Vec<elements::Declaration>,
    pub pattern: Vec<elements::PatternElement>,
}

impl ToString for PatternMessage {
    fn to_string(&self) -> String {
        let serialized_pattern = self
            .pattern
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("");

        let should_quote = serialized_pattern.starts_with(".") || !self.declarations.is_empty();
        let serialized_pattern: String = if should_quote {
            format!("{{{{{}}}}}", serialized_pattern)
        } else {
            serialized_pattern
        };

        if self.declarations.is_empty() {
            return serialized_pattern;
        }

        let serialized_declarations = self
            .declarations
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}\n\n{}", serialized_declarations, serialized_pattern)
    }
}
