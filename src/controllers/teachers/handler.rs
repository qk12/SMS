use crate::database::Pool;
use crate::errors::ServiceError;
use crate::services::teacher;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct CreateTeacherBody {
    gh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDateTime>,
    zc: Option<String>,
    yxh: String,
}

#[post("")]
pub async fn create(
    body: web::Json<CreateTeacherBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        teacher::create(
            body.gh.clone(),
            body.xm.clone(),
            body.xb.clone(),
            body.csrq.clone(),
            body.zc.clone(),
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
    let res = web::block(move || teacher::get_list(pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct UpdateTeacherBody {
    new_xm: Option<String>,
    new_xb: Option<String>,
    new_csrq: Option<NaiveDateTime>,
    new_zc: Option<String>,
    new_yxh: Option<String>,
}

#[put("/{gh}")]
pub async fn update(
    web::Path(gh): web::Path<String>,
    body: web::Json<UpdateTeacherBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        teacher::update(
            gh,
            body.new_xm.clone(),
            body.new_xb.clone(),
            body.new_csrq.clone(),
            body.new_zc.clone(),
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

#[delete("/{gh}")]
pub async fn delete(
    web::Path(gh): web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || teacher::delete(gh, pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}
