use std::string::ToString;

pub enum Message {
    Pattern(PatternMessage),
    Select(SelectMessage),
}

impl ToString for Message {
    fn to_string(&self) -> String {
        match self {
            Message::Pattern(p) => p.to_string(),
            Message::Select(s) => s.to_string(),
        }
    }
}

/// A message without selectors and with a single pattern
pub struct PatternMessage {
    pub declarations: Vec<Declaration>,
    pub pattern: Vec<PatternElement>,
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

/// A message that includes selectors
pub struct SelectMessage {
    pub declarations: Vec<Declaration>,
    pub selectors: Vec<Expression>,
    pub variants: Vec<Variant>,
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

pub enum Declaration {
    Input(InputDeclaration),
    Local(LocalDeclaration),
    UnsupportedStatement(UnsupportedStatement),
}

impl ToString for Declaration {
    fn to_string(&self) -> String {
        match self {
            Declaration::Input(i) => i.to_string(),
            Declaration::Local(l) => l.to_string(),
            Declaration::UnsupportedStatement(u) => u.to_string(),
        }
    }
}

pub struct InputDeclaration {
    /// The name of an InputDeclaration MUST be the same as the name in the VariableRef of its VariableExpression value
    pub name: String,
    pub value: VariableExpression,
}

impl ToString for InputDeclaration {
    fn to_string(&self) -> String {
        format!(".input {}={}", self.name, self.value.to_string())
    }
}

pub struct LocalDeclaration {
    pub name: String,
    pub value: Expression,
}

impl ToString for LocalDeclaration {
    fn to_string(&self) -> String {
        format!(".local {}={}", self.name, self.value.to_string())
    }
}

pub struct UnsupportedStatement {
    pub keyword: String,
    pub body: Option<String>,
    pub expressions: Vec<Expression>,
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

pub struct Variant {
    pub keys: Vec<VariantKey>,
    pub value: Vec<PatternElement>,
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

pub enum VariantKey {
    Literal(Literal),
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

pub struct Literal {
    pub value: String,
}

impl std::string::ToString for Literal {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

pub enum PatternElement {
    Literal(String),
    Expression(Expression),
    Markup(Markup),
}

impl ToString for PatternElement {
    fn to_string(&self) -> String {
        match self {
            PatternElement::Literal(l) => l.to_string(),
            PatternElement::Expression(e) => e.to_string(),
            PatternElement::Markup(m) => m.to_string(),
        }
    }
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
        format!("{{{}}}", self.annotation.to_string()) //TODO include attributes
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
            Some(v) => format!("@{}={}", self.name, v.to_string()),
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
    pub options: Vec<MFOption>,
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

pub struct Markup {
    pub kind: MarkupKind,
    pub name: String,
    pub options: Vec<MFOption>,

    /// Attributes are reserved for future standardization
    pub attributes: Vec<Attribute>,
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
                return format!("{{#{} {} }}", self.name, serialized_options);
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
                return format!("{{#{} {} /}}", self.name, serialized_options);
            }

            // {/name}
            MarkupKind::Close => format!("{{/{}}}", self.name),
        }
    }
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
    use std::vec;

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
            arg: VariableRef { name: "foo".into() },
            annotation: None,
            attributes: vec![],
        };

        assert_eq!(expression_without_annotation.to_string(), "{$foo}");

        let expression_with_annotation = VariableExpression {
            arg: VariableRef { name: "foo".into() },
            annotation: Some(Annotation::Function(FunctionAnnotation {
                name: "bar".into(),
                options: vec![],
            })),
            attributes: vec![],
        };

        assert_eq!(expression_with_annotation.to_string(), "{$foo :bar}");
    }

    #[test]
    fn it_serializes_a_funciton_expression() {
        let expression = FunctionExpression {
            annotation: FunctionAnnotation {
                name: "foo".into(),
                options: vec![MFOption {
                    name: "bar".into(),
                    value: OptionValue::Variable(VariableRef { name: "baz".into() }),
                }],
            },
            attributes: vec![],
        };

        assert_eq!(expression.to_string(), "{:foo bar=$baz}");
    }

    #[test]
    fn it_serializes_standalone_markup() {
        let markup = Markup {
            kind: MarkupKind::Standalone,
            name: "foo".into(),
            options: vec![MFOption {
                name: "bar".into(),
                value: OptionValue::Variable(VariableRef { name: "baz".into() }),
            }],
            attributes: vec![],
        };

        assert_eq!(markup.to_string(), "{#foo bar=$baz /}");
    }

    #[test]
    fn it_serializes_pattern_message() {
        let empty_message = PatternMessage {
            declarations: vec![],
            pattern: vec![],
        };

        let hello_world_message = PatternMessage {
            declarations: vec![],
            pattern: vec![
                PatternElement::Literal("Hello ".into()),
                PatternElement::Expression(Expression::Variable(VariableExpression {
                    annotation: None,
                    arg: VariableRef {
                        name: "name".into(),
                    },
                    attributes: vec![],
                })),
                PatternElement::Literal("!".into()),
            ],
        };

        let needs_to_be_quoted = PatternMessage {
            declarations: vec![],
            pattern: vec![PatternElement::Literal(".local".into())],
        };

        assert_eq!(empty_message.to_string(), "");
        assert_eq!(hello_world_message.to_string(), "Hello {$name}!");
        assert_eq!(needs_to_be_quoted.to_string(), "{{.local}}");
    }

    #[test]
    fn it_serializes_select_message() {
        let simple_message = SelectMessage {
            selectors: vec![Expression::Variable(VariableExpression {
                arg: VariableRef { name: "foo".into() },
                annotation: None,
                attributes: vec![],
            })],

            declarations: vec![],
            variants: vec![
                Variant {
                    keys: vec![VariantKey::Literal(Literal { value: "1".into() })],
                    value: vec![PatternElement::Literal("bar".into())],
                },
                Variant {
                    keys: vec![VariantKey::Catchall],
                    value: vec![PatternElement::Literal("baz".into())],
                },
            ],
        };

        let complex_message = SelectMessage {
            selectors: vec![
                Expression::Variable(VariableExpression {
                    arg: VariableRef {
                        name: "fist".into(),
                    },
                    annotation: None,
                    attributes: vec![],
                }),
                Expression::Function(FunctionExpression {
                    attributes: vec![],
                    annotation: FunctionAnnotation {
                        options: vec![],
                        name: "second".into(),
                    },
                }),
            ],
            declarations: vec![Declaration::Local(LocalDeclaration {
                name: "bar".into(),
                value: Expression::Variable(VariableExpression {
                    arg: VariableRef {
                        name: "baz".into(),
                    },
                    annotation: None,
                    attributes: vec![],
                })
            })],
            variants: vec![
                Variant {
                    keys: vec![
                        VariantKey::Literal(Literal { value: "1".into() }),
                        VariantKey::Catchall,
                    ],
                    value: vec![PatternElement::Literal("bar".into())],
                },
                Variant {
                    keys: vec![VariantKey::Catchall, VariantKey::Catchall],
                    value: vec![PatternElement::Literal("baz".into())],
                },
            ],
        };

        assert_eq!(
            simple_message.to_string(),
            ".match {$foo}\n1 {{bar}}\n* {{baz}}"
        );
        assert_eq!(complex_message.to_string(),
            ".local bar={$baz}\n.match {$fist} {:second}\n1 * {{bar}}\n* * {{baz}}"
        )
    }
}
