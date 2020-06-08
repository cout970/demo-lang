#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug, Clone)]
pub enum Statement {
    Variable(Variable),
    Expression(Expression),
    TypeDef(TypeDef),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub variants: Vec<TypeDefVariant>,
}

#[derive(Debug, Clone)]
pub struct TypeDefVariant {
    pub name: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Int { value: i32 },
    Float { value: f32 },
    String { value: String },
    FunCall { name: String, args: Vec<Expression> },
    Operator { operator: Operator, left: Box<Expression>, right: Box<Expression> },
    UnaryOperator { operator: UnaryOperator, expr: Box<Expression> },
    List { items: Vec<Expression> },
    Tuple { values: Vec<Expression> },
    Lambda { args: Vec<String>, code: Vec<Statement> },
    Return { value: Box<Expression> }
}

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    BiteAnd,
    BiteOr,
    Plus,
    Minus,
    Times,
    Div,
    Rem,
    Less,
    Greater,
    LessEquals,
    GreaterEquals,
    And,
    Or,
    Xor,
    Equals,
    NotEquals,
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}