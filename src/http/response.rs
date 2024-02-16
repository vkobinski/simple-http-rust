use std::any::type_name;

use serde_json::Result;

trait ContentTrait {
    fn get_content_type(&self) -> String;
    fn content_length(&self) -> usize;
    fn to_string(&self) -> String;
}

struct Content<T: ToString> {
    content: T,
}

impl<T: ToString> ContentTrait for Content<T> {
    fn get_content_type(&self) -> String {
        println!("{}", type_name::<T>());
        match type_name::<T>() {
            "serde_json::value::Value" => String::from("application/json"),
            "alloc::string::String" => String::from("text/plain"),
            "&str" => String::from("text/plain"),
            _ => "".to_string(),
        }
    }

    fn content_length(&self) -> usize {
        self.content.to_string().len()
    }

    fn to_string(&self) -> String {
        self.content.to_string()
    }

}

impl <T: ToString> Content<T> {
    fn new(c: T) -> Self {
        Self { content: c }
    }
}

impl<T: ToString> Into<String> for Content<T> {
    fn into(self) -> String {
        self.content.to_string()
    }

}

#[derive(Debug)]
pub struct StatusCode {
    code: usize,
    message: &'static str,
}

impl StatusCode {
    pub const OK: StatusCode = StatusCode { code: 200, message: "Ok" };
    pub const NOT_FOUND: StatusCode = StatusCode { code: 404, message: "Not Found" };
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode { code: 500, message: "Internal Server Error" };
}

pub struct Response<'a> {
    code: StatusCode,
    content: Box<dyn ContentTrait + 'a>,
}

impl<'a> Response<'a> {
    pub fn new<T: ToString + 'a >(status: StatusCode, content: T) -> Self {

        Self {
            code: status,
            content: Box::new(Content::new(content)),
        }
    }
}

impl Into<String> for Response<'_> {

    fn into(self) -> String {

        let mut response_str = String::new();

        let content_type = self.content.get_content_type();
        let content_length = self.content.content_length();
        let content = &self.content.to_string();

        response_str.push_str(format!("HTTP/1.1 {} {}\r\n", self.code.code, self.code.message).as_str());
        response_str.push_str(format!("Content-Length: {}\r\n", content_length).as_str());
        response_str.push_str(format!("Content-Type: {}\r\n", content_type).as_str());
        response_str.push_str("Server: Kobe");
        response_str.push_str("\r\n\r\n");
        response_str.push_str(content);

        println!("{}", response_str);

        response_str
    }
}

pub trait IntoResponse {
    fn into_response(&self) -> Response;
}

impl  IntoResponse for String {
    fn into_response(&self) -> Response {
        Response::new(StatusCode::OK, self)
    }
}

impl IntoResponse for Vec<String> {

    fn into_response(&self) -> Response {

        let mut res = String::new();

        self.iter()
            .for_each(|el| res.push_str(format!("{}\n",el.to_string()).as_str()));

        Response::new(StatusCode::OK, res)
    }

}


impl IntoResponse for Vec<&str> {

    fn into_response(&self) -> Response {

        let mut res = String::new();

        self.iter()
            .for_each(|el| res.push_str(format!("{}\n",el.to_string()).as_str()));

        Response::new(StatusCode::OK, res)
    }

}

//impl<T> IntoResponse for Result<T, > {
//
//    fn into_response(&self) -> Response {
//    }
//}

impl IntoResponse for serde_json::value::Value {
    fn into_response(&self) -> Response {
        Response::new(StatusCode::NOT_FOUND, self.to_string())
    }
}
