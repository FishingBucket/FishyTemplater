use assert_struct::assert_struct;

use crate::expr::{
    ast::{BinOp, Expr, ExprKind},
    ty::{
        tests::util::parse,
        types::{Type, TypeID},
    },
};

/*
    type Object = {
        foo     : int
        bar     : int[]
        recurse : Object
    }

    n       : int
    my_list : int[]
    object  : Object
    max     : (...int) -> int
            | (...float) -> float
    first   : (int[]) -> int
    maybe   : int?
*/

#[test]
fn test_variable() {
    assert_struct!(
        parse("n"),
        Ok(Expr {
            data: ExprKind::Variable("n"),
            ty: Type::Int,
            ..
        })
    );
    assert_struct!(
        parse("n + n"),
        Ok(Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::Variable("n"),
                    ty: Type::Int,
                    ..
                },
                operator: BinOp::Add,
                *right: Expr {
                    data: ExprKind::Variable("n"),
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Int,
            ..
        })
    );
}

#[test]
fn test_attribute() {
    assert_struct!(
        parse("object.foo"),
        Ok(Expr {
            data: ExprKind::Attribute {
                *value: Expr {
                    data: ExprKind::Variable("object"),
                    ty: Type::Object(TypeID(0)),
                    ..
                },
                attribute: "foo"
            },
            ty: Type::Int,
            ..
        })
    );
    assert_struct!(
        parse("object.recurse.foo"),
        Ok(Expr {
            data: ExprKind::Attribute {
                *value: Expr {
                    data: ExprKind::Attribute {
                        *value: Expr {
                            data: ExprKind::Variable("object"),
                            ty: Type::Object(TypeID(0)),
                            ..
                        },
                        attribute: "recurse"
                    },
                    ty: Type::Object(TypeID(0)),
                    ..
                },
                attribute: "foo"
            },
            ty: Type::Int,
            ..
        })
    );
}

#[test]
fn test_index() {
    assert_struct!(
        parse("my_list[0]"),
        Ok(Expr {
            data: ExprKind::Index {
                *value: Expr {
                    data: ExprKind::Variable("my_list"),
                    ty: Type::List {
                        *0: Type::Int
                    },
                    ..
                },
                *index: Expr {
                    data: ExprKind::IntLiteral(0),
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Int,
            ..
        })
    );
}

#[test]
fn test_func_method() {
    assert_struct!(
        parse("first(my_list)"),
        Ok(Expr {
            data: ExprKind::NameCall {
                name: "first",
                arguments: [
                    Expr {
                        data: ExprKind::Variable("my_list"),
                        ty: Type::List {
                            *0: Type::Int
                        },
                        ..
                    }
                ]
            },
            ty: Type::Int,
            ..
        })
    );
    assert_struct!(
        parse("my_list.first()"),
        Ok(Expr {
            data: ExprKind::MethodCall {
                *this: Expr {
                    data: ExprKind::Variable("my_list"),
                    ty: Type::List {
                        *0: Type::Int
                    },
                    ..
                },
                method_name: "first",
                arguments: []
            },
            ty: Type::Int,
            ..
        })
    );
}

#[test]
fn test_overload() {
    assert_struct!(
        parse("max(0, 1)"),
        Ok(Expr {
            data: ExprKind::NameCall {
                name: "max",
                arguments: [
                    Expr {
                        data: ExprKind::IntLiteral(0),
                        ty: Type::Int,
                        ..
                    },
                    Expr {
                        data: ExprKind::IntLiteral(1),
                        ty: Type::Int,
                        ..
                    }
                ]
            },
            ty: Type::Int,
            ..
        })
    );
    assert_struct!(
        parse("(1.0).max(2.0)"),
        Ok(Expr {
            data: ExprKind::MethodCall {
                *this: Expr {
                    data: ExprKind::FloatLiteral(1.0),
                    ty: Type::Float,
                    ..
                },
                method_name: "max",
                arguments: [
                    Expr {
                        data: ExprKind::FloatLiteral(2.0),
                        ty: Type::Float,
                        ..
                    }
                ]
            },
            ty: Type::Float,
            ..
        })
    );
    assert_struct!(
        parse("(1.0).max(2, 4.5)"),
        Ok(Expr {
            data: ExprKind::MethodCall {
                *this: Expr {
                    data: ExprKind::FloatLiteral(1.0),
                    ty: Type::Float,
                    ..
                },
                method_name: "max",
                arguments: [
                    Expr {
                        data: ExprKind::IntLiteral(2),
                        ty: Type::Int,
                        ..
                    },
                    Expr {
                        data: ExprKind::FloatLiteral(4.5),
                        ty: Type::Float,
                        ..
                    }
                ]
            },
            ty: Type::Float,
            ..
        })
    );
}

#[test]
fn test_walrus() {
    assert_struct!(
        parse("(x := 3) * 2"),
        Ok(Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::Walrus {
                        variable: "x",
                        *value: Expr {
                            data: ExprKind::IntLiteral(3),
                            ty: Type::Int,
                            ..
                        }
                    },
                    ty: Type::Int,
                    ..
                },
                operator: BinOp::Times,
                *right: Expr {
                    data: ExprKind::IntLiteral(2),
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Int,
            ..
        })
    );
}

#[test]
fn test_if_walrus() {
    assert_struct!(
        parse("(n := (n -> 4.0) ? n : 2.0) * n"),
        Ok(Expr {
            data: ExprKind::BinOp {
                *left: Expr {
                    data: ExprKind::IfWalrus {
                        variable: "n",
                        *condition: Expr {
                            data: ExprKind::Arrow {
                                *predicate: Expr {
                                    data: ExprKind::Variable("n"),
                                    ty: Type::Int,
                                    ..
                                },
                                *then: Expr {
                                    data: ExprKind::FloatLiteral(4.),
                                    ty: Type::Float,
                                    ..
                                }
                            },
                            ty: Type::Maybe {
                                *0: Type::Float
                            },
                            ..
                        },
                        *branch_true: Expr {
                            data: ExprKind::Variable("n"),
                            ty: Type::Float,
                            ..
                        },
                        *branch_false: Expr {
                            data: ExprKind::FloatLiteral(2.),
                            ty: Type::Float,
                            ..
                        }
                    },
                    ty: Type::Float,
                    ..
                },
                operator: BinOp::Times,
                *right: Expr {
                    data: ExprKind::Variable("n"),
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Float,
            ..
        })
    );
}

#[test]
fn test_arrow_walrus() {
    assert_struct!(
        parse("x := maybe -> x + 1"),
        Ok(Expr {
            data: ExprKind::ArrowWalrus {
                variable: "x",
                *predicate: Expr {
                    data: ExprKind::Variable("maybe"),
                    ty: Type::Maybe {
                        *0: Type::Int
                    },
                    ..
                },
                *then: Expr {
                    data: ExprKind::BinOp {
                        *left: Expr {
                            data: ExprKind::Variable("x"),
                            ty: Type::Int,
                            ..
                        },
                        operator: BinOp::Add,
                        *right: Expr {
                            data: ExprKind::IntLiteral(1),
                            ty: Type::Int,
                            ..
                        }
                    },
                    ty: Type::Int,
                    ..
                }
            },
            ty: Type::Maybe {
                *0: Type::Int
            },
            ..
        })
    );
}
