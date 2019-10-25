use super::lex_token::*;
pub type ExprBox<'a> = Box<(ExprBoxInterior<'a>)>;

#[derive(Debug)]
pub struct ExprBoxInterior<'a> {
    pub expr: Expr<'a>,
}

#[derive(Debug)]
pub enum Expr<'a> {
    Call {
        procedure_name: ExprBox<'a>,
        arguments: Vec<ExprBox<'a>>,
    },
    Binary {
        left: ExprBox<'a>,
        operator: Token<'a>,
        right: ExprBox<'a>,
    },
    Grouping {
        expressions: Vec<ExprBox<'a>>,
    },
    ArrayLiteral {
        arguments: Vec<ExprBox<'a>>,
    },
    Literal {
        literal_token: Token<'a>,
    },
    NumberStartDot {
        literal_token: Token<'a>,
    },
    NumberEndDot {
        literal_token: Token<'a>,
    },
    Unary {
        operator: Token<'a>,
        right: ExprBox<'a>,
    },
    Postfix {
        operator: Token<'a>,
        expr: ExprBox<'a>,
    },
    Assign {
        left: ExprBox<'a>,
        operator: Token<'a>,
        right: ExprBox<'a>,
    },
    Identifier {
        name: Token<'a>,
    },
    DotAccess {
        object_name: ExprBox<'a>,
        instance_variable: ExprBox<'a>,
    },
    DataStructureAccess {
        ds_name: ExprBox<'a>,
        access_type: Token<'a>,
        access_exprs: Vec<Expr<'a>>,
    },
    // x ? y : z;
    Ternary {
        conditional: ExprBox<'a>,
        left: ExprBox<'a>,
        right: ExprBox<'a>,
    },
    UnexpectedEnd,
}
