use crate::prelude::*;
use std::{io::Write, net::TcpStream};

use crate::{
    request::{Request, RequestMethod},
    response::{Response, ResponseBuilder},
    route::{ActionRoute, DirectoryRoute, Route, RouteAction},
};

#[derive(Default)]
pub struct Router {
    routes: Vec<Box<dyn Route>>,
}

impl Router {
    pub fn new() -> Self {
        Router::default()
    }

    pub fn add_route(
        &mut self,
        route: &str,
        method: RequestMethod,
        action: RouteAction,
    ) -> &mut Self {
        let route = ActionRoute::new(route, method, action);
        let route = Box::new(route);
        self.routes.push(route);
        self
    }

    pub fn add_dir(&mut self, route: &str, method: RequestMethod, dir: &str) -> &mut Self {
        let route = DirectoryRoute::new(route, method, dir);
        let route = Box::new(route);
        self.routes.push(route);
        self
    }

    pub fn match_all(&self, url: &str, method: RequestMethod) -> Option<&dyn Route> {
        self.routes.iter().find_map(|route| {
            if method == route.method() && route.matches(url) {
                Some(route.as_ref())
            } else {
                None
            }
        })
    }

    pub fn handle_connection(&self, mut stream: TcpStream) -> Result<()> {
        println!("Accepted new connection from {}", stream.peer_addr()?);

        let request = Request::from_stream(&mut stream)?;

        let response: Response;
        if let Some(route) = self.match_all(&request.url, request.method) {
            response = match route.handle_request(&request) {
                Ok(ok_response) => ok_response,
                Err(e) => ResponseBuilder::new()
                    .status(400)
                    .status_message(&e.to_string())
                    .build()
                    .unwrap(),
            };
        } else {
            response = ResponseBuilder::new()
                .status(404)
                .status_message("Not Found")
                .build()?
        }

        stream.write_all(response.to_string().as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}
