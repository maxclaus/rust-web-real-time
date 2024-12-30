use actix_web::{
    http::{self},
    web::Bytes,
    HttpResponse,
};
use futures::{channel::mpsc, SinkExt};
use std::time::Duration;
use tokio::time::sleep;

pub async fn get_eventstream() -> HttpResponse {
    let buf_size = 10_usize;

    // Create a mpsc channel with a buffer size.
    // MPSC is a a multi-producer, single-consumer queue
    // for sending values across asynchronous tasks.
    let (mut tx, rx) = mpsc::channel::<Result<Bytes, actix_web::Error>>(buf_size);

    // Spawn an async task to send data, so it can return the response
    // while the stream still being written on a background thread.
    actix_web::rt::spawn(async move {
        let text = "This is the message we want to stream!";

        for word in text.split_ascii_whitespace() {
            // Send the first event
            let event = format!("data: {}\n\n", word);
            if let Err(err) = tx.send(Ok(Bytes::from(event))).await {
                log::error!("Error sending first event: {:?}", err);
                return;
            }
            sleep(Duration::from_secs(1)).await;
        }

        // Close the sender to communicate there will not be
        // anymore data being written to this sender.
        drop(tx);
    });

    // Create the streaming response with the proper content type header.
    HttpResponse::Ok()
        .insert_header((http::header::CONTENT_TYPE, "text/event-stream"))
        .streaming(rx)
}
