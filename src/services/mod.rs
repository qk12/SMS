// 这里写数据查询和处理逻辑
pub mod student;
pub mod teacher;
pub mod department;
pub mod term;
pub mod admin;

mod login;
pub(crate) use login::login;

mod get_classes;
pub(crate) use get_classes::get_classes;

mod opencourse;
pub(crate) use opencourse::opencourse;

mod course_table;
pub(crate) use course_table::{get_student_course_table, get_teacher_course_table};

mod report_card;
pub(crate) use report_card::{get_student_report_card, get_teacher_report_card};

mod choose_course;
pub(crate) use choose_course::choose_course;

mod drop_course;
pub(crate) use drop_course::drop_course;

mod manage_grade;
pub(crate) use manage_grade::manage_grade;

mod teacher_open_class;
pub(crate) use teacher_open_class::teacher_open_class;
