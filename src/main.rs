extern crate hyper;
extern crate futures;

use hyper::server::{Http, Request, Response, Service};

use futures::Stream;
use futures::Future;

use futures::future::{Either, Map};
use futures::stream::Concat2;
use hyper::Chunk;
use hyper::{Method, StatusCode};

fn reverse(chunk: Chunk) -> Response {
    let reversed = chunk.iter()
        //.rev()
        .cloned()
        .collect::<Vec<u8>>();
    Response::new()
        .with_body(reversed)
}

struct Echo;
impl Service for Echo {
    type Request = Request;
    type Error = hyper::Error;

    type Future = Either<
        futures::future::FutureResult<Self::Response, Self::Error>,
        Map<Concat2<hyper::Body>, fn(Chunk) -> Self::Response>
    >;
    type Response = Response;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                Either::A(
                    futures::future::ok(
                        Response::new().with_body("Try POSTing data to /echo")
                ))
            },
            (&Method::Post, "/echo") => {
                Either::B(
                    req.body()
                        .concat2()
                        .map(reverse)
                )
            },
            _ => {
                Either::A(futures::future::ok(
                        Response::new().with_status(StatusCode::NotFound)
                ))
            },
        }
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
