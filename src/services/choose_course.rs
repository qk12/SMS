use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::*;
use crate::schema::{class, openclass, xuanketable};
use crate::statics::*;
use actix_web::web;
use diesel::dsl::sum;
use diesel::prelude::*;
use diesel::sql_query;
use serde_json::json;

pub fn choose_course(
    xh: String,
    xq: String,
    kh: String,
    gh: String,
    pool: web::Data<Pool>,
) -> ServiceResult<serde_json::Value> {
    let conn = &db_connection(&pool)?;

    // 是否开设
    let data: Option<KaiKe> = match openclass::table
        .filter(
            openclass::xq
                .eq(xq.clone())
                .and(openclass::kh.eq(kh.clone()))
                .and(openclass::gh.eq(gh.clone())),
        )
        .first(conn)
    {
        Ok(kaike) => Some(kaike),
        _ => None,
    };

    if data.is_none() {
        let res: serde_json::Value = json!({
            "res":{ "message": "未开设该门课"}
        });
        return Ok(res);
    }

    // 是否超学分
    let data = match xuanketable::table
        .inner_join(class::table.on(xuanketable::kh.eq(class::kh)))
        .filter(xuanketable::xh.eq(xh.clone()))
        .select(sum(class::xf))
        .first::<Option<i64>>(conn)
    {
        Ok(temp) => Some(temp),
        _ => None,
    };

    let temp = class::table
        .filter(class::kh.eq(kh.clone()))
        .select(class::xf)
        .first::<Option<i32>>(conn)
        .unwrap();

    eprintln!("temp: {}", temp.clone().unwrap());

    if data.is_some() {
        let temp2 = data.unwrap();
        if temp2.is_some() {
            eprintln!("temp2: {}", temp2.clone().unwrap());

            unsafe {
                if (temp2.unwrap() as i32 + temp.unwrap()) > SCORE_LIMIT {
                    let res: serde_json::Value = json!({
                        "res":{ "message": "超出学分"}
                    });
                    return Ok(res);
                }
            }
        }
    }

    //是否选过
    let data: Option<ChooseCourse> = match xuanketable::table
        .filter(
            xuanketable::xh
                .eq(xh.clone())
                .and(xuanketable::xq.eq(xq.clone()))
                .and(xuanketable::kh.eq(kh.clone()))
                .and(xuanketable::gh.eq(gh.clone())),
        )
        .first(conn)
    {
        Ok(choosecourse) => Some(choosecourse),
        _ => None,
    };

    let temp: Option<i32> = None;
    let temp2: Option<f32> = None;

    if data.is_none() {
        let new_choosecourse = ChooseCourse {
            xh: xh.clone(),
            xq: xq.clone(),
            kh: kh.clone(),
            gh: gh.clone(),
            zpcj: temp,
            grade: temp2,
        };

        diesel::insert_into(xuanketable::table)
            .values(&new_choosecourse)
            .execute(conn)
            .unwrap();

        sql_query(format!("call change_num('{}', '{}', '{}', 1)", xq, kh, gh))
            .execute(conn)
            .unwrap();
    }

    let res: serde_json::Value = if data.is_none() {
        json!({
            "res":{"affectedRows": 1}
        })
    } else {
        json!({
            "res":{ "message": "已经存在这条记录了"}
        })
    };

    Ok(res)
}
