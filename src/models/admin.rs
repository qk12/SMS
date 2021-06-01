use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Admin {
    account: String,
    password: Option<String>,
}