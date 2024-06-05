use crate::prelude::*;

use crate::{
    request::{Request, RequestMethod},
    response::Response,
};

use super::{Route, RouteParams};

pub type RouteAction = fn(request: &Request, params: RouteParams) -> Result<Response>;

pub struct ActionRoute {
    pub url: String,
    pub action: RouteAction,
    pub method: RequestMethod,
}

impl ActionRoute {
    pub fn new(url: &str, method: RequestMethod, action: RouteAction) -> Self {
        Self {
            url: url.into(),
            method,
            action,
        }
    }
    pub fn extract_params(&self, url: &str) -> RouteParams {
        let url_paths: Vec<_> = self.url.split('/').collect();
        let pattern_paths: Vec<_> = url.split('/').collect();

        let mut params = RouteParams::new();
        for (up, pp) in url_paths.iter().zip(pattern_paths) {
            if up.starts_with('{') && up.ends_with('}') {
                let param_name = &up[1..up.len() - 1];
                params
                    .params
                    .insert(String::from(param_name), String::from(pp));
                continue;
            }
        }

        params
    }
}

impl Route for ActionRoute {
    fn url(&self) -> &str {
        &self.url
    }

    fn matches(&self, pattern: &str) -> bool {
        let url_paths: Vec<_> = self.url.split('/').collect();
        let pattern_paths: Vec<_> = pattern.split('/').collect();

        if url_paths.len() != pattern_paths.len() {
            return false;
        }

        for (up, pp) in url_paths.iter().zip(pattern_paths) {
            if up.starts_with('{') && up.ends_with('}') {
                continue;
            }

            if *up != pp {
                return false;
            }
        }

        true
    }

    fn handle_request(&self, request: &Request) -> Result<Response> {
        let params = self.extract_params(&request.url);
        (self.action)(request, params)
    }

    fn method(&self) -> RequestMethod {
        self.method
    }
}
