use std::{env, fs};

use actix::Actor;
use actix_cors::Cors;
use actix_files::Files;
use actix_rt;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Logger, Next};
use actix_web::web::Data;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};

mod evenstream_handler;
mod julia;
mod ws;
mod ws_handler;
mod ws_webrtc_handler;

use julia::{julia_generate, JuliaParams};

use env_logger::Env;
use serde::Deserialize;
use serde::Serialize;
use ws::lobby::Lobby;

#[derive(Serialize)]
struct ApiData {
    status: String,
    data: Vec<u8>,
}
#[derive(Debug, Deserialize)]
pub enum ResponseType {
    Token,
    Code,
}

async fn julia_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that
    // the response is returned immediately
    actix_rt::spawn(ws_handler::julia_ws(session, msg_stream));

    Ok(res)
}

#[get("/julia-image")]
async fn get_julia_image(query: web::Query<JuliaParams>) -> Result<impl Responder> {
    let data = julia_generate(&query.into_inner());

    let obj = ApiData {
        status: "healthy".to_string(),
        data,
    };
    Ok(web::Json(obj))
}

async fn index() -> impl Responder {
    let html_content = fs::read_to_string("app/dist/index.html")
        .unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());

    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

async fn error_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let res = next.call(req).await?;

    if let Some(error) = res.response().error() {
        log::error!("Error in response: {:?}", error);
    }

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug logs
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let server_port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    // Create and spin up a lobby
    let chat_server = Data::new(Lobby::default().start());

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(from_fn(error_mw))
            // API routes
            .service(get_julia_image)
            .service(web::resource("/api/ws/julia").route(web::get().to(julia_ws)))
            .service(
                web::resource("/api/eventstream")
                    .route(web::get().to(evenstream_handler::get_eventstream)),
            )
            .service(
                web::resource("api/ws/videochat/{group_id}")
                    .route(web::get().to(ws_webrtc_handler::start_connection)),
            )
            // HTML routes
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/eventstream").route(web::get().to(index)))
            .service(web::resource("/videochat").route(web::get().to(index)))
            // Static files
            .service(Files::new("/", "./app/dist"))
            .app_data(chat_server.clone())
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}
