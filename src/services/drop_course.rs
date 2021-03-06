use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use crate::schema::xuanketable;
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn drop_course(
    xh: String,
    xq: String,
    kh: String,
    gh: String,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    //是否选过
    let data: Option<ChooseCourse> = match xuanketable::table
        .filter(
            xuanketable::xh
                .eq(xh.clone())
                .and(xuanketable::xq.eq(xq.clone()))
                .and(xuanketable::kh.eq(kh.clone()))
                .and(xuanketable::gh.eq(gh.clone())),
        )
        .first(conn)
    {
        Ok(choosecourse) => Some(choosecourse),
        _ => None,
    };

    if data.is_some() {
        diesel::delete(
            xuanketable::table.filter(
                xuanketable::xh
                    .eq(xh.clone())
                    .and(xuanketable::xq.eq(xq.clone()))
                    .and(xuanketable::kh.eq(kh.clone()))
                    .and(xuanketable::gh.eq(gh.clone())),
            ),
        )
        .execute(conn)
        .unwrap();
    }

    let res: serde_json::Value = if data.is_some() {
        json!({
            "res":{"affectedRows": 1}
        })
    } else {
        json!({
            "res":{ "message": "你没有这条选课记录"}
        })
    };

    Ok(res)
}
