use actix::{Actor, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::time::{Duration};

use crate::redis_conn;

/// Define Websocket struct
struct MyWs;

impl MyWs {
    fn send_leaderboard_data(ctx: &mut <MyWs as Actor>::Context) {

        let leaderboard_data = redis_conn::get_leaderboard();
        // println!("LEADERBOARD:");
        // println!("{}", leaderboard_data);
        ctx.text(leaderboard_data);
    }
}


impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

     /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        
        // Send leaderboard data immidiately when a new connection is established.
        Self::send_leaderboard_data(ctx);

        // Send leaderboard data every 10 seconds.
        ctx.run_interval(Duration::new(10, 0), |_, ctx| {
            Self::send_leaderboard_data(ctx);
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn new_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    //println!("RESPONSE | {:?}", resp);
    resp
}

pub async fn fetch_leaderboard() -> impl Responder {

    let leaderboard_data = redis_conn::get_leaderboard();
    HttpResponse::Ok().body(leaderboard_data)
}
