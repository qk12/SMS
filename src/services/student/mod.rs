use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::students::*;
use crate::schema::{department, student};
use actix_web::web;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json::json;

pub fn create(
    xh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDateTime>,
    jg: Option<String>,
    sjhm: Option<String>,
    yxh: String,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::insert_into(student::table)
        .values(&Student {
            xh: xh,
            xm: xm,
            xb: xb,
            csrq: csrq,
            jg: jg,
            sjhm: sjhm,
            yxh: yxh,
        })
        .execute(conn)?;

    Ok(())
}

pub fn get_list(pool: web::Data<Pool>) -> ServiceResult<serde_json::Value> {
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

pub fn update(
    xh: String,
    new_xm: Option<String>,
    new_xb: Option<String>,
    new_csrq: Option<NaiveDateTime>,
    new_jg: Option<String>,
    new_sjhm: Option<String>,
    new_yxh: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::update(student::table.filter(student::xh.eq(xh)))
        .set(StudentForm {
            xm: new_xm,
            xb: new_xb,
            csrq: new_csrq,
            jg: new_jg,
            sjhm: new_sjhm,
            yxh: new_yxh,
        })
        .execute(conn)?;

    Ok(())
}

pub fn delete(xh: String, pool: web::Data<Pool>) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::delete(student::table.filter(student::xh.eq(xh))).execute(conn)?;

    Ok(())
}
