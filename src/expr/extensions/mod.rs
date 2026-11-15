use cel::{
    IdedExpr, ParseError,
    common::ast::{CallExpr, Expr, ListExpr, LiteralValue},
    parser::MacroExprHelper,
};

/// Request extension module
pub mod request;
/// Response extension module
pub mod response;

pub(crate) const ACCUMULATOR: &str = "@result";

fn extreme(
    helper: &mut MacroExprHelper<'_>,
    target: &str,
    args: &mut Vec<IdedExpr>,
    macro_name: &str,
    function: &str,
) -> Result<Option<IdedExpr>, ParseError> {
    let invalid = match args.as_slice() {
        [arg] if !is_string_list(arg) && !may_be_string(arg) => {
            Some((arg.id, "invalid single argument value"))
        }
        [_] => None,
        args => args
            .iter()
            .find(|arg| !may_be_string(arg))
            .map(|arg| (arg.id, "simple literal arguments must be string")),
    };
    if let Some((id, message)) = invalid {
        return Err(helper.new_error(id, format!("{macro_name}() {message}")));
    }
    let args = vec![
        helper.next_expr(Expr::Ident(target.to_string())),
        helper.next_expr(Expr::List(ListExpr::new(args.to_vec()))),
    ];
    Ok(Some(helper.next_expr(Expr::Call(CallExpr {
        func_name: function.to_owned(),
        target: None,
        args,
    }))))
}

fn may_be_string(arg: &IdedExpr) -> bool {
    match &arg.expr {
        Expr::Literal(literal) => matches!(literal, LiteralValue::String(_)),
        Expr::List(_) | Expr::Map(_) | Expr::Struct(_) => false,
        _ => true,
    }
}

/// Whether `arg` is a list literal of things that can be numbers, not empty.
fn is_string_list(arg: &IdedExpr) -> bool {
    matches!(&arg.expr, Expr::List(list)
        if !list.elements.is_empty() && list.elements.iter().all(may_be_string))
}
