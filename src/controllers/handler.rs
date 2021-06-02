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

#[derive(Deserialize)]
pub struct OpenClassParams {
    term: String,
}

pub async fn opencourse(
    query: web::Query<OpenClassParams>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || services::opencourse(query.term.clone(), pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct KaikeBody {
    kh: String,
    sksj: String,
    gh: String,
    xq: String,
}

pub async fn teacher_open_class(
    body: web::Json<KaikeBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        services::teacher_open_class(
            body.kh.clone(),
            body.sksj.clone(),
            body.gh.clone(),
            body.xq.clone(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct CourseTableParams {
    xh: Option<String>,
    gh: Option<String>,
    term: String,
}

pub async fn course_table(
    query: web::Query<CourseTableParams>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if query.xh.is_some() {
        let res = web::block(move || {
            services::get_student_course_table(query.xh.clone().unwrap(), query.term.clone(), pool)
        })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

        return Ok(HttpResponse::Ok().json(res));
    }
    let res = web::block(move || {
        services::get_teacher_course_table(query.gh.clone().unwrap(), query.term.clone(), pool)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct ReportCardParams {
    xh: Option<String>,
    gh: Option<String>,
    term: String,
}

pub async fn report_card(
    query: web::Query<ReportCardParams>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if query.xh.is_some() {
        let res = web::block(move || {
            services::get_student_report_card(query.xh.clone().unwrap(), query.term.clone(), pool)
        })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            e
        })?;

        return Ok(HttpResponse::Ok().json(res));
    }
    let res = web::block(move || {
        services::get_teacher_report_card(query.gh.clone().unwrap(), query.term.clone(), pool)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct CourseBody {
    xh: String,
    xq: String,
    kh: String,
    gh: String,
}

pub async fn choose_course(
    body: web::Json<CourseBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        services::choose_course(
            body.xh.clone(),
            body.xq.clone(),
            body.kh.clone(),
            body.gh.clone(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn drop_course(
    body: web::Json<CourseBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        services::drop_course(
            body.xh.clone(),
            body.xq.clone(),
            body.kh.clone(),
            body.gh.clone(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize)]
pub struct ManageCourseBody {
    xh: String,
    xq: String,
    kh: String,
    gh: String,
    zpcj: Option<i32>,
}

pub async fn manage_grade(
    body: web::Json<ManageCourseBody>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        services::manage_grade(
            body.xh.clone(),
            body.xq.clone(),
            body.kh.clone(),
            body.gh.clone(),
            body.zpcj.clone().unwrap(),
            pool,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        e
    })?;
    Ok(HttpResponse::Ok().json(res))
}
