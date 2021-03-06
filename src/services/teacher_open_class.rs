use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use crate::schema::openclass;
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn teacher_open_class(
    kh: String,
    sksj: String,
    gh: String,
    xq: String,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    // 是否开过这门课
    let data: Option<KaiKe> = match openclass::table
        .filter(
            openclass::xq
                .eq(xq.clone())
                .and(openclass::kh.eq(kh.clone()))
                .and(openclass::gh.eq(gh.clone()))
                .and(openclass::sksj.eq(sksj.clone())),
        )
        .first(conn)
    {
        Ok(kaike) => Some(kaike),
        _ => None,
    };

    let temp: Option<String> = Some(sksj);

    if data.is_none() {
        let new_kaike = KaiKe {
            xq: xq.clone(),
            kh: kh.clone(),
            gh: gh.clone(),
            sksj: temp.clone(),
        };

        diesel::insert_into(openclass::table)
            .values(&new_kaike)
            .execute(conn)
            .unwrap();
    }

    let res: serde_json::Value = if data.is_none() {
        json!({
            "res":{"affectedRows": 1}
        })
    } else {
        json!({
            "res":{ "message": "你已经开过这门课了"}
        })
    };

    Ok(res)
}
