#![allow(missing_docs)]

extern crate firecracker_api;
extern crate iron;
extern crate futures;
extern crate clap;
extern crate swagger;

use clap::{App, Arg};
use iron::{Iron, Chain};
use swagger::auth::AllowAllMiddleware;

// Import the module that defines the Server struct.
mod server_lib;

/// Create custom server, wire it to the autogenerated router,
/// and pass it to the web server.
fn main() {
    let matches = App::new("server")
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .get_matches();

    let server = server_lib::Server{};
    let router = firecracker_api::router(server);

    let mut chain = Chain::new(router);
    chain.link_before(firecracker_api::server::ExtractAuthData);
    // add authentication middlewares into the chain here
    // for the purpose of this example, pretend we have authenticated a user
    chain.link_before(AllowAllMiddleware::new("cosmo"));

    if matches.is_present("https") {
        // Using Simple HTTPS
    } else {
        // Using HTTP
        Iron::new(chain).http("localhost:8080").expect("Failed to start HTTP server");
    }
}