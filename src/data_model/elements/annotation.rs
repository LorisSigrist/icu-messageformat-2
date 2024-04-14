use std::string::ToString;
use crate::data_model::elements;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Annotation {
    Function(elements::FunctionAnnotation),
    Unsupported(elements::UnsupportedAnnotation),
}

impl ToString for Annotation {
    fn to_string(&self) -> String {
        match self {
            Annotation::Function(f) => f.to_string(),
            Annotation::Unsupported(u) => u.to_string(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnsupportedAnnotation {
    pub source: String,
}

impl ToString for UnsupportedAnnotation {
    fn to_string(&self) -> String {
        self.source.clone()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionAnnotation {
    pub name: String,
    pub options: Vec<elements::Option>,
}

impl ToString for FunctionAnnotation {
    fn to_string(&self) -> String {
        if self.options.is_empty() {
            return format!(":{}", self.name.clone());
        }

        format!(
            ":{} {}",
            self.name,
            self.options
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}