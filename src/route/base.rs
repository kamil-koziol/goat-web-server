use crate::prelude::*;

use std::collections::HashMap;

use crate::{
    request::{Request, RequestMethod},
    response::Response,
};

#[derive(Default, Debug)]
pub struct RouteParams {
    pub params: HashMap<String, String>,
}

impl RouteParams {
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait Route: Send + Sync {
    fn url(&self) -> &str;
    fn matches(&self, pattern: &str) -> bool;
    fn handle_request(&self, request: &Request) -> Result<Response>;
    fn method(&self) -> RequestMethod;
}
