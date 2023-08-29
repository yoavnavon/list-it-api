//add the modules
mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::list::{create_list, delete_list, get_all_lists, get_list, update_list};
use api::user::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
            .service(create_list)
            .service(get_list)
            .service(update_list)
            .service(delete_list)
            .service(get_all_lists)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
