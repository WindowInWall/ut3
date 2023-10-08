use crate::game::{Player, UltimateTicTacToe};
use crate::messages::{BoardPos, GameUpdate};
use std::net::TcpStream;
use std::io::{self, Error, ErrorKind};

const CONFIG: bincode::config::Configuration = bincode::config::standard();

pub fn run(ip: String, port: String) -> io::Result<()> {
    let mut stream = TcpStream::connect(ip + ":" + &port)?;
    let identity = bincode::decode_from_std_read(&mut stream, CONFIG).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    
    match identity {
        Player::Ex => println!("You are X"),
        Player::Oh => println!("You are O"),
    };

    let opponent = match identity {
        Player::Ex => Player::Oh,
        Player::Oh => Player::Ex,
    };

    let mut game = UltimateTicTacToe::new();
    let mut prev_move: Option<BoardPos> = None;

    loop {
        let message = bincode::decode_from_std_read(&mut stream, CONFIG).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        match message {
            GameUpdate::Start => {
                println!("{}", game.board_as_string());
                let board_pos = prompt_for_move(&game);
                prev_move = Some(board_pos);
                bincode::encode_into_std_write(board_pos, &mut stream, CONFIG).map_err(|e| Error::new(ErrorKind::Other, e))?;
            },
            GameUpdate::Move(pos) => {
                if let Some(m) = prev_move {
                    game.place(identity, m)
                        .expect("server validated move");
                }
                game.place(opponent, pos).expect("server should have validated move");
                println!("{}", game.board_as_string());
                let board_pos = prompt_for_move(&game);
                prev_move = Some(board_pos);
                bincode::encode_into_std_write(board_pos, &mut stream, CONFIG).map_err(|e| Error::new(ErrorKind::Other, e))?;
            }
            GameUpdate::GameWon => {
                game.place(identity, prev_move.expect("no way for game to be won in zero moves"))
                    .expect("server should have validated move");
                println!("{}", game.board_as_string());
                println!("You won - yay!");
                break;
            }
            GameUpdate::GameWonByOpponent(pos) => {
                game.place(identity, prev_move.expect("no way for game to be won in zero moves"))
                    .expect("server should have validated move");
                game.place(opponent, pos).expect("server should have validated move");
                println!("{}", game.board_as_string());
                println!("You lost - :(");
                break;
            }
            GameUpdate::GameDrawn => {
                game.place(identity, prev_move.expect("no way for game to be drawn in zero moves"))
                    .expect("server should have validated move");
                println!("{}", game.board_as_string());
                println!("Game drawn");
                break;
            }
            GameUpdate::GameDrawnByOpponent(pos) => {
                game.place(identity, prev_move.expect("no way for game to be drawn in zero moves"))
                    .expect("server should have validated move");
                game.place(opponent, pos).expect("server should have validated move");
                println!("{}", game.board_as_string());
                println!("Game drawn");
                break;
            }
            GameUpdate::BadMove => {
                println!("server says you made a bad move >:(");
                println!("{}", game.board_as_string());
                let board_pos = prompt_for_move(&game);
                prev_move = Some(board_pos);
                bincode::encode_into_std_write(board_pos, &mut stream, CONFIG).map_err(|e| Error::new(ErrorKind::Other, e))?;
            }
        };
    }
    Ok(())
}

fn prompt_for_move(game: &UltimateTicTacToe) -> BoardPos {
    let stdin = io::stdin();

    if let Some(f) = game.focus {
        loop {
            println!("playing in square {}, input subsquare to play in:", f + 1);
            let mut buffer = String::new();
            stdin
                .read_line(&mut buffer)
                .expect("failed to read line");
            let square: usize = match buffer.trim().parse() {
                Ok(num) if num != 0 && num <= 9 => num,
                _ => {
                    println!("invalid input");
                    continue;
                },
            };
            return BoardPos::WithoutFocus(square);
        }
    } else {
        let focus;
        loop {
            println!("input supersquare to play in:");
            let mut buffer = String::new();
            stdin
                .read_line(&mut buffer)
                .expect("failed to read line");
            match buffer.trim().parse() {
                Ok(num) if num != 0 && num <= 9 => {
                    focus = num;
                    break;
                },
                _ => {
                    println!("invalid input");
                    continue;
                },
            };
        }

        let square;
        loop {
            println!("input subsquare to play in:");
            let mut buffer = String::new();
            stdin
                .read_line(&mut buffer)
                .expect("failed to read line");
            match buffer.trim().parse() {
                Ok(num) if num != 0 && num <= 9 => {
                    square = num;
                    break;
                },
                _ => {
                    println!("invalid input");
                    continue;
                },
            };
        }
        return BoardPos::WithFocus(focus, square);
    }
}
