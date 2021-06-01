use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::statics::*;
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn set(now_term: Option<String>, score_limit: Option<i32>) -> ServiceResult<()> {
    if now_term.is_some() {
        unsafe {
            NOW_TERM = now_term.unwrap();

            eprintln!("NOW_TERM:{}", NOW_TERM.clone());
        }
    }

    if score_limit.is_some() {
        unsafe {
            SCORE_LIMIT = score_limit.unwrap();

            eprintln!("SCORE_LIMIT:{}", SCORE_LIMIT.clone());
        }
    }

    Ok(())
}

pub fn get_now_term() -> ServiceResult<serde_json::Value> {
    unsafe {
        let res: serde_json::Value = json!({
            "res" :serde_json::to_value(NOW_TERM.clone()).unwrap()
        });

        Ok(res)
    }
}

pub fn get_score_limit() -> ServiceResult<serde_json::Value> {
    unsafe {
        let res: serde_json::Value = json!({
            "res" :serde_json::to_value(SCORE_LIMIT.clone()).unwrap()
        });

        Ok(res)
    }
}
