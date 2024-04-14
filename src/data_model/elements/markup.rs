use std::string::ToString;
use crate::data_model::elements;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Markup {
    pub kind: MarkupKind,
    pub name: String,
    pub options: Vec<elements::Option>,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<elements::Attribute>,
}

impl ToString for Markup {
    fn to_string(&self) -> String {
        match self.kind {
            // {#name} or {#name option1=value1 option2=value2 }
            MarkupKind::Open => {
                if self.options.is_empty() {
                    return format!("{{#{}}}", self.name);
                }

                let serialized_options = self
                    .options
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ");

                format!("{{#{} {} }}", self.name, serialized_options)
            }

            // {#tag option1=value1 option2=value2 /}
            MarkupKind::Standalone => {
                if self.options.is_empty() {
                    return format!("{{#{}/}}", self.name);
                }

                let serialized_options = self
                    .options
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ");

                format!("{{#{} {} /}}", self.name, serialized_options)
            }

            // {/name}
            MarkupKind::Close => format!("{{/{}}}", self.name),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MarkupKind {
    Open,
    Standalone,
    Close,
}