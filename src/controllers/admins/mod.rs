pub mod handler;

use actix_web::web;

// 声明请求方式和请求路径
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admins")
            .service(handler::set)
            .service(handler::get_now_term)
            .service(handler::get_score_limit),
    );
}
