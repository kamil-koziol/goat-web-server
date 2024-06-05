use goat_ws::error::*;

use goat_ws::{
    request::Request,
    response::{Response, ResponseBuilder},
    route::RouteParams,
};

pub fn echo(_request: &Request, _params: RouteParams) -> Result<Response> {
    let msg = _params.params.get("msg").unwrap();
    ResponseBuilder::new()
        .status(200)
        .status_message("OK")
        .header("Content-Type", "text/plain")
        .body(msg)
        .build()
}
