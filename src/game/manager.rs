use super::player::Player;
use super::board::Board;
use super::gobblet::Gobblet;
use js_sys::Math;

#[derive(Debug)]
pub enum Turn {
    Player1,
    Player2
}

#[derive(Debug)]
pub struct Manager {
    player1: Player,
    player2: Player,
    board: Board,
    turn: Turn,
}

impl Manager {
    pub fn new(name1: String, name2: String) -> Manager {
        let player1 = Player::new(name1);
        let player2 = Player::new(name2);
        let board = Board::new();
        let turn = Manager::random_turn(); 

        Manager{ player1, player2, board, turn }
    }

    pub fn get_turn(&self) -> &Turn {
        &self.turn
    }

    fn random_turn() -> Turn {
        return if Math::random() > 0.5 {
            Turn::Player1
        } else {
            Turn::Player2
        }
    }
}