extern crate hyper;
extern crate futures;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

struct HelloWorld;

const PHRASE: &'static str = "Hello, World!";

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body("Try POSTing data to /echo");
            },
            (&Method::Post, "/echo") => {
                // we'll be back
                response.set_body("Try POSTing data to /echo");
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        futures::future::ok(response)
        /*
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
            )
         */
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    //let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
