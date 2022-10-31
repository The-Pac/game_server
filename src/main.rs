mod route;
mod games;

use actix_web::{App, http, HttpServer};
use actix_cors::Cors;
use lazy_static::lazy_static;
use mutex::Mutex;
use crate::route::{set_player_connected, get_players_connected, player_ready, tetris, snake};
use serde::{Serialize, Deserialize};

const IP_ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 4006;


#[derive(Serialize, Deserialize)]
pub struct Player {
    pseudo: String,
    is_connected: bool,
    id: String,
    game: String,
}




lazy_static! {
    static ref PLAYERS: Mutex<Vec<Player>> = Mutex::new(vec![]);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server start on {}:{}", IP_ADDRESS, PORT);
    HttpServer::new(|| {
        /* let cors: Cors = Cors::default()
             .allow_any_origin()
             .send_wildcard();*/
        let cors: Cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(set_player_connected)
            .service(get_players_connected)
            .service(player_ready)
            .service(tetris)
            .service(snake)
    })
        .bind((IP_ADDRESS, PORT))?
        .run()
        .await.expect("Impossible de demarrer le serveur");


    Ok(())
}

fn select_game_mode() {}