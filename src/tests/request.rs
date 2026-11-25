use crate::expr::extensions::request;
use cel::{Context, Env, extensions::Network};
use deboa_tokio::HttpBody;
use http::{Method, Request as HttpRequest, Version};
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};
use vetis::Request;

fn create_request<'a>() -> Request {
    let mut inner_request = HttpRequest::builder()
        .uri("https://www.google.com/")
        .version(Version::HTTP_11)
        .method(Method::GET)
        .body(HttpBody::empty())
        .expect("Coult not create request");
    inner_request
        .extensions_mut()
        .insert(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    Request::new(inner_request)
}

#[test]
fn test_client_ip() {
    let request = create_request();
    let request_ref = request.as_ref();
    let mut env = Env::stdlib();
    env.add_library(Network::default())
        .unwrap();
    env.add_library(request::Request::default())
        .unwrap();

    let env = Arc::new(env);
    let program = env
        .compile("client_ip('192.168.0.0/24')")
        .unwrap();
    let mut context = Context::with_env(env.clone());
    context
        .add_variable("request", &request_ref)
        .unwrap();

    let value = program
        .execute(&context)
        .unwrap();

    assert!(matches!(value, cel::Value::Bool(value) if value == true ))
}

#[test]
fn test_method() {
    let request = create_request();
    let request_ref = request.as_ref();
    let mut env = Env::stdlib();
    env.add_library(request::Request::default())
        .unwrap();

    let env = Arc::new(env);
    let program = env
        .compile("method('POST', 'DELETE', 'GET')")
        .unwrap();
    let mut context = Context::with_env(env.clone());
    context
        .add_variable("request", &request_ref)
        .unwrap();

    let value = program
        .execute(&context)
        .unwrap();

    assert!(matches!(value, cel::Value::Bool(value) if value == true ))
}

#[test]
fn test_host() {
    let request = create_request();
    let request_ref = request.as_ref();
    let mut env = Env::stdlib();
    env.add_library(request::Request::default())
        .unwrap();

    let env = Arc::new(env);
    let program = env
        .compile("host('www.apple.com', 'localhost', 'www.google.com')")
        .unwrap();
    let mut context = Context::with_env(env.clone());
    context
        .add_variable("request", &request_ref)
        .unwrap();

    let value = program
        .execute(&context)
        .unwrap();

    assert!(matches!(value, cel::Value::Bool(value) if value == true ))
}

#[test]
fn test_scheme() {
    let request = create_request();
    let request_ref = request.as_ref();
    let mut env = Env::stdlib();
    env.add_library(request::Request::default())
        .unwrap();

    let env = Arc::new(env);
    let program = env
        .compile("scheme('ws', 'https', 'http')")
        .unwrap();
    let mut context = Context::with_env(env.clone());
    context
        .add_variable("request", &request_ref)
        .unwrap();

    let value = program
        .execute(&context)
        .unwrap();

    assert!(matches!(value, cel::Value::Bool(value) if value == true ))
}

#[test]
fn test_version() {
    let request = create_request();
    let request_ref = request.as_ref();
    let mut env = Env::stdlib();
    env.add_library(request::Request::default())
        .unwrap();

    let env = Arc::new(env);
    let program = env
        .compile("version('HTTP/3.0', 'HTTP/1.1')")
        .unwrap();
    let mut context = Context::with_env(env.clone());
    context
        .add_variable("request", &request_ref)
        .unwrap();

    let value = program
        .execute(&context)
        .unwrap();

    assert!(matches!(value, cel::Value::Bool(value) if value == true ))
}
