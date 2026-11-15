use crate::expr::extensions::extreme;
use cel::{
    DeclarationError, Env, Library, add_overload,
    common::types::{CelBool, CelList, CelMap, CelMapKey, CelString},
    parser::Macro,
};
use simplematch::dowild;

/// Add extension to CEL
pub fn extension(env: &mut Env) -> Result<(), DeclarationError> {
    env.add_library(Response::default())?;
    Ok(())
}

#[derive(Clone, Debug, Default)]
/// Library for response functions
pub struct Response {}

impl Library for Response {
    fn name(&self) -> Option<&str> {
        Some("vetis.res")
    }

    fn register(&self, env: &mut Env) -> Result<(), DeclarationError> {
        add_overload!(env, fn status: (CelMap, CelList) -> CelBool, name = "req.@status")?;
        env.add_macro(Macro::global_var_arg("status", |helper, _, args| {
            extreme(helper, "response", args, "status", "req.@status")
        }))?;
        Ok(())
    }
}

fn status(response: &CelMap, args: &CelList) -> CelBool {
    if let Some((_, val)) = response.get_key_value(&CelMapKey::String("status".into())) {
        let Some(val) = val.downcast_ref::<CelString>() else { return CelBool::FALSE };
        args.iter()
            .any(|status| {
                let Some(code) = status.downcast_ref::<CelString>() else { return false };
                match code.inner() {
                    "2xx" | "2XX" => dowild("2*".as_bytes(), val.as_bytes()),
                    "3xx" | "3XX" => dowild("3*".as_bytes(), val.as_bytes()),
                    "4xx" | "4XX" => dowild("4*".as_bytes(), val.as_bytes()),
                    "5xx" | "5XX" => dowild("5*".as_bytes(), val.as_bytes()),
                    _ => code.eq_ignore_ascii_case(val.inner()),
                }
            })
            .into()
    } else {
        CelBool::FALSE
    }
}
