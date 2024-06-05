use crate::prelude::*;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::str::FromStr;

#[derive(Default, Clone)]
pub struct RequestBuilder {
    method: Option<RequestMethod>,
    url: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}
impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn method(&mut self, method: &str) -> Result<&mut Self> {
        let method = RequestMethod::from_str(method)?;
        let _ = self.method.insert(method);
        Ok(self)
    }

    pub fn url(&mut self, url: &str) -> &mut Self {
        let _ = self.url.insert(url.into());
        self
    }

    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn body(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(&self) -> Result<Request> {
        let method = self.method.ok_or("Method not set")?;
        let url = self.url.clone().ok_or("URL not set")?;
        let headers = self.headers.clone();
        let body = self.body.clone();

        Ok(Request {
            method,
            url,
            headers,
            body,
        })
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl FromStr for RequestMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "GET" => Ok(RequestMethod::GET),
            "POST" => Ok(RequestMethod::POST),
            "PUT" => Ok(RequestMethod::PUT),
            "PATCH" => Ok(RequestMethod::PATCH),
            "DELETE" => Ok(RequestMethod::DELETE),
            _ => Err("Cannot parse".into()),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: RequestMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Request> {
        // GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n
        let mut reader = BufReader::new(stream);
        let mut http_request: Vec<String> = vec![];

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }

            if line == "\r\n" {
                break;
            }

            http_request.push(line);
        }

        let mut request_builder = RequestBuilder::new();

        let request_line: Vec<&str> = http_request[0].split_whitespace().collect();
        let method: String = request_line[0].into();

        request_builder.method(&method)?;

        let url: String = request_line[1].into();

        request_builder.url(&url);

        let _version: String = request_line[2].into();

        let headers = &http_request[1..];
        for header in headers.iter() {
            let mut nameval = header.split(": ");
            let name = nameval.next().unwrap();
            let value = nameval.next().unwrap();

            request_builder.header(name, value.trim_end());
        }

        if let Some(content_length) = request_builder
            .headers
            .iter()
            .find(|h| h.0 == "Content-Length")
        {
            let length: usize = content_length.1.parse()?;
            if length != 0 {
                let mut body = vec![0; length];
                let _ = reader.read_exact(&mut body);
                let body = String::from_utf8(body)?;
                request_builder.body = Some(body);
            }
        }

        let request = request_builder.build()?;
        Ok(request)
    }
}
