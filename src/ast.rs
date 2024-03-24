use std::string::ToString;

pub enum Message {
    Pattern(PatternMessage),
    Select(SelectMessage),
}

/// A message without selectors and with a single pattern
pub struct PatternMessage {
    pub declarations: Vec<Declaration>,
    pub pattern: Pattern,
}

/// A message that includes selectors
pub struct SelectMessage {
    pub declarations: Vec<Declaration>,
    pub selectors: Vec<Expression>,
    pub variants: Vec<Variant>,
}

pub enum Declaration {
    Input(InputDeclaration),
    Local(LocalDeclaration),
    UnsupportedStatement(UnsupportedStatement),
}

pub struct InputDeclaration {
    /// The name of an InputDeclaration MUST be the same as the name in the VariableRef of its VariableExpression value
    pub name: String,
    pub value: VariableExpression,
}

pub struct LocalDeclaration {
    pub name: String,
    pub value: Expression,
}

pub struct UnsupportedStatement {
    pub keyword: String,
    pub body: Option<String>,
    pub expressions: Vec<Expression>,
}

pub struct Variant {
    pub keys: Vec<VariantKey>,
    pub value: Pattern,
}

pub enum VariantKey {
    Literal(Literal),
    Catchall(CatchallKey),
}

impl ToString for VariantKey {
    fn to_string(&self) -> String {
        match self {
            VariantKey::Literal(l) => l.to_string(),
            VariantKey::Catchall(c) => c.to_string(),
        }
    }
}

pub struct Literal {
    pub value: String,
}

impl std::string::ToString for Literal {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

// For the CatchallKey, a string value may be provided to retain an identifier.
// This is always '*' in MessageFormat 2 syntax, but may vary in other formats.
// This implementation omits this & always uses '*'.
pub struct CatchallKey;

impl ToString for CatchallKey {
    fn to_string(&self) -> String {
        "*".into()
    }
}

pub type Pattern = Vec<PatternElement>;

pub enum PatternElement {
    Literal(String),
    Expression(Expression),
    Markup(Markup),
}

pub enum Expression {
    Literal(LiteralExpression),
    Variable(VariableExpression),
    Function(FunctionExpression),
    Unsupported(UnsupportedExpression),
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

pub struct LiteralExpression {
    pub arg: Literal,
    pub annotation: Option<Annotation>,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<Attribute>,
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

pub struct VariableExpression {
    pub arg: VariableRef,
    pub annotation: Option<Annotation>,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<Attribute>,
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

pub enum Annotation {
    Function(FunctionAnnotation),
    Unsupported(UnsupportedAnnotation),
}

impl ToString for Annotation {
    fn to_string(&self) -> String {
        match self {
            Annotation::Function(f) => f.to_string(),
            Annotation::Unsupported(u) => u.to_string(),
        }
    }
}

pub struct FunctionExpression {
    pub annotation: FunctionAnnotation,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<Attribute>,
}

impl ToString for FunctionExpression {
    fn to_string(&self) -> String {
        self.annotation.to_string() //TODO include attributes
    }
}

pub struct UnsupportedExpression {
    pub annotation: UnsupportedAnnotation,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<Attribute>,
}

impl ToString for UnsupportedExpression {
    fn to_string(&self) -> String {
        self.annotation.to_string() //TODO include attributes
    }
}

/// Attributes are reserved for future standardization
pub struct Attribute {
    pub name: String,
    pub value: Option<AttributeValue>,
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        match &self.value {
            Some(v) => format!("{}={}", self.name, v.to_string()),
            None => self.name.clone(),
        }
    }
}
/// Attributes are reserved for future standardization
pub enum AttributeValue {
    Literal(Literal),
    Variable(VariableRef),
}

impl ToString for AttributeValue {
    fn to_string(&self) -> String {
        match self {
            AttributeValue::Literal(l) => l.to_string(),
            AttributeValue::Variable(v) => v.to_string(),
        }
    }
}

pub struct VariableRef {
    pub name: String,
}

impl ToString for VariableRef {
    fn to_string(&self) -> String {
        format!("${}", self.name)
    }
}

pub struct UnsupportedAnnotation {
    pub source: String,
}

impl ToString for UnsupportedAnnotation {
    fn to_string(&self) -> String {
        self.source.clone()
    }
}

pub struct FunctionAnnotation {
    pub name: String,
    pub options: Vec<OptionValue>,
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
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

pub struct Markup {
    pub kind: MarkupKind,
    pub name: String,
    pub options: Vec<OptionValue>,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<Attribute>,
}

pub enum MarkupKind {
    Open,
    Standalone,
    Close,
}

/// An argument to a function. The spec calls this "option", but that conflicts with Rust's "option".
pub struct MFOption {
    pub name: String,
    pub value: OptionValue,
}

impl ToString for MFOption {
    fn to_string(&self) -> String {
        format!("{}={}", self.name, self.value.to_string())
    }
}

pub enum OptionValue {
    Literal(Literal),
    Variable(VariableRef),
}

impl ToString for OptionValue {
    fn to_string(&self) -> String {
        match self {
            OptionValue::Literal(l) => l.to_string(),
            OptionValue::Variable(v) => v.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serialized_a_variable_option() {
        let option = MFOption {
            name: "foo".into(),
            value: OptionValue::Variable(VariableRef { name: "bar".into() }),
        };

        assert_eq!(option.to_string(), "foo=$bar");
    }

    #[test]
    fn it_serializes_a_literal_option() {
        let option = MFOption {
            name: "foo".into(),
            value: OptionValue::Literal(Literal {
                value: "bar".into(),
            }),
        };

        assert_eq!(option.to_string(), "foo=bar");
    }

    #[test]
    fn it_serializes_a_literal_expression() {
        let expression_without_annotation = LiteralExpression {
            arg: Literal {
                value: "foo".into(),
            },
            annotation: None,
            attributes: vec![],
        };

        assert_eq!(expression_without_annotation.to_string(), "{foo}");

        let expression_with_annotation = LiteralExpression {
            arg: Literal {
                value: "foo".into(),
            },
            annotation: Some(Annotation::Function(FunctionAnnotation {
                name: "bar".into(),
                options: vec![],
            })),
            attributes: vec![],
        };

        assert_eq!(expression_with_annotation.to_string(), "{foo :bar}");
    }

    #[test]
    fn it_serializes_a_variable_expression() {
        let expression_without_annotation = VariableExpression {
            arg: VariableRef {
                name: "foo".into(),
            },
            annotation: None,
            attributes: vec![],
        };

        assert_eq!(expression_without_annotation.to_string(), "{$foo}");
        
        let expression_with_annotation = VariableExpression {
            arg: VariableRef {
                name: "foo".into(),
            },
            annotation: Some(Annotation::Function(FunctionAnnotation {
                name: "bar".into(),
                options: vec![],
            })),
            attributes: vec![],
        };

        assert_eq!(expression_with_annotation.to_string(), "{$foo :bar}");
    }
}
