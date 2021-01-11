use crate::survey::NewSurvey;
use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use mongodb::Client;

mod mongo;
mod survey;

#[post("/survey")]
async fn create_survey(new_survey: web::Json<NewSurvey>, db: web::Data<Client>) -> impl Responder {
    println!("new survey: {:?}", new_survey);
    match mongo::create_survey(db.get_ref(), &new_survey).await {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

// todo add methods getByName, getAll, removeByName methods for managing Survey

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    let client = crate::mongo::init().await?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .data(client.clone())
            .service(create_survey)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
