use std::string::ToString;
use crate::data_model::elements;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Declaration {
    Input(elements::InputDeclaration),
    Local(elements::LocalDeclaration),
    UnsupportedStatement(elements::UnsupportedStatement),
}

impl ToString for Declaration {
    fn to_string(&self) -> String {
        match self {
            Self::Input(i) => i.to_string(),
            Self::Local(l) => l.to_string(),
            Self::UnsupportedStatement(u) => u.to_string(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputDeclaration {
    /// The name of an InputDeclaration MUST be the same as the name in the VariableRef of its VariableExpression value
    pub name: String,
    pub value: elements::VariableExpression,
}

impl ToString for InputDeclaration {
    fn to_string(&self) -> String {
        format!(".input {}={}", self.name, self.value.to_string())
    }
}



#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LocalDeclaration {
    pub name: String,
    pub value: elements::Expression,
}

impl ToString for LocalDeclaration {
    fn to_string(&self) -> String {
        format!(".local {}={}", self.name, self.value.to_string())
    }
}


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnsupportedStatement {
    pub keyword: String,
    pub body: Option<String>,
    pub expressions: Vec<elements::Expression>,
}

impl ToString for UnsupportedStatement {
    fn to_string(&self) -> String {
        format!(
            ".{} {}",
            self.keyword,
            self.body.clone().unwrap_or("".into())
        )
    }
}
