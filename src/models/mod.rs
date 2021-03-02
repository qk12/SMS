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