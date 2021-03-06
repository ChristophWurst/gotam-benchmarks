extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate primal;

use futures::future::ok;
use gotham::handler::HandlerFuture;
use gotham::http::response::create_response;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::State;
use hyper::StatusCode;

pub fn say_hello(state: State) -> Box<HandlerFuture> {
    let max = 10_000_000;
    let p = primal::Primes::all().nth(max).unwrap();

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            format!("The {}st prime is {}", max + 1, p).into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );

    Box::new(ok((state, res)))
}

fn router() -> Router {
    let (chain, pipelines) = single_pipeline(new_pipeline().build());

    build_router(chain, pipelines, |route| {
        route.get("/").to(say_hello);
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start_with_num_threads(addr, 12, router());
}
