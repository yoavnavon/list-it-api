use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListItem {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub items: Vec<ListItem>,
}
