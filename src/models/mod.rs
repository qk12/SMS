// 这里声明用于ORM的结构体
pub mod admin;
pub mod departments;
pub mod students;
pub mod teachers;
pub mod terms;
pub mod classes;

use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Openclass {
    xq: String,
    km: Option<String>,
    kh: String,
    sksj: Option<String>,
    xm: Option<String>,
    gh: String,
    num: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct StudentCourseTable {
    xq: String,
    kh: String,
    km: Option<String>,
    sksj: Option<String>,
    xm: Option<String>,
    gh: String,
    num: Option<i32>,
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
    grade: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct TeacherReportCard {
    xm: Option<String>,
    xh: String,
    kh: String,
    km: Option<String>,
    zpcj: Option<i32>,
    grade: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, QueryableByName)]
#[table_name = "openclass"]
pub struct KaiKe {
    pub xq: String,
    pub kh: String,
    pub gh: String,
    pub sksj: Option<String>,
    pub num: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "xuanketable"]
pub struct ChooseCourse {
    pub xh: String,
    pub xq: String,
    pub kh: String,
    pub gh: String,
    pub zpcj: Option<i32>,
    pub grade: Option<f32>,
}
