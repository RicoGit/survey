use crate::survey::NewSurvey;
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod survey;

#[post("/survey")]
async fn create_survey(new_survey: web::Json<NewSurvey>) -> impl Responder {
    // todo create new surver to mongo db
    println!("{:?}", new_survey);
    HttpResponse::Created()
}

// todo add methods getByName, getAll, removeByName methods for managing Survey

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new().wrap(cors).service(hello).service(create_survey)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
