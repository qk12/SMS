pub mod students;
pub mod teachers;
pub mod departments;
pub mod terms;
pub mod admins;
pub mod classes;

mod handler;
use actix_web::web;

use crate::controllers::handler::*;

// 声明请求方式和请求路径
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/openCourse").route(web::get().to(opencourse)))
            .service(web::resource("/openCourse").route(web::post().to(teacher_open_class)))
            .service(web::resource("/courseTable").route(web::get().to(course_table)))
            .service(web::resource("/reportCard").route(web::get().to(report_card)))
            .service(web::resource("/chooseCourse").route(web::post().to(choose_course)))
            .service(web::resource("/dropCourse").route(web::post().to(drop_course)))
            .service(web::resource("/manageGrade").route(web::post().to(manage_grade)))
    );
}