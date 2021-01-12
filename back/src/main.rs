extern crate wither_derive;

use actix_cors::Cors;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use wither::mongodb::bson::doc;

use futures::stream::StreamExt;
use wither::bson::oid::ObjectId;

use log::info;
use wither::mongodb::{Client, Database};
use wither::prelude::*;

use crate::survey::Survey;

mod survey;

#[post("/survey")]
async fn create_survey(new_survey: web::Json<Survey>, db: web::Data<Database>) -> impl Responder {
    info!("new survey: {:?}", new_survey);

    match new_survey.into_inner().save(db.get_ref(), None).await {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[get("/survey/{id}")]
async fn get_survey(id: web::Path<String>, db: web::Data<Database>) -> impl Responder {
    info!("get survey: {:?}", id.0);

    match ObjectId::with_string(&id.0) {
        Ok(id) => match Survey::find_one(db.get_ref(), doc! { "_id": id }, None).await {
            Ok(survey) => HttpResponse::Ok().json(survey),
            Err(_) => HttpResponse::InternalServerError().json("internal error"),
        },
        Err(_) => HttpResponse::BadRequest().json("Invalid id"),
    }
}

#[get("/surveys")]
async fn get_survey_list(db: web::Data<Database>) -> impl Responder {
    info!("get survey list");

    match Survey::find(db.get_ref(), None, None).await {
        Ok(cursor) => {
            let vec: Vec<_> = StreamExt::collect(cursor).await;
            let vec: Vec<Survey> = vec
                .into_iter()
                .map(|res| res.expect("Get list failed"))
                .collect();
            HttpResponse::Ok().json(vec)
        }
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[delete("/survey/{id}")]
async fn delete_survey(id: web::Path<String>, db: web::Data<Database>) -> impl Responder {
    info!("delete survey: {:?}", id.0);

    match ObjectId::with_string(&id.0) {
        Ok(id) => match Survey::find_one_and_delete(db.get_ref(), doc! { "_id": id }, None).await {
            Ok(survey) => HttpResponse::Ok().json(survey),
            Err(_) => HttpResponse::InternalServerError().json(""),
        },
        Err(_) => HttpResponse::BadRequest().json("Invalid id"),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    let db = init().await?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .data(db.clone())
            .service(create_survey)
            .service(get_survey)
            .service(get_survey_list)
            .service(delete_survey)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

/// Connects db and sends check request
pub async fn init() -> Result<Database> {
    let db = Client::with_uri_str("mongodb://localhost:27017/")
        .await?
        .database("survey");

    // Ping the server to see if you can connect to the cluster
    db.run_command(doc! {"ping": 1}, None).await?;

    info!("MongoDb was connected successfully.");

    Ok(db)
}
