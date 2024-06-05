use crate::prelude::*;

use std::{fs::File, io::Write, path::Path};

use crate::{
    request::{Request, RequestMethod},
    response::{Response, ResponseBuilder},
};

use super::Route;

pub struct DirectoryRoute {
    pub url: String,
    pub dir: String,
    pub method: RequestMethod,
}

impl DirectoryRoute {
    pub fn new(url: &str, method: RequestMethod, dir: &str) -> Self {
        DirectoryRoute {
            url: url.into(),
            dir: dir.into(),
            method,
        }
    }

    pub fn extract_file_path(&self, url: &str) -> String {
        String::from(&url[self.url.len()..])
    }
}

impl Route for DirectoryRoute {
    fn url(&self) -> &str {
        &self.url
    }

    fn matches(&self, pattern: &str) -> bool {
        pattern.starts_with(&self.url)
    }

    fn handle_request(&self, request: &Request) -> Result<Response> {
        let requested_path = self.extract_file_path(&request.url);
        let file_path = Path::new(&self.dir).join(requested_path);
        let file_path = file_path.to_str().unwrap();

        match &self.method {
            RequestMethod::GET => {
                Response::from_file(file_path).map_err(|_| "Couldn't read from file".into())
            }
            RequestMethod::POST => {
                let mut f = File::create(file_path)?;
                if let Some(contents) = &request.body {
                    f.write_all(contents.as_bytes())?;
                }
                Ok(ResponseBuilder::new()
                    .status(201)
                    .status_message("Created")
                    .build()
                    .unwrap())
            }

            _ => {
                todo!();
            }
        }
    }

    fn method(&self) -> crate::request::RequestMethod {
        self.method
    }
}
