use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::{list::List, user::User};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId, to_bson},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    users: Collection<User>,
    lists: Collection<List>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let users: Collection<User> = db.collection("User");
        let lists: Collection<List> = db.collection("List");
        MongoRepo { users, lists }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            _id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .users
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn create_list(&self, new_list: List) -> Result<InsertOneResult, Error> {
        let new_doc = List {
            _id: None,
            name: new_list.name,
            description: new_list.description,
            items: new_list.items,
        };
        let list = self
            .lists
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating list");
        Ok(list)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .users
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn get_list(&self, id: &String) -> Result<List, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let list_detail = self
            .lists
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting list's detail");
        Ok(list_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title,
                },
        };
        let updated_doc = self
            .users
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
    pub async fn update_list(&self, id: &String, new_list: List) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "name": new_list.name,
                    "description": new_list.description,
                    "items": to_bson(&new_list.items).unwrap()
                },
        };
        let updated_doc = self
            .lists
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating list");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .users
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }
    pub async fn delete_list(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let list_detail = self
            .lists
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting list");
        Ok(list_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .users
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }
    pub async fn get_all_lists(&self) -> Result<Vec<List>, Error> {
        let mut cursors = self
            .lists
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of lists");
        let mut lists: Vec<List> = Vec::new();
        while let Some(list) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            lists.push(list)
        }
        Ok(lists)
    }
}
