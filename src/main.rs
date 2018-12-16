extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate clap;

use std::sync::Arc;
use clap::Arg;
use clap::App as Clap;
use actix_web::{fs, middleware, server, App};

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let matches = Clap::new("Statik")
                      .version("0.1.0")
                      .author("Gabriel Taylor Russ <gabriel.taylor.russ@gmail.com>")
                      .about("A simple static file server")
                      .arg(Arg::with_name("port")
                           .short("p")
                           .long("port")
                           .required(true)
                           .help("Sets the port of the web server")
                           .env("PORT"))
                      .arg(Arg::with_name("directory")
                           .short("d")
                           .long("directory")
                           .required(true)
                           .help("Sets directory of the static files")
                           .env("DIRECTORY"))
                      .arg(Arg::with_name("file")
                           .short("f")
                           .long("file")
                           .help("Specifies the index file name")
                           .env("FILE")
                           .default_value("index.html"))
                      .get_matches();

    let matches = Arc::new(matches);
    let thread_matches = matches.clone();
    let port = matches.value_of("port").unwrap();
    let address = format!("127.0.0.1:{}", port);

    let sys = actix::System::new("statik");

    server::new(move || {
        let directory = thread_matches.value_of("directory").unwrap();
        let file = thread_matches.value_of("file").unwrap();

        App::new()
            // enable logger
            .middleware(middleware::Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %Dms"#))
            .handler(
                "/",
                fs::StaticFiles::new(directory).unwrap().index_file(file)
            )
    }).bind(&address)
        .expect("Cannot start server on given IP/Port")
        .start();

    println!("Started http server: {}", &address);
    let _ = sys.run();
}
