extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate tokio_uds;

use actix::*;
use actix_web::*;
use tokio_uds::UnixListener;


fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();
    let sys = actix::System::new("unix-socket");

    let listener = UnixListener::bind("/tmp/actix-uds.socket", Arbiter::handle()).expect("bind failed");
    let _addr = HttpServer::new(
        || Application::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/index.html", |r| r.f(|_| "Hello world!"))
            .resource("/", |r| r.f(index)))
        .start_incoming(listener.incoming(), false);

    println!("Started http server: /tmp/actix-uds.socket");
    let _ = sys.run();
}
