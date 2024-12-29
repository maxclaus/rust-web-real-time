use std::sync::Mutex;

use actix::Actor;
use actix_cors::Cors;
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

async fn error_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let res = next.call(req).await?;

    if let Some(error) = res.response().error() {
        println!("Error in response: {:?}", error);
    }

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    // Create and spin up a lobby
    let chat_server = Data::new(Lobby::default().start());

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(from_fn(error_mw))
            .service(get_julia_image)
            .service(evenstream_handler::get_eventstream)
            .service(web::resource("/ws").route(web::get().to(julia_ws)))
            .service(ws_webrtc_handler::start_connection)
            .app_data(chat_server.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
