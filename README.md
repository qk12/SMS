# SMS

学生管理系统

create `.env` according to `.env.example`

then excute command in your shell `diesel migration run`
(you need to intall `diesel_cli` by running `cargo install diesel_cli` first)

* API
```js
- POST /api/login       data: { userName, password }

- GET /api/terms

- GET /api/classes

// 当前学期的开课表
- GET /api/openCourse   data: { params: { term } }

// 教师开设课程
- POST /api/openCourse   data: { xq, kh, gh, sksj }

// xh,gh 选传一个
- GET /api/courseTable  data: { params: { xh, term, gh }}

- GET /api/students 

// xh,gh 选传一个
// gh 表示这个老师某个学期所教学生的成绩单
- GET /api/reportCard   data: { params: { xh, term, gh }}

- POST /api/chooseCourse    data: { xh, xq, kh, gh }

- POST /api/dropCourse    data: { xh, xq, kh, gh }

- POST /api/manageGrade     data: { xh, kh, pscj, kscj, zpcj }
```