use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn actix_main() -> std::io::Result<()> {
    let address = String::from("0.0.0.0:5000");
    println!("starting server on address: {:?}", address);
    HttpServer::new(|| {
        App::new()
            .service(hello)
    }).bind(address)?.run().await
}

fn main() {
    println!("Hello, world!");
    actix_main().unwrap();
}

