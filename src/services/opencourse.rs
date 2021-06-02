use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use crate::schema::{class, openclass, teacher};
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn opencourse(term: String, pool: web::Data<Pool>) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let data = openclass::table
        .inner_join(teacher::table.on(openclass::gh.eq(teacher::gh)))
        .inner_join(class::table.on(openclass::kh.eq(class::kh)))
        .filter(openclass::xq.eq(term.clone()))
        .select((
            openclass::xq,
            class::km,
            openclass::kh,
            openclass::sksj,
            teacher::xm,
            teacher::gh,
            openclass::num,
        ))
        .load::<Openclass>(conn)?;

    let res: serde_json::Value = json!({
        "res" :serde_json::to_value(data).unwrap()
    });

    Ok(res)
}
