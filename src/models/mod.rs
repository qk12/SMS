// 这里声明用于ORM的结构体

use crate::schema::*;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Student {
    xh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDate>,
    jg: Option<String>,
    sjhm: Option<String>,
    yxh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Teacher {
    gh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDate>,
    zc: Option<String>,
    yxh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "terms"]
pub struct Term {
    pub term: String,
    id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Openclass {
    xq: String,
    km: Option<String>,
    kh: String,
    sksj: Option<String>,
    xm: Option<String>,
    gh: String,
}
