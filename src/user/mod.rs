mod handler;
pub mod model;
pub(crate) mod service;
pub mod util;

use crate::user::handler::{login, logout, me, register};
use actix_web::web;

// 声明请求方式和请求路径，接受post方式请求/login路径
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").route(web::get().to(logout)))
            .service(web::resource("/me").route(web::get().to(me))),
    );
}

pub use util::has_role;
