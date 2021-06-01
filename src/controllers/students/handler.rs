use crate::database::Pool;
use crate::errors::ServiceError;
use crate::services::student;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct CreateStudentBody {
    xh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDateTime>,
    jg: Option<String>,
    sjhm: Option<String>,
    yxh: String,
}

#[post("")]
pub async fn create(
    body: web::Json<CreateStudentBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        student::create(
            body.xh.clone(),
            body.xm.clone(),
            body.xb.clone(),
            body.csrq.clone(),
            body.jg.clone(),
            body.sjhm.clone(),
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
    let res = web::block(move || student::get_list(pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct UpdateStudentBody {
    new_xm: Option<String>,
    new_xb: Option<String>,
    new_csrq: Option<NaiveDateTime>,
    new_jg: Option<String>,
    new_sjhm: Option<String>,
    new_yxh: Option<String>,
}

#[put("/{xh}")]
pub async fn update(
    web::Path(xh): web::Path<String>,
    body: web::Json<UpdateStudentBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        student::update(
            xh,
            body.new_xm.clone(),
            body.new_xb.clone(),
            body.new_csrq.clone(),
            body.new_jg.clone(),
            body.new_sjhm.clone(),
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

#[delete("/{xh}")]
pub async fn delete(
    web::Path(xh): web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || student::delete(xh, pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

    Ok(HttpResponse::Ok().json(&res))
}
