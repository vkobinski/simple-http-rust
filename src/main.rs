use std::net::TcpListener;
use http::{response::IntoResponse, routes::{Method, Routes}};
use serde_json::json;
use crate::http::response::Response;

mod http;

fn get_api() -> String {

    String::from("oi")

}

fn post_api() -> serde_json::Value {

    json!({"teste" : "teste"})

}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    let mut routes = Routes::new();
    let _ = routes.add_route(Method::POST, "/api".to_string(), post_api );
    let _ = routes.add_route(Method::GET, "/api".to_string(), get_api );

    for stream in listener.incoming() {

        match stream {
            Ok(mut stream) => {
                routes.handle(&mut stream);
            }
            Err(e) => println!("Could not get cliente: {e:?}"),
        }
    }

}
