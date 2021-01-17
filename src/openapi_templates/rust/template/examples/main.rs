use {{snakecase info.title}}::server::*;
use actix_web::{HttpServer, App};

#[derive(Clone)]
struct Server;
impl {{camelcase info.title}} for Server {
    type Error = std::io::Error;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = Server {};
    HttpServer::new(move || App::new().data(server.clone()).configure(config::<Server>))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}