// 这里写数据查询和处理逻辑

use crate::errors::{ ServiceResult, ServiceError };
use crate::database::{ db_connection, Pool };
use actix_web::web;
use diesel::prelude::*;
use crate::models::*;

pub fn login(
    userName: String,
    password: String,
    pool: web::Data<Pool>,
) -> ServiceResult<Student> {
    let conn = &db_connection(&pool)?;

    use crate::schema::student as student_schema;
    let student: Student = student_schema::table.filter(student_schema::xh.eq(userName)).first(conn)?;

    Ok(student)
}