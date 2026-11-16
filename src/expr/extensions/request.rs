use crate::expr::extensions::{ACCUMULATOR, extreme, is_string_list, may_be_string};
use cel::{
    DeclarationError, Env, IdedExpr, Library, ParseError, add_overload,
    common::{
        ast::{
            CallExpr, ComprehensionExpr,
            Expr::{self},
            ListExpr, LiteralValue, SelectExpr,
            operators::{LOGICAL_NOT, LOGICAL_OR, NOT_STRICTLY_FALSE},
        },
        types::{CelBool, CelList, CelMap, CelMapKey, CelString},
    },
    parser::{Macro, MacroExprHelper},
};
use regex::Regex;
use simplematch::dowild;

/// Add extension to CEL
pub fn extension(env: &mut Env) -> Result<(), DeclarationError> {
    env.add_library(Request::default())?;
    Ok(())
}

#[derive(Clone, Debug, Default)]
/// Library for request functions
pub struct Request {}

impl Library for Request {
    fn name(&self) -> Option<&str> {
        Some("vetis.req")
    }

    fn register(&self, env: &mut Env) -> Result<(), DeclarationError> {
        env.add_macro(Macro::global_var_arg("client_ip", |helper, _, args| {
            contains_ip(helper, args, "client_ip")
        }))?;
        add_overload!(env, fn version: (CelMap, CelList) -> CelBool, name = "req.@version")?;
        env.add_macro(Macro::global_var_arg("version", |helper, _, args| {
            extreme(helper, "request", args, "version", "req.@version")
        }))?;
        add_overload!(env, fn method: (CelMap, CelList) -> CelBool, name = "req.@method")?;
        env.add_macro(Macro::global_var_arg("method", |helper, _, args| {
            extreme(helper, "request", args, "method", "req.@method")
        }))?;
        add_overload!(env, fn scheme: (CelMap, CelList) -> CelBool, name = "req.@scheme")?;
        env.add_macro(Macro::global_var_arg("scheme", |helper, _, args| {
            extreme(helper, "request", args, "scheme", "req.@scheme")
        }))?;
        add_overload!(env, fn host: (CelMap, CelList) -> CelBool, name = "req.@host")?;
        env.add_macro(Macro::global_var_arg("host", |helper, _, args| {
            extreme(helper, "request", args, "host", "req.@host")
        }))?;
        add_overload!(env, fn path: (CelMap, CelList) -> CelBool, name = "req.@path")?;
        env.add_macro(Macro::global_var_arg("path", |helper, _, args| {
            extreme(helper, "request", args, "path", "req.@path")
        }))?;

        Ok(())
    }
}

fn contains_ip(
    helper: &mut MacroExprHelper<'_>,
    args: &mut Vec<IdedExpr>,
    macro_name: &str,
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

    let array = helper.next_expr(Expr::List(ListExpr::new(args.to_vec())));
    let request_ident = helper.next_expr(Expr::Ident("request".to_owned()));
    let range_ident = helper.next_expr(Expr::Ident("range".to_owned()));
    let cidr_call = helper.next_expr(Expr::Call(CallExpr {
        func_name: "cidr".to_owned(),
        target: None,
        args: vec![range_ident],
    }));

    let client_ip_ident = helper.next_expr(Expr::Select(SelectExpr {
        operand: Box::new(request_ident),
        field: "client_ip".to_owned(),
        test: false,
    }));
    let contains_ip_call = helper.next_expr(Expr::Call(CallExpr {
        func_name: "containsIP".to_owned(),
        target: Some(Box::new(cidr_call)),
        args: vec![client_ip_ident],
    }));

    let init = helper.next_expr(Expr::Literal(LiteralValue::Boolean(false.into())));
    let accu_ident = helper.next_expr(Expr::Ident(ACCUMULATOR.to_string()));
    let arg = helper.next_expr(Expr::Call(CallExpr {
        func_name: LOGICAL_NOT.to_string(),
        target: None,
        args: vec![accu_ident],
    }));
    let condition = helper.next_expr(Expr::Call(CallExpr {
        func_name: NOT_STRICTLY_FALSE.to_string(),
        target: None,
        args: vec![arg],
    }));
    let accu = helper.next_expr(Expr::Ident(ACCUMULATOR.to_string()));
    let step = helper.next_expr(Expr::Call(CallExpr {
        func_name: LOGICAL_OR.to_string(),
        target: None,
        args: vec![accu, contains_ip_call],
    }));
    let result = helper.next_expr(Expr::Ident(ACCUMULATOR.to_string()));
    Ok(Some(helper.next_expr(Expr::Comprehension(Box::new(ComprehensionExpr {
        iter_range: array,
        iter_var: "range".to_owned(),
        iter_var2: None,
        accu_var: ACCUMULATOR.to_string(),
        accu_init: init,
        loop_cond: condition,
        loop_step: step,
        result,
    })))))
}

fn contains(map: &CelMap, args: &CelList, field: &str) -> CelBool {
    if let Some((_, val)) = map.get_key_value(&CelMapKey::String(field.into())) {
        args.contains(val)
            .into()
    } else {
        CelBool::FALSE
    }
}

/// Check if request method is any of the arguments of this function
pub fn method(request: &CelMap, args: &CelList) -> CelBool {
    contains(request, args, "method")
}

/// Check if request host is any of the arguments of this function
pub fn host(request: &CelMap, args: &CelList) -> CelBool {
    contains(request, args, "host")
}

/// Check if request protocol scheme is any of the arguments of this function
pub fn scheme(request: &CelMap, args: &CelList) -> CelBool {
    contains(request, args, "scheme")
}

/// Check if request protocol version is any of the arguments of this function
pub fn version(request: &CelMap, args: &CelList) -> CelBool {
    contains(request, args, "version")
}

fn path(request: &CelMap, args: &CelList) -> CelBool {
    if let Some((_, val)) = request.get_key_value(&CelMapKey::String("path".into())) {
        let Some(val) = val.downcast_ref::<CelString>() else { return CelBool::FALSE };
        args.iter()
            .any(|path| {
                if let Some(path) = path.downcast_ref::<CelString>() {
                    dowild(path.as_bytes(), val.as_bytes())
                } else {
                    false
                }
            })
            .into()
    } else {
        CelBool::FALSE
    }
}

#[allow(dead_code)]
fn path_regex(request: &CelMap, args: &CelList) -> CelBool {
    if let Some((_, val)) = request.get_key_value(&CelMapKey::String("path".into())) {
        let Some(val) = val.downcast_ref::<CelString>() else { return CelBool::FALSE };
        args.iter()
            .any(|path| {
                if let Some(path) = path.downcast_ref::<CelString>() {
                    let Ok(regex) = Regex::new(path) else { return false };
                    regex.is_match(val)
                } else {
                    false
                }
            })
            .into()
    } else {
        CelBool::FALSE
    }
}
