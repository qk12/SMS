use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::terms::*;
use actix_web::web;
use diesel::prelude::*;
use diesel::sql_query;
use serde_json::json;

pub fn get_list(pool: web::Data<Pool>) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    let terms = sql_query("select * from terms order by id asc").load::<Term>(conn)?;

    let mut v: Vec<String> = Vec::new();
    for i in terms {
        v.push(i.term)
    }

    let res: serde_json::Value = json!({
        "res":{
            "terms":v,
            "nowTerm":v.last().unwrap()
        }
    });

    Ok(res)
}
