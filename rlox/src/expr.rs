use crate::token::*;
enum Expr {
    Binary(ExprBinary),
    Unary(ExprUnary),
    Literal(ExprLiteral),
    Grouping(ExprGrouping),
}
struct ExprBinary {
    left: Expr,
    operator: Token,
    right: Expr,
}
impl ExprBinary {
    fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
struct ExprUnary {
    operator: Token,
    right: Expr,
}
impl ExprUnary {
    fn new(operator: Token, right: Expr) -> Self {
        Self { operator, right }
    }
}
struct ExprLiteral {
    value: Literal,
}
impl ExprLiteral {
    fn new(value: Literal) -> Self {
        Self { value }
    }
}
struct ExprGrouping {
    expression: Expr,
}
impl ExprGrouping {
    fn new(expression: Expr) -> Self {
        Self { expression }
    }
}
