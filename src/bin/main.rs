extern crate goat_ws;
pub use goat_ws::{request::RequestMethod, App};
pub mod handlers;

const DIR_PATH: &str = "/tmp/goat_ws/files";

fn main() {
    App::builder()
        .add_route("/", RequestMethod::GET, handlers::index)
        .add_route("/echo/{msg}", RequestMethod::GET, handlers::echo)
        .add_route("/user-agent", RequestMethod::GET, handlers::user_agent)
        .add_dir("/files/", RequestMethod::GET, DIR_PATH)
        .add_dir("/files/", RequestMethod::POST, DIR_PATH)
        .bind("127.0.0.1:4221")
        .expect("Failed to run app")
        .run();
}
