use actix_web::{rt, HttpRequest, HttpResponse, Responder, web};
use actix_ws::Message;
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;

pub async fn log_stream_ws(
    req: HttpRequest,
    body: web::Payload,
    path: web::Path<i32>
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
    let mut stream = msg_stream.aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));
    rt::spawn(async move{
        while let Some(msg) = stream.next().await{
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                },

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                },

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                },

                _ => {}
            }
        }
    });
    Ok(response)
}
