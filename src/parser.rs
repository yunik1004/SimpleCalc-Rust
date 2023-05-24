use lazy_static::lazy_static;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::{self, Parser};
use std::io;

#[derive(pest_derive::Parser)]
#[grammar = "calculator.pest"]
pub struct CalculatorParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(plus) | Op::prefix(minus))
    };
}

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    UnaryOp {
        op: UOp,
        rhs: Box<Expr>,
    },
    BinOp {
        lhs: Box<Expr>,
        op: BOp,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum UOp {
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum BOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => BOp::Add,
                Rule::subtract => BOp::Subtract,
                Rule::multiply => BOp::Multiply,
                Rule::divide => BOp::Divide,
                Rule::modulo => BOp::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| {
            let op = match op.as_rule() {
                Rule::plus => UOp::Plus,
                Rule::minus => UOp::Minus,
                rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
            };
            Expr::UnaryOp {
                op,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)
}

pub fn test(input: String) -> io::Result<()> {
    match CalculatorParser::parse(Rule::equation, &input) {
        Ok(mut pairs) => {
            println!(
                "Parsed: {:#?}",
                // inner of expr
                parse_expr(pairs.next().unwrap().into_inner())
            );
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
        }
    }
    Ok(())
}
