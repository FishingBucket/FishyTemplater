mod ast;
mod span;
mod ty;
mod typecheck;
mod env;
mod value;

#[cfg(test)]
mod tests {
    use super::ast::{Expr, ExprKind, operators};
    use super::span::Span;

    #[test]
    fn ast_test() {
        let tree = Expr::unknown(
            ExprKind::BinOp {
                left: Box::new(Expr::unknown(
                    ExprKind::IntLiteral(1),
                    Span {
                        start: 0,
                        end: 1
                    }
                )),
                operator: operators::Binary::Add,
                right: Box::new(Expr::unknown(
                    ExprKind::FloatLiteral(3.14),
                    Span {
                        start: 4,
                        end: 8
                    }
                ))
            },
            Span {
                start: 0,
                end: 8
            }
        );
        println!("{}", tree);
    }
}
