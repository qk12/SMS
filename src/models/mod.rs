// 这里声明用于ORM的结构体

use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Student {
    xh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDateTime>,
    jg: Option<String>,
    sjhm: Option<String>,
    yxh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct StudentInfo {
    xh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDateTime>,
    jg: Option<String>,
    sjhm: Option<String>,
    mc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Teacher {
    gh: String,
    xm: Option<String>,
    xb: Option<String>,
    csrq: Option<NaiveDateTime>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct StudentCourseTable {
    xq: String,
    kh: String,
    km: Option<String>,
    sksj: Option<String>,
    xm: Option<String>,
    gh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct TeacherCourseTable {
    kh: String,
    km: Option<String>,
    sksj: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct StudentReportCard {
    kh: String,
    km: Option<String>,
    zpcj: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct TeacherReportCard {
    xm: Option<String>,
    xh: String,
    kh: String,
    km: Option<String>,
    zpcj: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "openclass"]
pub struct KaiKe {
    pub xq: String,
    pub kh: String,
    pub gh: String,
    pub sksj: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "xuanketable"]
pub struct ChooseCourse {
    pub xh: String,
    pub xq: String,
    pub kh: String,
    pub gh: String,
    pub zpcj: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Class {
    kh: String,
    km: Option<String>,
    xf: Option<i32>,
    xs: Option<i32>,
    yxh: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimClass {
    kh: String,
    km: Option<String>,
}

impl From<Class> for SlimClass {
    fn from(raw: Class) -> Self {
        Self {
            kh: raw.kh,
            km: raw.km,
        }
    }
}
