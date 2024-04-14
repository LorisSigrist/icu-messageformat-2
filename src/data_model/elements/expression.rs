use std::string::ToString;
use crate::data_model::elements;


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    Literal(elements::LiteralExpression),
    Variable(elements::VariableExpression),
    Function(elements::FunctionExpression),
    Unsupported(elements::UnsupportedExpression),
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Literal(l) => l.to_string(),
            Expression::Variable(v) => v.to_string(),
            Expression::Function(f) => f.to_string(),
            Expression::Unsupported(u) => u.to_string(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiteralExpression {
    pub arg: elements::Literal,
    pub annotation: Option<elements::Annotation>,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<elements::Attribute>,
}

impl ToString for LiteralExpression {
    fn to_string(&self) -> String {
        //TODO include attributes
        match &self.annotation {
            Some(annotation) => format!("{{{} {}}}", self.arg.value, annotation.to_string()),
            None => format!("{{{}}}", self.arg.value),
        }
    }
}


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariableExpression {
    pub arg: elements::VariableRef,
    pub annotation: Option<elements::Annotation>,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<elements::Attribute>,
}

impl ToString for VariableExpression {
    fn to_string(&self) -> String {
        //TODO include attributes
        match &self.annotation {
            Some(annotation) => format!("{{{} {}}}", self.arg.to_string(), annotation.to_string()),
            None => format!("{{{}}}", self.arg.to_string()),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionExpression {
    pub annotation: elements::FunctionAnnotation,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<elements::Attribute>,
}

impl ToString for FunctionExpression {
    fn to_string(&self) -> String {
        format!("{{{}}}", self.annotation.to_string()) //TODO include attributes
    }
}


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnsupportedExpression {
    pub annotation: elements::UnsupportedAnnotation,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<elements::Attribute>,
}

impl ToString for UnsupportedExpression {
    fn to_string(&self) -> String {
        self.annotation.to_string() //TODO include attributes
    }
}
