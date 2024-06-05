use crate::prelude::*;
use std::fs;

#[derive(Default, Clone)]
pub struct ResponseBuilder {
    status: u32,
    status_message: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(&mut self, status: usize) -> &mut Self {
        self.status = status as u32;
        self
    }

    pub fn status_message(&mut self, status_message: &str) -> &mut Self {
        self.status_message = status_message.into();
        self
    }
    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn body(&mut self, body: &str) -> &mut Self {
        self.headers
            .push(("Content-Length".into(), body.len().to_string()));
        self.body = Some(body.into());
        self
    }

    pub fn build(&self) -> Result<Response> {
        let status = self.status;
        let status_message = self.status_message.clone();
        let headers = self.headers.clone();
        let body = self.body.clone();

        Ok(Response {
            status,
            status_message,
            headers,
            body,
        })
    }
}

#[derive(Debug, Default)]
pub struct Response {
    pub status: u32,
    pub status_message: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl Response {
    pub fn from_file(file_path: &str) -> Result<Response> {
        if let Ok(content) = fs::read_to_string(file_path) {
            ResponseBuilder::new()
                .status(200)
                .status_message("OK")
                .header("Content-Type", "application/octet-stream")
                .body(&content)
                .build()
        } else {
            ResponseBuilder::new()
                .status(404)
                .status_message("Not Found")
                .build()
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response = String::new();
        response += format!("HTTP/1.1 {} {}\r\n", self.status, self.status_message).as_str();

        for header in &self.headers {
            response += format!("{}: {}\r\n", header.0, header.1).as_str();
        }

        response += "\r\n";

        if let Some(body) = &self.body {
            response += body;
        }

        response
    }
}
