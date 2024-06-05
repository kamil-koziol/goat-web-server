use std::net::TcpListener;
use std::sync::Arc;

use crate::prelude::*;

use crate::{request::RequestMethod, route::RouteAction, router::Router, thread_pool::ThreadPool};

pub struct AppBuilder {
    router: Router,
    threads: usize,
}

const DEFAULT_AMOUNT_OF_THREADS: usize = 4;

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            threads: DEFAULT_AMOUNT_OF_THREADS,
        }
    }

    pub fn add_route(mut self, path: &str, method: RequestMethod, handler: RouteAction) -> Self {
        self.router.add_route(path, method, handler);
        self
    }

    pub fn add_dir(mut self, path: &str, method: RequestMethod, dir: &str) -> Self {
        self.router.add_dir(path, method, dir);
        self
    }

    pub fn threads(mut self, threads: usize) -> Self {
        assert!(threads > 0);
        self.threads = threads;
        self
    }

    pub fn bind(self, address: &str) -> Result<App> {
        Ok(App {
            listener: TcpListener::bind(address)?,
            pool: ThreadPool::new(self.threads),
            router: Arc::new(self.router),
        })
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct App {
    listener: TcpListener,
    pool: ThreadPool,
    router: Arc<Router>,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn run(self) {
        println!("Server started at {}", self.listener.local_addr().unwrap());
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = Arc::clone(&self.router);
                    self.pool.execute(move || {
                        let _ = router.handle_connection(stream);
                    })
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
