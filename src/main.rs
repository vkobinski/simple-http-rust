use std::net::TcpListener;
use http::{response::IntoResponse, routes::{Method, Route, Routes}};
use serde_json::{json, Result};
use crate::http::response::Response;

mod http;

fn get_api() -> impl IntoResponse {

    String::from("oi")

}


fn post_api() -> impl IntoResponse {

    let response = vec!(String::from("teste"), String::from("teste2"));

    response

}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    let mut routes = Routes::new();
    let _ = routes.add_route(Method::GET, "/api".to_string(), get_api );

    for stream in listener.incoming() {

        println!("ended");

        match stream {
            Ok(mut stream) => {
                routes.handle(&mut stream);
            }
            Err(e) => println!("Could not get cliente: {e:?}"),
        }

        println!("ended");
    }

}
