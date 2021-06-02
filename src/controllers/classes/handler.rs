use crate::database::Pool;
use crate::errors::ServiceError;
use crate::services::class;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct CreateClassBody {
    pub kh: String,
    pub km: Option<String>,
    pub xf: Option<i32>,
    pub xs: Option<i32>,
    pub yxh: Option<String>,
}

#[post("")]
pub async fn create(
    body: web::Json<CreateClassBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        class::create(
            body.kh.clone(),
            body.km.clone(),
            body.xf.clone(),
            body.xs.clone(),
            body.yxh.clone(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[get("")]
pub async fn get_list(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || class::get_list(pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct UpdateClassBody {
    new_km: Option<String>,
    new_xf: Option<i32>,
    new_xs: Option<i32>,
    new_yxh: Option<String>,
}

#[put("/{kh}")]
pub async fn update(
    web::Path(kh): web::Path<String>,
    body: web::Json<UpdateClassBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        class::update(
            kh,
            body.new_km.clone(),
            body.new_xf.clone(),
            body.new_xs.clone(),
            body.new_yxh.clone(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(&res))
}

#[delete("/{kh}")]
pub async fn delete(
    web::Path(kh): web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || class::delete(kh, pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}
