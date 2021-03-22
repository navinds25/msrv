use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = String::from("192.168.1.1:5000");
    println!("starting server on address: {:?}", address);
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/", ".").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}

