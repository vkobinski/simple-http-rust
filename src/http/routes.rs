use std::net::TcpStream;
use std::io::{Read, Write};

use crate::Response;

use super::response::{IntoResponse, StatusCode};

#[derive(PartialEq, Debug)]
pub enum Method {
    GET,
    POST,
}

impl From<&str> for Method {

    fn from(str: &str) -> Method {
        match str {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => panic!("Could not parse method"),
        }
    }
}


pub struct Route<T>
where T: IntoResponse
{
    pub method: Method,
    pub path: String,
    pub func: fn() -> T,
}

impl<T> Route<T>
where T: IntoResponse
{
    pub fn new(method: Method, path: String, func: fn() -> T)  -> Self {
        Self {
            method,
            path,
            func
        }
    }
}

pub struct Routes<T>
where T: IntoResponse
{
    pub routes : Vec<Route<T>>
}

#[derive(Debug)]
pub enum RouteError {
    AlreadyInUse,
    DoesNotExist,
    CouldNotProcess

}

impl<T> Routes<T>
where T: IntoResponse
{
    pub fn new() -> Self {
        Self {
            routes: vec!(),
        }
    }

    pub fn add_route(&mut self, method: Method, path: String, func: fn() -> T) -> Result<(), RouteError> {

        let route = Route::new(method, path, func);

        if let Some(_) = self.routes.iter().find(|r| r.path == route.path && r.method == route.method) {
                Err(RouteError::AlreadyInUse)
        } else {
            self.routes.push(route);
            Ok(())
        }
    }

    pub fn process_request(&self, stream: &mut TcpStream,path: String, method: &str) -> Result<(), RouteError> {
        let method: Method = Method::from(method);

        if let Some(&ref route) = self.routes.iter().find(|r| r.path == path && r.method == method) {
            let res = (route.func)();
            let str : String = (res).into_response().into();
            stream.write_all(str.as_bytes()).unwrap();
            Ok(())
        } else {
            Err(RouteError::DoesNotExist)
        }

    }

    pub fn handle(&self, stream: &mut TcpStream) {

        let mut buf = [0; 100];
        stream.read_exact(&mut buf).unwrap();

        let string_buf = String::from_utf8(buf.to_vec()).unwrap();

        let mut tokens = string_buf.split(" ");

        let method = tokens.next().unwrap();
        let path = tokens.next().unwrap().to_string();

        match self.process_request(stream, path, method) {
            Ok(_) => {
            }
            Err(_) => {
                let res = Response::new(StatusCode::NOT_FOUND, "");
                let str : String = res.into();
                stream.write_all(str.as_bytes()).unwrap();
            }
        }

    }

}

#[macro_export]
macro_rules! add_route {
    ($routes:expr, $method:expr, $path:expr, $func:expr) => {
        {
            let route = Route{method: $method, path: $path.to_string(), func: $func};
            $routes.add_route(route)
        }
    };
}
