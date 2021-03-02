#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod cli_args;
mod database;
mod errors;
mod schema;
mod controllers;
mod models;
mod services;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Gets enviroment variables from `.env`
    dotenv::dotenv().ok();

    // Initiates error logger
    env_logger::init();

    // Sets options to enviroment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    // Database
    let pool = database::pool::establish_connection(opt.clone());

    // Authorisation
    let domain = opt.domain.clone();
    let cookie_secret_key = opt.auth_secret_key.clone();
    let secure_cookie = opt.secure_cookie;
    let auth_duration = time::Duration::hours(i64::from(opt.auth_duration_in_hour));

    // Server port
    let port = opt.port;

    // Server
    let server = HttpServer::new(move || {

        App::new()
            // Database
            .data(pool.clone())
            // Options
            .data(opt.clone())
            // Error logging
            .wrap(Logger::default())
            // Authorisation
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(&domain)
                    // Time from creation that cookie remains valid
                    .max_age_time(auth_duration)
                    // Restricted to https?
                    .secure(secure_cookie),
            ))
            // Sets routes via secondary files
            .service(controllers::login)
    })
    // Running at `format!("{}:{}",port,"0.0.0.0")`
    .bind(("127.0.0.1", port))
    .unwrap()
    // Starts server
    .run();

    eprintln!("Listening on 127.0.0.1:{}", port);

    // Awaiting server to exit
    server.await
}
