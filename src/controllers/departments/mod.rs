pub mod handler;

use actix_web::web;

// 声明请求方式和请求路径
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/departments").service(handler::get_list));
}
