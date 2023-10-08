use crate::game::{Player, UltimateTicTacToe, GameState};
use crate::messages::{GameUpdate};

use std::io::{self, Error, ErrorKind};
use std::net::{TcpListener, TcpStream};

const CONFIG: bincode::config::Configuration = bincode::config::standard();

pub fn run(port: String) -> io::Result<()> {
    let listener = TcpListener::bind(String::from("0.0.0.0:") + &port)?;

    // TODO: randomize who is who, as well as who starts first
    let mut player1 = get_stream(&listener)?;
    let mut player2 = get_stream(&listener)?;

    // telling player 1 they're X
    bincode::encode_into_std_write(Player::Ex, &mut player1, CONFIG)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    // telling player 2 they're O
    bincode::encode_into_std_write(Player::Oh, &mut player2, CONFIG)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    let mut game = UltimateTicTacToe::new();

    let mut player1_turn = true;
    bincode::encode_into_std_write(GameUpdate::Start, &mut player1, CONFIG)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    fn get_player_stream<'a>(is_p1_turn: bool, p1: &'a mut TcpStream, p2: &'a mut TcpStream) -> &'a mut TcpStream {
        if is_p1_turn { p1 } else { p2 }
    }

    loop {
        let board_pos_resp = bincode::decode_from_std_read(get_player_stream(player1_turn, &mut player1, &mut player2), CONFIG)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        if !game.move_is_valid(board_pos_resp) {
            bincode::encode_into_std_write(GameUpdate::BadMove, get_player_stream(player1_turn, &mut player1, &mut player2), CONFIG)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
            continue;
        }

        let _ = game.place(if player1_turn { Player::Ex } else { Player::Oh }, board_pos_resp);

        match game.game_state {
            GameState::Ongoing => {
                player1_turn = !player1_turn;

                bincode::encode_into_std_write(
                    GameUpdate::Move(board_pos_resp),
                    get_player_stream(player1_turn, &mut player1, &mut player2),
                    CONFIG
                ).map_err(|e| Error::new(ErrorKind::Other, e))?;
            }
            GameState::Won(_) => {
                bincode::encode_into_std_write(
                    GameUpdate::GameWon,
                    get_player_stream(player1_turn, &mut player1, &mut player2),
                    CONFIG
                ).map_err(|e| Error::new(ErrorKind::Other, e))?;

                bincode::encode_into_std_write(
                    GameUpdate::GameWonByOpponent(board_pos_resp),
                    get_player_stream(!player1_turn, &mut player1, &mut player2),
                    CONFIG
                ).map_err(|e| Error::new(ErrorKind::Other, e))?;

                break;
            }
            GameState::Drawn => {
                bincode::encode_into_std_write(
                    GameUpdate::GameDrawn,
                    get_player_stream(player1_turn, &mut player1, &mut player2),
                    CONFIG
                ).map_err(|e| Error::new(ErrorKind::Other, e))?;

                bincode::encode_into_std_write(
                    GameUpdate::GameDrawnByOpponent(board_pos_resp),
                    get_player_stream(!player1_turn, &mut player1, &mut player2),
                    CONFIG
                ).map_err(|e| Error::new(ErrorKind::Other, e))?;

                break;
            }
        };

    }

    Ok(())
}

const ERR_LIMIT: usize = 10;

fn get_stream(listener: &TcpListener) -> io::Result<TcpStream> {
    let mut num_times_erred = 0;

    loop {
        match listener.accept() {
            Ok((stream, _)) => return Ok(stream),
            Err(e) => {
                println!("Error connecting: {e}");
                num_times_erred += 1;

                if num_times_erred >= ERR_LIMIT {
                    return Err(Error::new(ErrorKind::NotConnected, e));
                }
            }
        }
    }
}
