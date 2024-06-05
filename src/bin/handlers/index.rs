use goat_ws::error::*;

use goat_ws::{
    request::Request,
    response::{Response, ResponseBuilder},
    route::RouteParams,
};

pub fn index(_: &Request, _params: RouteParams) -> Result<Response> {
    ResponseBuilder::new()
        .status(200)
        .status_message("OK")
        .header("Content-Type", "text-plain")
        .body("abc")
        .build()
}
