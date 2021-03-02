// 这里写api接口

use actix_web::{web, HttpResponse, HttpRequest, get, post, put, delete};
use crate::errors::ServiceError;
use crate::database::Pool;
use crate::services;

#[derive(Deserialize)]
pub struct LoginBody {
    userName: String,
    password: String,
}

#[post("/api/login")]
pub async fn login(
    body: web::Json<LoginBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || services::login(
        body.userName.clone(), 
        body.password.clone(), 
        pool
    )).await.map_err(|e| {
        eprintln!("{}", e);
        e
    })?;
    
    Ok(HttpResponse::Ok().json(res))
}