use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use crate::schema::class;
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn get_classes(pool: web::Data<Pool>) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let classes: Vec<Class> = class::table.load::<Class>(conn)?;

    let mut v: Vec<SlimClass> = Vec::new();
    for it in classes {
        v.push(SlimClass::from(it));
    }

    let res: serde_json::Value = json!({ "res": v });

    Ok(res)
}
