use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use crate::schema::{class, student, xuanketable};
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn get_student_report_card(
    xh: String,
    term: String,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let data = xuanketable::table
        .inner_join(class::table.on(xuanketable::kh.eq(class::kh)))
        .filter(
            xuanketable::xh
                .eq(xh.clone())
                .and(xuanketable::xq.eq(term.clone())),
        )
        .select((
            xuanketable::kh,
            class::km,
            xuanketable::zpcj,
            xuanketable::grade,
        ))
        .load::<StudentReportCard>(conn)?;

    let res: serde_json::Value = json!({
        "res" :serde_json::to_value(data).unwrap()
    });

    Ok(res)
}

pub fn get_teacher_report_card(
    gh: String,
    term: String,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let data = xuanketable::table
        .inner_join(class::table.on(xuanketable::kh.eq(class::kh)))
        .inner_join(student::table.on(xuanketable::xh.eq(student::xh)))
        .filter(
            xuanketable::gh
                .eq(gh.clone())
                .and(xuanketable::xq.eq(term.clone())),
        )
        .select((
            student::xm,
            xuanketable::xh,
            xuanketable::kh,
            class::km,
            xuanketable::zpcj,
            xuanketable::grade,
        ))
        .load::<TeacherReportCard>(conn)?;

    let res: serde_json::Value = json!({
        "res" :serde_json::to_value(data).unwrap()
    });

    Ok(res)
}
