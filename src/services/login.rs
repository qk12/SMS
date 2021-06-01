use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::admin::*;
use crate::models::students::*;
use crate::models::teachers::*;
use actix_web::web;
use diesel::prelude::*;
use serde_json::json;

pub fn login(
    username: String,
    password: String,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    use crate::schema::student as student_schema;
    let try_student: Option<Student> = match student_schema::table
        .filter(student_schema::xh.eq(username.clone()))
        .first(conn)
    {
        Ok(student) => Some(student),
        _ => None,
    };

    use crate::schema::teacher as teacher_schema;
    let try_teacher: Option<Teacher> = match teacher_schema::table
        .filter(teacher_schema::gh.eq(username.clone()))
        .first(conn)
    {
        Ok(teacher) => Some(teacher),
        _ => None,
    };

    use crate::schema::admin as admin_schema;
    let try_admin: Option<Admin> = match admin_schema::table
        .filter(admin_schema::account.eq(username.clone()))
        .first(conn)
    {
        Ok(admin) => Some(admin),
        _ => None,
    };

    let res: serde_json::Value = if try_student.is_some() {
        json!({
            "res":{
                "role":"student",
                "userInfo":serde_json::to_value(try_student.unwrap()).unwrap()
            }
        })
    } else if try_teacher.is_some() {
        json!({
            "res":{
                "role":"teacher",
                "userInfo":serde_json::to_value(try_teacher.unwrap()).unwrap()
            }
        })
    } else {
        json!({
                "res":{
                    "role":"admin",
                    "userInfo":serde_json::to_value(try_admin.unwrap()).unwrap()
                }
        })
    };

    Ok(res)
}
