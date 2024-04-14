pub mod errors;
pub mod elements;

#[cfg(test)]
mod tests {
    use std::vec;
    
    use super::*;
    use elements::*;

    #[test]
    fn it_serialized_a_variable_option() {
        let option = Option {
            name: "foo".into(),
            value: OptionValue::Variable(VariableRef { name: "bar".into() }),
        };

        assert_eq!(option.to_string(), "foo=$bar");
    }

    #[test]
    fn it_serializes_a_literal_option() {
        let option = Option {
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
                options: vec![Option {
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
            options: vec![Option {
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
