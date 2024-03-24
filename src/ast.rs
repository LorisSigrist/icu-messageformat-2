pub enum Message {
    Pattern(PatternMessage),
    Select(SelectMessage),
}

/// A message without selectors and with a single pattern
pub struct PatternMessage {
    pub declarations: Vec<Declaration>,
    pub pattern: Vec<PatternElement>,
}

/// A message that includes selectors
pub struct SelectMessage {
    declarations: Vec<Declaration>,
    selectors: Vec<Expression>,
    variants: Vec<Variant>,
}

enum Declaration {
    Input(InputDeclaration),
    Local(LocalDeclaration),
    UnsupportedStatement(UnsupportedStatement),
}

struct InputDeclaration {
    name: String,
    value: VariableExpression,
}

struct LocalDeclaration {
    name: String,
    value: Expression,
}

struct UnsupportedStatement {
    keyword: String,
    body: Option<String>,
    expressions: Vec<Expression>,
}

struct Variant {
    keys: Vec<VariantKey>,
    value: Vec<PatternElement>,
}

enum VariantKey {
    Literal(Literal),
    Catchall(CatchallKey),
}

struct CatchallKey {
    value: Option<String>,
}

enum PatternElement {
    Literal(String),
    Expression(Expression),
    Markup(Markup),
}

enum Expression {
    Literal(LiteralExpression),
    Variable(VariableExpression),
    Function(FunctionExpression),
    Unsupported,
}

struct LiteralExpression {
    arg: Literal,
    annotation: Option<Annotation>,
    attributes: Vec<Attribute>,
}

struct VariableExpression {
    arg: VariableRef,
    annotation: Option<Annotation>,
    attributes: Vec<Attribute>,
}

enum Annotation {
    Function(FunctionAnnotation),
    Unsupported(UnsupportedAnnotation),
}

struct FunctionExpression {
    annotation: FunctionAnnotation,
    attributes: Vec<Attribute>,
}

struct UnsupportedExpression {
    annotation: UnsupportedAnnotation,
    attributes: Vec<Attribute>,
}

struct Attribute {
    name: String,
    value: Option<AttributeValue>,
}

enum AttributeValue {
    Literal(Literal),
    Variable(VariableRef),
}

struct Literal {
    value: String,
}

struct VariableRef {
    name: String,
}

struct UnsupportedAnnotation {
    source: String,
}

struct FunctionAnnotation {
    name: String,
    options: Vec<ArgumentValue>,
}

struct Markup {
    kind: MarkupKind,
    name: String,
    options: Vec<ArgumentValue>,
    attributes: Vec<Attribute>,
}

enum MarkupKind {
    Open,
    Standalone,
    Close,
}

struct Argument {
    name: String,
    value: ArgumentValue,
}

enum ArgumentValue {
    Literal(Literal),
    Variable(VariableRef),
}
