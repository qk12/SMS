use crate::database::Pool;
use crate::errors::ServiceError;
use crate::services::department;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("")]
pub async fn get_list(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || department::get_list(pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;
    Ok(HttpResponse::Ok().json(res))
}
