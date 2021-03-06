use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::schema::xuanketable;
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn manage_grade(
    xh: String,
    xq: String,
    kh: String,
    gh: String,
    zpcj: i32,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let temp: Option<i32> = Some(zpcj);

    diesel::update(
        xuanketable::table.filter(
            xuanketable::xh
                .eq(xh.clone())
                .and(xuanketable::xq.eq(xq.clone()))
                .and(xuanketable::kh.eq(kh.clone()))
                .and(xuanketable::gh.eq(gh.clone())),
        ),
    )
    .set(xuanketable::zpcj.eq(temp.clone()))
    .execute(conn)?;

    let res = json!({
        "res":{ "message": "修改成功"}
    });

    Ok(res)
}
