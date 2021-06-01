use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Department {
    yxh: String,
    mc: Option<String>,
    dz: Option<String>,
    lxdh: Option<String>,
}
