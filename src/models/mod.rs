// 这里声明用于ORM的结构体

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
