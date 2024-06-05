use goat_ws::{
    error::*,
    request::Request,
    response::{Response, ResponseBuilder},
    route::RouteParams,
};

pub fn user_agent(request: &Request, _params: RouteParams) -> Result<Response> {
    if let Some(header) = request.headers.iter().find(|h| h.0 == "User-Agent") {
        ResponseBuilder::new()
            .status(200)
            .status_message("OK")
            .header("Content-Type", "text/plain")
            .body(&header.1)
            .build()
    } else {
        ResponseBuilder::new()
            .status(400)
            .status_message("Missing User-Agent Header")
            .build()
    }
}
