# HTTP Server Starter Rust

A lightweight, customizable HTTP server library for Rust, designed to help you get a web server up and running quickly. This library supports adding routes with various HTTP methods and serving files from directories with route matching.

## Features

- Add routes with custom handlers for different HTTP methods.
- Serve files from directories with route matching.
- Multi-threaded request handling using a thread pool.


## Example Usage

```rust

pub fn index(request: &Request, params: RouteParams) -> Result<Response> {
    ResponseBuilder::new()
        .status(200)
        .status_message("OK")
        .header("Content-Type", "text-plain")
        .body("It works!")
        .build()
}

pub fn hello(request: &Request, params: RouteParams) -> Result<Response> {
    let name = params.params.get("name").unwrap();
    // ...
}

fn main() {
    App::builder()
        .add_route("/", RequestMethod::GET, index)
        .add_route("/hello/{name}", RequestMethod::GET, hello) // {name} will be matched and passed as String
        .add_dir("/files/", RequestMethod::GET, DIR_PATH)
        .bind("127.0.0.1:4221")
        .expect("Failed to run app")
        .run();
}
```

## Installation

[dependencies]
goat_ws = "0.1.0"

## Routes

### Action Route

For performing actions

```rust
    .add_route("/", RequestMethod::GET, fn)
```

Params extraction

```rust
    .add_route("/{param1}/{param2}/hello", RequestMethod::GET, fn)
```

will pass param1 and param2 to fn as RouteParams

### Directory Route

For serving files

```rust
    .add_dir("/files", RequestMethod::GET, DIR)
```

/files/example/index.html will return contents of DIR/example/index.html

```rust
    .add_dir("/files", RequestMethod::POST, DIR)
```

/files/example/index.html will create DIR/example/index.html with contents in POST
