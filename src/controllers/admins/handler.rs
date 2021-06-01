use crate::database::Pool;
use crate::errors::ServiceError;
use crate::services::admin;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct CreateSetBody {
    now_term: Option<String>,
    score_limit: Option<i32>,
}

#[put("/set")]
pub async fn set(
    body: web::Json<CreateSetBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || admin::set(body.now_term.clone(), body.score_limit.clone()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[get("/now_term")]
pub async fn get_now_term() -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || admin::get_now_term())
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[get("/score_limit")]
pub async fn get_score_limit() -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || admin::get_score_limit())
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}
