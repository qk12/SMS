use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::classes::*;
use crate::schema::class;
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn create(
    kh: String,
    km: Option<String>,
    xf: Option<i32>,
    xs: Option<i32>,
    yxh: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::insert_into(class::table)
        .values(&Class {
            kh: kh,
            km: km,
            xf: xf,
            xs: xs,
            yxh: yxh,
        })
        .execute(conn)?;

    Ok(())
}

pub fn get_list(pool: web::Data<Pool>) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let data = class::table.load::<Class>(conn)?;

    let res: serde_json::Value = json!({
        "res" :serde_json::to_value(data).unwrap()
    });

    Ok(res)
}

pub fn update(
    kh: String,
    new_km: Option<String>,
    new_xf: Option<i32>,
    new_xs: Option<i32>,
    new_yxh: Option<String>,
    pool: web::Data<Pool>,
) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::update(class::table.filter(class::kh.eq(kh)))
        .set(ClassForm {
            km: new_km,
            xf: new_xf,
            xs: new_xs,
            yxh: new_yxh,
        })
        .execute(conn)?;

    Ok(())
}

pub fn delete(kh: String, pool: web::Data<Pool>) -> ServiceResult<()> {
    let conn = &db_connection(&pool)?;

    diesel::delete(class::table.filter(class::kh.eq(kh))).execute(conn)?;

    Ok(())
}
