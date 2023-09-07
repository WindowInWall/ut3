use super::game::{
    UltimateTicTacToe,
    BoardPos,
    Player,
    GameState,
    GameError
};

use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

use game::{MoveRequest, game_client::GameClient};

// duplicated in server, fix
pub mod game {
    tonic::include_proto!("game");
}

pub async fn run(ip: String, port: String) -> Result<(), Box<dyn std::error::Error>> {
    //let listener = TcpListener::bind(String::from("0.0.0.0:") + &port).unwrap();

    //let game = UltimateTicTacToe::new();
    let mut client = GameClient::connect("http://[::1]:8080").await?;

    let req = tonic::Request::new(MoveRequest {
        meta_board_pos: Some(1),
        small_board_pos: 1,
    });
    client.make_move(req);
    Ok(())
}

    
