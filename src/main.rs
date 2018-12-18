extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate env_logger;

use actix_web::{fs, server, App};
use clap::App as Clap;
use clap::{Arg, ArgMatches};
use std::sync::Arc;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = Arc::new(get_config());
    let thread_config = config.clone();
    let port = config.value_of("port").unwrap();
    let address = format!("0.0.0.0:{}", port);

    let sys = actix::System::new("statik");

    server::new(move || {
        let directory = thread_config.value_of("directory").unwrap();
        let file = thread_config.value_of("file").unwrap();

        App::new().handler(
            "/",
            fs::StaticFiles::new(directory).unwrap().index_file(file),
        )
    })
    .bind(&address)
    .expect("Cannot start server on given IP/Port")
    .start();

    println!("Started http server: {}", &address);
    let _ = sys.run();
}

fn get_config() -> ArgMatches<'static> {
    Clap::new("Statik")
        .version("0.1.0")
        .author("Gabriel Taylor Russ <gabriel.taylor.russ@gmail.com>")
        .about("A simple static file server")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .required(true)
                .help("Sets the port of the web server")
                .env("PORT"),
        )
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .required(true)
                .help("Sets directory of the static files")
                .env("DIRECTORY"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Specifies the index file name")
                .env("FILE")
                .default_value("index.html"),
        )
        .get_matches()
}
