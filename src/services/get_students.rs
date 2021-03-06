use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use crate::schema::{department, student};
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn get_students(pool: web::Data<Pool>) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let data = student::table
        .inner_join(department::table.on(student::yxh.eq(department::yxh)))
        .select((
            student::xh,
            student::xm,
            student::xb,
            student::csrq,
            student::jg,
            student::sjhm,
            department::mc,
        ))
        .load::<StudentInfo>(conn)?;

    let res: serde_json::Value = json!({
        "res" :serde_json::to_value(data).unwrap()
    });

    Ok(res)
}
