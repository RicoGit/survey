extern crate wither_derive;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use log::info;

use mongodb::Database;
use wither::prelude::*;

use crate::survey::Survey;

mod mongo;
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

    match Survey::find_one(db.get_ref(), None, None).await {
        Ok(survey) => HttpResponse::Ok().json(survey),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

// todo add methods getByName, getAll, removeByName methods for managing Survey

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    let db = crate::mongo::init().await?;

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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
