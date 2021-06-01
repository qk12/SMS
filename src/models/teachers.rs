use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "teacher"]
pub struct Teacher {
    pub gh: String,
    pub xm: Option<String>,
    pub xb: Option<String>,
    pub csrq: Option<NaiveDateTime>,
    pub zc: Option<String>,
    pub yxh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct TeacherInfo {
    pub gh: String,
    pub xm: Option<String>,
    pub xb: Option<String>,
    pub csrq: Option<NaiveDateTime>,
    pub zc: Option<String>,
    pub mc: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "teacher"]
pub struct TeacherForm {
    pub xm: Option<String>,
    pub xb: Option<String>,
    pub csrq: Option<NaiveDateTime>,
    pub zc: Option<String>,
    pub yxh: Option<String>,
}
