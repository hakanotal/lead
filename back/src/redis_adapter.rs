use redis::Commands;
use serde::Serialize;
use rand::Rng;
use lazy_static::lazy_static;
use std::sync::Mutex;
use serde_json::json;

// Initialize Redis connection
lazy_static! {
    static ref REDIS_CONNECTION: Mutex<redis::Connection> = {
        println!("Connecting to Redis...");

        let conn = redis::Client::open("redis://127.0.0.1/")
            .expect("Invalid connection URL")
            .get_connection()
            .expect("failed to connect to Redis");
        
        Mutex::new(conn)
    };
}

// Define Leaderboard Entry struct
#[derive(Serialize)]
struct LeaderboardEntry {
    name: String,
    score: i32,
}

// Initialize the leaderboard in Redis
pub fn init_leaderboard() {
    let mut conn = REDIS_CONNECTION.lock().unwrap();

    println!("Initializing leaderboard in Redis...");
    let sorted_set = "leaderboard";

    let _: () = redis::cmd("ZADD")
        .arg(sorted_set)
        .arg(rand::thread_rng().gen_range(1..10))
        .arg("player-1")
        .query(&mut *conn)
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


// Get the leaderboard
pub fn get_leaderboard(start: isize, end: isize) -> String {
    let mut conn = REDIS_CONNECTION.lock().unwrap();

    let leaderboard: Vec<(String, isize)> = conn
        .zrevrange_withscores("leaderboard", start, end - 1)
        .expect("ZREVRANGE failed");

    // Create a JSON array of leaderboard entries
    let json_leaderboard = json!(leaderboard
        .iter()
        .map(|(name, score)| {
            json!({
                "name": name,
                "score": score
            })
        })
        .collect::<Vec<_>>());

    json_leaderboard.to_string()
}

// Get the count of players in the leaderboard
pub fn get_leaderboard_count() -> String {
    let mut conn = REDIS_CONNECTION.lock().unwrap();

    let count: isize = conn
        .zcard("leaderboard")
        .expect("failed to execute ZCARD for 'leaderboard'");

    println!("PLAYER COUNT: {}", count);

    count.to_string()
}

// Add a player to the leaderboard
pub fn insert_player(name: &str, score: i32) {
    let mut conn = REDIS_CONNECTION.lock().unwrap();

    let _: () = conn
        .zadd("leaderboard", name, score)
        .expect("failed to execute ZADD for 'leaderboard'");
}

// Remove a player from the leaderboard
pub fn remove_player(name: &str) {
    let mut conn = REDIS_CONNECTION.lock().unwrap();

    let _: () = conn
        .zrem("leaderboard", name)
        .expect("failed to execute ZREM for 'leaderboard'");
}

// Update a player's score in the leaderboard
pub fn update_player(name: &str, score: i32) {
    let mut conn = REDIS_CONNECTION.lock().unwrap();

    let _: () = conn
        .zadd("leaderboard", name, score)
        .expect("failed to execute ZADD for 'leaderboard'");
}