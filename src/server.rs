use super::game::{
    UltimateTicTacToe,
    BoardPos,
    Player,
    GameState,
    GameError
};

use tonic::{
    transport::Server,
    Request,
    Response,
    Status
};

use game::{
    MoveRequest,
    MoveResponse,
    game_server::{
        Game,
        GameServer
    }
};

pub mod game {
    tonic::include_proto!("game");
}

#[derive(Default)]
pub struct GameService {
    game: UltimateTicTacToe,
}


#[tonic::async_trait]
impl Game for GameService {
    async fn make_move(&self, request: Request<MoveRequest>) -> Result<Response<MoveResponse>, Status> {
        let r = request.into_inner();
        if let Some(meta_board_pos) = r.meta_board_pos {
            // Not handling error for now
            let _ = self.game.place(Player::Ex, BoardPos::WithFocus(meta_board_pos as usize, r.small_board_pos as usize));
        } else {
            // Same here
            let _ = self.game.place(Player::Ex, BoardPos::WithoutFocus(r.small_board_pos as usize));
        }

        match self.game.game_state {
            GameState::Ongoing => server_make_move(&self.game),
            _ => panic!("what am i supposed to do here?!"),
        };

        Err(Status::new(tonic::Code::OutOfRange, "Invalid vote provided"))
    }
}

//pub fn run(port: String) -> Result<(), Box<dyn std::error::Error>> {
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    //let listener = TcpListener::bind(String::from("0.0.0.0:") + &port).unwrap();
    //let (mut player1, _) = listener.accept().unwrap();
    //let (mut player2, _) = listener.accept().unwrap();

    let mut game = UltimateTicTacToe::new();
    let address = "[::1]:8080".parse().unwrap();
    let game_service = GameService::default();

    Server::builder().add_service(GameServer::new(game_service))
        .serve(address)
        .await?;
    Ok(())
}

async fn server_make_move(_game: &UltimateTicTacToe) {
    println!("we made it here!");
}
