// 这里写api接口

use crate::database::Pool;
use crate::errors::ServiceError;
use crate::services;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};

#[derive(Deserialize)]
pub struct LoginBody {
    username: String,
    password: String,
}

#[post("/api/login")]
pub async fn login(
    body: web::Json<LoginBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res =
        web::block(move || services::login(body.username.clone(), body.password.clone(), pool))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                e
            })?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/api/terms")]
pub async fn get_terms(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || services::get_terms(pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;
    Ok(HttpResponse::Ok().json(res))
}
