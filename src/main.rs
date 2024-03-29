#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#[allow(clippy::unused_async)]
mod handlers;
mod image;

use std::path::PathBuf;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use clap::Parser;
use handlers::{hello, images, random};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the folder that should be served
    #[clap(short, long, default_value = "./")]
    image_path: String,

    /// Set a custom port number
    #[clap(short, long, default_value_t = 8080)]
    port: u16,
}

pub struct WebContext {
    path: PathBuf,
    index: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let logger = Logger::default();
        // Load up the index site in memory so we don't have to access it constantly
        let index = include_str!("../index.html");

        let web_context = Data::new(WebContext {
            path: PathBuf::from(shellexpand::tilde(&args.image_path).into_owned()),
            index: index.to_string(),
        });

        App::new()
            .app_data(web_context)
            .wrap(logger)
            .service(hello)
            .service(random)
            .service(images)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
