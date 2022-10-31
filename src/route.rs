use actix_web::{post, get, web, HttpRequest, HttpResponse, Responder};
use hyper::{Body, Client, Method, Request};
use rand::Rng;
use serde_json::{json, Value};
use crate::{Player, PLAYERS};
use crate::games::{Snake, SnakeGame, TetrisGame};


#[post("/set-player")]
pub async fn set_player_connected(req_body: String, _req: HttpRequest) -> impl Responder {
    let new_player: Player = serde_json::from_str(req_body.as_str()).unwrap();
    let mut mutex_clone = PLAYERS.lock().unwrap();

    match mutex_clone.iter_mut().find(|player| player.id == new_player.id) {
        Some(found_player) => {
            found_player.is_connected = new_player.is_connected;
        }
        None => {
            mutex_clone.push(new_player);
        }
    }
    HttpResponse::Ok()
}


#[get("/get-players-connected")]
pub async fn get_players_connected() -> impl Responder {
    HttpResponse::Ok().json(&*PLAYERS)
}


#[post("/tetris")]
pub async fn tetris(req_body: String, _req: HttpRequest) -> impl Responder {
    let mut tetris: TetrisGame = serde_json::from_str(req_body.as_str()).unwrap();


    tetris.game();

    HttpResponse::Ok().json(tetris)
}

#[post("/snake")]
pub async fn snake(req_body: String, _req: HttpRequest) -> impl Responder {
    let mut snakeGame: SnakeGame = serde_json::from_str(req_body.as_str()).unwrap();

    snakeGame.game();

    HttpResponse::Ok().json(tetris)
}

#[post("/player-ready")]
pub async fn player_ready(req_body: String, _req: HttpRequest) -> impl Responder {
    let player_ready: Value = serde_json::from_str(req_body.as_str()).unwrap();
    let mut mutex_clone = PLAYERS.lock().unwrap();
    match mutex_clone.iter_mut().find(|player| player.id == player_ready["id"]) {
        Some(found_player) => {
            found_player.game = player_ready["game"].to_string();
        }
        None => {}
    }
    HttpResponse::Ok()
}


pub async fn post_request(json: Value, url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("content-type", "application/json")
        .body(Body::from(json.to_string()))?;

    Client::new().request(req).await?;
    Ok(())
}