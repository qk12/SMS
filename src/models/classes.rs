use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "class"]
pub struct Class {
    pub kh: String,
    pub km: Option<String>,
    pub xf: Option<i32>,
    pub xs: Option<i32>,
    pub yxh: Option<String>,
}

#[derive(AsChangeset)]
#[table_name = "class"]
pub struct ClassForm {
    pub km: Option<String>,
    pub xf: Option<i32>,
    pub xs: Option<i32>,
    pub yxh: Option<String>,
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
