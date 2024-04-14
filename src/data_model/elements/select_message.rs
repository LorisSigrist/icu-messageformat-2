use std::string::ToString;
use crate::data_model::elements;

/// A message that includes selectors
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectMessage {
    pub declarations: Vec<elements::Declaration>,
    pub selectors: Vec<elements::Expression>,
    pub variants: Vec<elements::Variant>,
}

impl ToString for SelectMessage {
    fn to_string(&self) -> String {
        let serialized_selectors = self
            .selectors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        let serialized_variants = self
            .variants
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        let serialized_declarations = self
            .declarations
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        let serialized_match = format!(".match {}\n{}", serialized_selectors, serialized_variants);

        if self.declarations.is_empty() {
            return serialized_match;
        } else {
            format!("{}\n{}", serialized_declarations, serialized_match)
        }
    }
}
