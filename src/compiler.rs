use crate::parser;
use inkwell::{builder::Builder, context::Context, values::IntValue};

pub fn codegen(ast: &parser::Expr, name: &str) -> String {
    let context = Context::create();
    let module = context.create_module(name);

    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[], false);
    let function = module.add_function(name, fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    let result = codegen_ast(ast, &context, &builder);
    builder.build_return(Some(&result));

    module.print_to_string().to_string()
}

fn codegen_ast<'a>(
    ast: &'a parser::Expr,
    context: &'a Context,
    builder: &'a Builder<'a>,
) -> IntValue<'a> {
    match ast {
        parser::Expr::Integer(n) => context.i64_type().const_int(*n as u64, false),
        parser::Expr::UnaryOp { op, rhs } => {
            let rhs_val = codegen_ast(rhs, context, builder);
            match op {
                parser::UOp::Plus => rhs_val,
                parser::UOp::Minus => rhs_val.const_neg(),
            }
        }
        parser::Expr::BinOp { lhs, op, rhs } => {
            let lhs_val = codegen_ast(lhs, context, builder);
            let rhs_val = codegen_ast(rhs, context, builder);
            match op {
                parser::BOp::Add => builder.build_int_add(lhs_val, rhs_val, "add"),
                parser::BOp::Subtract => builder.build_int_sub(lhs_val, rhs_val, "subtract"),
                parser::BOp::Multiply => builder.build_int_mul(lhs_val, rhs_val, "multiply"),
                parser::BOp::Divide => builder.build_int_signed_div(lhs_val, rhs_val, "divide"),
                parser::BOp::Modulo => builder.build_int_signed_rem(lhs_val, rhs_val, "modulo"),
            }
        }
    }
}
