// 这里写数据查询和处理逻辑

use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use actix_web::web;
use diesel::prelude::*;
use diesel::sql_query;
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
    /*
    let res: serde_json::Value = if try_student.is_some() {
        serde_json::to_value(try_student.unwrap()).unwrap()
    } else {
        serde_json::to_value(try_teacher.unwrap()).unwrap()
    };*/

    let res: serde_json::Value = if try_student.is_some() {
        json!({
            "res":{
                "role":"student",
                "userInfo":serde_json::to_value(try_student.unwrap()).unwrap()
            }
        })
    } else {
        json!({
            "res":{
                "role":"teacher",
                "userInfo":serde_json::to_value(try_teacher.unwrap()).unwrap()
            }
        })
    };

    Ok(res)
}

pub fn get_terms(pool: web::Data<Pool>) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    //use crate::schema::term as term_schema;
    //let terms: Vec<Term> = term_schema::table.load::<Term>(conn)?;

    let terms = sql_query("select * from terms order by id asc").load::<Term>(conn)?;

    let mut v: Vec<String> = Vec::new();
    for i in terms {
        v.push(i.term)
    }

    let res: serde_json::Value = json!({
        "res":{
            "terms":v,
            "nowTerm":v.last().unwrap()
        }
    });

    Ok(res)
}
