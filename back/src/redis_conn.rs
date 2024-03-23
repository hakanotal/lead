//use std::env;
use redis::Commands;
use serde::{Serialize};
use rand::Rng;
use serde_json::to_string;


pub fn connect() -> redis::Connection {
    println!("Connecting to Redis...");

    // //format - host:port
    // let redis_host_name = env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
    // let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    // //if Redis server needs secure connection
    // let uri_scheme = match env::var("IS_TLS") {
    //     Ok(_) => "rediss",
    //     Err(_) => "redis",
    // };

    // let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    
    redis::Client::open("redis://127.0.0.1/")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

pub fn init_leaderboard() {
    let mut conn = connect();

    println!("Initializing leaderboard in Redis...");
    let sorted_set = "leaderboard";

    let _: () = redis::cmd("ZADD")
        .arg(sorted_set)
        .arg(rand::thread_rng().gen_range(1..10))
        .arg("player-1")
        .query(&mut conn)
        .expect("failed to execute ZADD for 'leaderboard'");
        
    for num in 2..=5 {
        let _: () = conn
            .zadd(
                sorted_set,
                String::from("player-") + &num.to_string(),
                rand::thread_rng().gen_range(1..10),
            )
            .expect("failed to execute ZADD for 'leaderboard'");
    }

    let count: isize = conn
        .zcard(sorted_set)
        .expect("failed to execute ZCARD for 'leaderboard'");

    println!("PLAYER COUNT: {}", count);

    let leaderboard: Vec<(String, isize)> = conn
        .zrange_withscores(sorted_set, 0, count - 1)
        .expect("ZRANGE failed");

    println!("LEADERBOARD:");

    for item in &leaderboard {
        println!("{} : {}", item.0, item.1)
    }
}

// Define Leaderboard Entry struct
#[derive(Serialize)]
struct LeaderboardEntry {
    name: String,
    score: i32,
}

pub fn get_leaderboard() -> String {
    let mut conn = connect();

    let count: isize = conn
        .zcard("leaderboard")
        .expect("failed to execute ZCARD for 'leaderboard'");

    println!("PLAYER COUNT: {}", count);

    let leaderboard: Vec<(String, isize)> = conn
        .zrange_withscores("leaderboard", 0, count - 1)
        .expect("ZRANGE failed");

    // println!("LEADERBOARD:");

    // for item in &leaderboard {
    //     println!("{} : {}", item.0, item.1)
    // }

    // Convert raw leaderboard data into structured data
    let structured_leaderboard: Vec<LeaderboardEntry> = leaderboard
        .iter()
        .map(|(name, score)| LeaderboardEntry {
            name: name.clone(),
            score: *score as i32,
        })
        .collect();

    // Serialize the structured leaderboard data into JSON
    let json_leaderboard = to_string(&structured_leaderboard)
        .expect("failed to serialize leaderboard data");

    json_leaderboard
}