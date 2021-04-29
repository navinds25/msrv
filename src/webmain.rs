use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn startsrv(address: String, _media_path: String) -> std::io::Result<()> {
    println!("starting server on address: {:?}", address);
    //let paths = fs::Files::new("/", media_path).show_files_listing();
    HttpServer::new(|| App::new().service(fs::Files::new("/", ".").show_files_listing()))
        .bind(address)?
        .run()
        .await
}
