use std::net::TcpStream;
use std::io::{Read, Write};

use crate::Response;

use super::response::{IntoResponse, StatusCode};

#[derive(PartialEq, Debug, Clone, Copy)]
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
where T: Clone + IntoResponse
{
    pub method: Method,
    pub path: String,
    pub func: Box<dyn Fn() -> T>,
}

impl<T>  Route<T>
where T: Clone + IntoResponse
{
    fn new(method: Method, path: String, func: impl Fn() -> T + 'static)  -> Self
    {
        Self {
            method,
            path,
            func: Box::new(func)
        }
    }
}

impl<T> AnyRoute for Route<T>
where T: Clone + IntoResponse
{

    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_method(&self) -> Method {
        self.method
    }

    fn execute(&self) -> Response {
        (self.func)().into_response().clone()
    }

}


trait AnyRoute {

    fn get_path(&self) -> &String;
    fn get_method(&self) -> Method;
    fn execute(&self) -> Response;
}


pub struct Routes
{
    pub routes : Vec<Box<dyn AnyRoute>>
}

#[derive(Debug)]
pub enum RouteError {
    AlreadyInUse,
    DoesNotExist,
    CouldNotProcess

}

impl Routes
{
    pub fn new() -> Self {
        Self {
            routes: vec!(),
        }
    }

    pub fn add_route<T: Clone + IntoResponse + 'static>(&mut self, method: Method, path: String, func: impl Fn() -> T + 'static) -> Result<(), RouteError>
    {

        let route = Route::new(method, path, func);

        if let Some(_) = self.routes.iter().find(|r| *r.get_path() == route.path && r.get_method() == route.method) {
                Err(RouteError::AlreadyInUse)
        } else {
            self.routes.push(Box::new(route));
            Ok(())
        }
    }

    pub fn process_request(&self, stream: &mut TcpStream,path: String, method: &str) -> Result<(), RouteError> {
        let method: Method = Method::from(method);

        if let Some(route) = self.routes.iter().find(|r| *r.get_path() == path && r.get_method() == method) {

            let res = route.execute();
            let str : String = (res).into();
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
