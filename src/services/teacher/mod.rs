use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::teachers::*;
use crate::schema::{department, teacher};
use actix_web::web;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json::json;

pub fn create(
    gh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDateTime>,
    zc: Option<String>,
    yxh: String,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::insert_into(teacher::table)
        .values(&Teacher {
            gh: gh,
            xm: xm,
            xb: xb,
            csrq: csrq,
            zc: zc,
            yxh: yxh,
        })
        .execute(conn)?;

    Ok(())
}

pub fn get_list(pool: web::Data<Pool>) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let data = teacher::table
        .inner_join(department::table.on(teacher::yxh.eq(department::yxh)))
        .select((
            teacher::gh,
            teacher::xm,
            teacher::xb,
            teacher::csrq,
            teacher::zc,
            department::mc,
        ))
        .load::<TeacherInfo>(conn)?;

    let res: serde_json::Value = json!({
        "res" :serde_json::to_value(data).unwrap()
    });

    Ok(res)
}

pub fn update(
    gh: String,
    new_xm: Option<String>,
    new_xb: Option<String>,
    new_csrq: Option<NaiveDateTime>,
    new_zc: Option<String>,
    new_yxh: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::update(teacher::table.filter(teacher::gh.eq(gh)))
        .set(TeacherForm {
            xm: new_xm,
            xb: new_xb,
            csrq: new_csrq,
            zc: new_zc,
            yxh: new_yxh,
        })
        .execute(conn)?;

    Ok(())
}

pub fn delete(gh: String, pool: web::Data<Pool>) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::delete(teacher::table.filter(teacher::gh.eq(gh))).execute(conn)?;

    Ok(())
}
