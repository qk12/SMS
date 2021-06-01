use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "terms"]
pub struct Term {
    pub term: String,
    id: i32,
}