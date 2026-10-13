mod ast;
mod env;
mod parse;
mod span;
mod ty;
mod value;

#[cfg(test)]
mod tests {
    use super::ast::{Expr, ExprKind, operators};
    use super::span::Span;

    #[test]
    fn ast_test() {
        let tree = Expr::unknown(
            ExprKind::BinOp {
                left: Box::new(Expr::unknown(ExprKind::IntLiteral(1), Span::new(0, 1))),
                operator: operators::Binary::Add,
                right: Box::new(Expr::unknown(ExprKind::FloatLiteral(3.14), Span::new(4, 8))),
            },
            Span::new(0, 8),
        );
        println!("{}", tree);
    }
}
