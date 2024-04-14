use std::string::ToString;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariableRef {
    pub name: String,
}

impl ToString for VariableRef {
    fn to_string(&self) -> String {
        format!("${}", self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Literal {
    pub value: String,
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}


/// An argument to a function. The spec calls this "option". Watch out for conflicts with Rusts `Option` type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Option {
    pub name: String,
    pub value: OptionValue,
}

impl ToString for Option {
    fn to_string(&self) -> String {
        format!("{}={}", self.name, self.value.to_string())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionValue {
    Literal(Literal),
    Variable(VariableRef),
}

impl ToString for OptionValue {
    fn to_string(&self) -> String {
        match self {
            Self::Literal(l) => l.to_string(),
            Self::Variable(v) => v.to_string(),
        }
    }
}

