use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "item_type", rename_all = "lowercase")]
pub enum Content {
    Text { text: String },
    Image { url: String },
    List { list: List },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListItem {
    pub content: Content,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct List {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub items: Vec<ListItem>,
}
