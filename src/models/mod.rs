// 这里声明用于ORM的结构体

use crate::schema::*;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Student {
    xh: String,
    xm: String,
    xb: String,
    csrq: Option<NaiveDate>,
    jg: Option<String>,
    sjhm: String,
    yxh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Teacher {
    gh: String,
    xm: String,
    xb: String,
    csrq: String,
    zc: Option<String>,
    yxh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "terms"]
pub struct Term {
    pub term: String,
    id: i32,
}
