use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde_json;
use shuttle_actix_web::ShuttleActixWeb;
use std::{fs, io};

#[derive(Deserialize)]
struct Jokes{
    jokes: Vec<String>
}

fn read_jokes_from_file() -> Result<Jokes, io::Error> {
    let data = fs::read_to_string("jokes.json")?;
    let jokes: Jokes = serde_json::from_str(&data)?;
    Ok(jokes)
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("hello rustaceans! 🦀")
}

#[get("/joke")]
async fn joke() -> impl Responder {
    match read_jokes_from_file() {
        Ok(jokes) => {
          if let Some(random_joke) = jokes.jokes.choose(&mut rand::thread_rng()) {
                HttpResponse::Ok()
                    .content_type("text/plain; charset=utf-8")
                    .insert_header(("Content-Type", "text/plain; charset=utf-8"))
                    .body(random_joke.to_string())
            } else {
                HttpResponse::InternalServerError().body("No jokes found")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error reading jokes file"),
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world).service(joke);
    };

    Ok(config.into())
}
