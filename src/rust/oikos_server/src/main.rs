pub mod server;

use actix_files as fs;
use actix_web::{
    error, guard,
    middleware::{Compress, Logger},
    web::{self, scope, JsonConfig},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use dotenv::dotenv;
use fs::NamedFile;
use std::{net::IpAddr, path::PathBuf};
use structopt::StructOpt;
use uom::si::{
    information::{byte, megabyte},
    usize::Information,
};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "p", long = "port", default_value = "8000", env = "PORT")]
    port: u16,
    #[structopt(long = "host", default_value = "127.0.0.1", env = "HOST")]
    host: IpAddr,
    #[structopt(
        long = "log_config_file",
        env = "LOG_CONFIG_FILE",
        default_value = "log4rs.yml"
    )]
    log_config_file: PathBuf,
    #[structopt(
        long = "app_path",
        env = "APP_PATH",
        default_value = "src/rust/oikos_web/static"
    )]
    app_path: String,
}

async fn index(front_path: web::Data<String>, _: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = [&front_path, "index.html"].iter().collect();
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args = Cli::from_args();
    log4rs::init_file(&args.log_config_file, Default::default())
        .expect("error during log initialization");
    let server = server::Server::new().await;
    let app_path = args.app_path.clone();
    HttpServer::new(move || {
        let mut app = App::new()
            .data(server.clone())
            .app_data(
                JsonConfig::default()
                    .limit(Information::new::<megabyte>(10).get::<byte>())
                    .error_handler(|err, _req| error::ErrorBadRequest(err)),
            )
            .service(scope("/api").configure(oikos_api::server::config::<server::Server>));

        app = app
            .data(app_path.clone())
            .service(
                fs::Files::new("/", app_path.clone())
                    .show_files_listing()
                    .index_file("index.html"),
            )
            .default_service(
                web::resource("").route(web::get().to(index)).route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
            );

        app.wrap(Logger::default()).wrap(Compress::default())
    })
    .bind(format!("{}:{}", args.host, args.port))?
    .run()
    .await
}
