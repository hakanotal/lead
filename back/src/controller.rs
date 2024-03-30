use actix::{Actor, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::time::{Duration};

use crate::redis_adapter;

// Define Websocket struct
struct MyWs;

impl MyWs {
    fn send_leaderboard_data(ctx: &mut <MyWs as Actor>::Context) {
        let leaderboard_data = redis_adapter::get_leaderboard(0, 5);
        // println!("LEADERBOARD:");
        // println!("{}", leaderboard_data);
        ctx.text(leaderboard_data);
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    // Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        
        // Send leaderboard data immidiately when a new connection is established.
        Self::send_leaderboard_data(ctx);

        // Send leaderboard data every 10 seconds.
        ctx.run_interval(Duration::new(10, 0), |_, ctx| {
            Self::send_leaderboard_data(ctx);
        });
    }
}

// Websocket stream handler
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

// New websocket connection is established
pub async fn new_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    //println!("RESPONSE | {:?}", resp);
    resp
}

// Controller functions
pub async fn fetch_top_leaderboard() -> impl Responder {
    let leaderboard_data = redis_adapter::get_leaderboard(0, 5);
    HttpResponse::Ok().body(leaderboard_data)
}

pub async fn fetch_leaderboard(req: HttpRequest) -> impl Responder {
    let page = req.match_info().get("page").unwrap_or("1").parse::<i32>().unwrap_or(1);
    let leaderboard_data = redis_adapter::get_leaderboard(((page-1)*5) as isize, (page*5) as isize);
    HttpResponse::Ok().body(leaderboard_data)
}

pub async fn fetch_leaderboard_count() -> impl Responder {
    let leaderboard_count = redis_adapter::get_leaderboard_count();
    HttpResponse::Ok().body(leaderboard_count)
}

pub async fn delete_player(req: HttpRequest) -> impl Responder {
    let player_name = req.match_info().get("name").unwrap_or("player");

    redis_adapter::remove_player(player_name);

    HttpResponse::Ok().body(format!("Player {} deleted", player_name))
}

pub async fn add_player(req: HttpRequest) -> impl Responder {
    let player_name = req.match_info().get("name").unwrap_or("player");
    let player_score = req.match_info().get("score").unwrap_or("0").parse::<i32>().unwrap_or(0);

    redis_adapter::insert_player(player_name, player_score);

    HttpResponse::Ok().body(format!("Player {} added with score {}", player_name, player_score))
}

pub async fn update_player(req: HttpRequest) -> impl Responder {
    let player_name = req.match_info().get("name").unwrap_or("player");
    let player_score = req.match_info().get("score").unwrap_or("0").parse::<i32>().unwrap_or(0);

    redis_adapter::update_player(player_name, player_score);

    HttpResponse::Ok().body(format!("Player {} updated with score {}", player_name, player_score))
}

