use crate::database::Pool;
use crate::errors::ServiceError;
use crate::services::term;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct CreateTermBody {
    term: String,
    id: i32,
}
/*
#[post("")]
pub async fn create(
    body: web::Json<CreateTermBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        term::create(
            body.term.clone(),
            body.id.clone(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}*/

#[get("")]
pub async fn get_list(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || term::get_list(pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;
    Ok(HttpResponse::Ok().json(res))
}
