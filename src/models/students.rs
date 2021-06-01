use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "student"]
pub struct Student {
    pub xh: String,
    pub xm: Option<String>,
    pub xb: Option<String>,
    pub csrq: Option<NaiveDateTime>,
    pub jg: Option<String>,
    pub sjhm: Option<String>,
    pub yxh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct StudentInfo {
    pub xh: String,
    pub xm: Option<String>,
    pub xb: Option<String>,
    pub csrq: Option<NaiveDateTime>,
    pub jg: Option<String>,
    pub sjhm: Option<String>,
    pub mc: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "student"]
pub struct StudentForm {
    pub xm: Option<String>,
    pub xb: Option<String>,
    pub csrq: Option<NaiveDateTime>,
    pub jg: Option<String>,
    pub sjhm: Option<String>,
    pub yxh: Option<String>,
}
