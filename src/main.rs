use actix_files::Files;
use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let data = Context::new();
    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn my_projects(tera: web::Data<Tera>) -> impl Responder {
    let data = Context::new();
    let rendered = tera.render("my_projects.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn posts(tera: web::Data<Tera>) -> impl Responder {
    let data = Context::new();
    let rendered = tera.render("posts.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn movies(tera: web::Data<Tera>) -> impl Responder {
    let data = Context::new();
    let rendered = tera.render("movies.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn games(tera: web::Data<Tera>) -> impl Responder {
    let data = Context::new();
    let rendered = tera.render("games.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn general(tera: web::Data<Tera>) -> impl Responder {
    let data = Context::new();
    let rendered = tera.render("general_posts.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

async fn login(tera: web::Data<Tera>) -> impl Responder {
    let data = Context::new();
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_login(data: web::Form<LoginUser>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Logged in: {}", data.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/", web::get().to(index))
            .route("/my_projects", web::get().to(my_projects))
            .route("/log_in", web::get().to(login))
            .route("/log_in", web::post().to(process_login))
            .route("/posts", web::get().to(posts))
            .route("/movies", web::get().to(movies))
            .route("/games", web::get().to(games))
            .route("/general_posts", web::get().to(general))
            .service(Files::new("/", "templates/").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
