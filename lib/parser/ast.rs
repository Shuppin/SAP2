use crate::core::Object;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

pub struct Conditional {
    pub condition: Box<Node>,
    pub body: Box<Node>,
}

pub enum Node {
    Program {
        statements: Vec<Node>,
    },
    Import {
        path: String,
    },
    VariableDecl {
        name: String,
        value: Box<Node>,
    },
    FunctionCall {
        name: String,
        args: Vec<Node>,
    },
    FunctionDecl {
        name: String,
        args: Vec<String>,
        body: Box<Node>,
    },
    Selection {
        if_conditionals: Vec<Conditional>,
        else_conditional: Option<Box<Node>>,
    },
    While {
        conditional: Conditional,
    },
    List {
        elements: Vec<Node>,
    },
    UnaryOp {
        op: Operator,
        child: Box<Node>,
    },
    BinaryOp {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    // Literals
    Literal(Object),
}
