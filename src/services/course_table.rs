use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use crate::schema::{class, openclass, teacher, xuanketable};
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn get_student_course_table(
    xh: String,
    term: String,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let data = xuanketable::table
        .inner_join(
            openclass::table.on(xuanketable::kh
                .eq(openclass::kh)
                .and(xuanketable::gh.eq(openclass::gh))),
        )
        .inner_join(class::table.on(xuanketable::kh.eq(class::kh)))
        .inner_join(teacher::table.on(xuanketable::gh.eq(teacher::gh)))
        .filter(
            xuanketable::xh
                .eq(xh.clone())
                .and(xuanketable::xq.eq(term.clone())),
        )
        .select((
            xuanketable::xq,
            xuanketable::kh,
            class::km,
            openclass::sksj,
            teacher::xm,
            teacher::gh,
            openclass::num,
        ))
        .load::<StudentCourseTable>(conn)?;

    let res: serde_json::Value = json!({
        "res" :serde_json::to_value(data).unwrap()
    });

    Ok(res)
}

pub fn get_teacher_course_table(
    gh: String,
    term: String,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let data = openclass::table
        .inner_join(class::table.on(openclass::kh.eq(class::kh)))
        .filter(
            openclass::gh
                .eq(gh.clone())
                .and(openclass::xq.eq(term.clone())),
        )
        .select((openclass::kh, class::km, openclass::sksj))
        .load::<TeacherCourseTable>(conn)?;

    let res: serde_json::Value = json!({
        "res" :serde_json::to_value(data).unwrap()
    });

    Ok(res)
}
