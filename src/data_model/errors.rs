// See: https://github.com/unicode-org/message-format-wg/blob/6d7b4ba213e686ff2d403d3025d38d76b42b75f7/spec/errors.md


// Syntax Errors Errors

/// Syntax Errors occur when the syntax representation of a message is not well-formed.
#[derive(Debug)]
pub struct SyntaxError;


// Data Model Errors

/// A Variant Key Mismatch occurs when the number of keys on a variant does not equal the number of selectors.
#[derive(Debug)]
pub struct VariantKeyMismatch;

/// A Missing Fallback Variant error occurs when the message does not include a variant with only catch-all keys.
#[derive(Debug)]
pub struct MissingFallbackVariant;

/// A Missing Selector Annotation error occurs when the message contains a selector that does not have an annotation, or contains a variable that does not directly or indirectly reference a declaration with an annotation.
#[derive(Debug)]
pub struct MissingSelectorAnnotation;

/// A Duplicate Declaration error occurs when a variable is declared more than once. Note that an input variable is implicitly declared when it is first used, so explicitly declaring it after such use is also an error.
#[derive(Debug)]
pub struct DuplicateDeclaration;

/// A Duplicate Option Name error occurs when the same identifier appears on the left-hand side of more than one option in the same expression.
#[derive(Debug)]
pub struct DuplicateOptionName;


// Resolution Errors

/// An Unresolved Variable error occurs when a variable reference cannot be resolved.
#[derive(Debug)]
pub struct UnresolvedVariable;

/// An Unknown Function error occurs when an expression includes a reference to a function which cannot be resolved.
#[derive(Debug)]
pub struct UnknownFunction;

/// An Unsupported Expression error occurs when an expression uses syntax reserved for future standardization, or for private implementation use that is not supported by the current implementation.
#[derive(Debug)]
pub struct UnsupportedExpression;

/// An Invalid Expression error occurs when a message includes an expression whose implementation-defined internal requirements produce an error during function resolution or when a function returns a value (such as null) that the implementation does not support.
#[derive(Debug)]
pub struct InvalidExpression;

/// An Unsupported Statement error occurs when a message includes a reserved statement.
#[derive(Debug)]
pub struct UnsupportedStatement;


// Selection Errors

/// Selection Errors occur when message selection fails.
#[derive(Debug)]
pub struct SelectionError;


// Formatting Errors

/// Formatting Errors occur during the formatting of a resolved value, for example when encountering a value with an unsupported type or an internally inconsistent set of options.
#[derive(Debug)]
pub struct FormattingError;
