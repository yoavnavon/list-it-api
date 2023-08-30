use crate::{
    models::list::{List, ListItem},
    repository::mongodb::MongoRepo,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

use mongodb::bson::oid::ObjectId;

#[post("/list")]
pub async fn create_list(db: Data<MongoRepo>, new_list: Json<List>) -> HttpResponse {
    let data = List {
        _id: None,
        name: new_list.name.to_owned(),
        description: new_list.description.to_owned(),
        items: new_list.items.to_owned(),
    };
    let list_detail = db.create_list(data).await;
    match list_detail {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list/{id}")]
pub async fn get_list(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let list_detail = db.get_list(&id).await;
    match list_detail {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/list/{id}")]
pub async fn update_list(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_list: Json<List>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = List {
        _id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_list.name.to_owned(),
        description: new_list.description.to_owned(),
        items: new_list.items.to_owned(),
    };
    let update_result = db.update_list(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_list_info = db.get_list(&id).await;
                return match updated_list_info {
                    Ok(list) => HttpResponse::Ok().json(list),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No list found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/list/{id}")]
pub async fn delete_list(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_list(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("List successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("List with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/lists")]
pub async fn get_all_lists(db: Data<MongoRepo>) -> HttpResponse {
    let lists = db.get_all_lists().await;
    match lists {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
