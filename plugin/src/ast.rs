// taken from https://github.com/lalrpop/lalrpop/blob/793df8d6b4fa1c1bc9c253ee7c173d25cb202a9d/doc/calculator/src/ast.rs
// Copyright (c) 2015 The LALRPOP Project Developers
// used under the MIT license:
// https://github.com/lalrpop/lalrpop/blob/793df8d6b4fa1c1bc9c253ee7c173d25cb202a9d/LICENSE-MIT

use std::fmt::{Debug, Error, Formatter};

pub enum Expr<'input> {
    Number(i32),
    Variable(&'input str),
    Binary(Operator, Box<Expr<'input>>, Box<Expr<'input>>),
}

#[derive(Copy, Clone)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

impl<'input> Debug for Expr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Variable(v) => write!(fmt, "{}", v),
            Binary(op, ref l, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}