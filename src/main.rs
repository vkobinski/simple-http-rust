use std::net::TcpListener;
use http::{response::IntoResponse, routes::{Method, Route, Routes}};
use serde_json::{json, Result};
use crate::http::response::Response;

mod http;

fn get_api() -> impl IntoResponse {

    String::from("oi")

}


fn post_api() -> Response {

    vec!(String::from("teste"), String::from("teste2")).into_response()
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    let mut routes = Routes::new();
    let _ = routes.add_route(Method::GET, "/api".to_string(), post_api );

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
