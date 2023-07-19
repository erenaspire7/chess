use crate::state::STATE;

use super::piece::Piece;
use super::utils::is_within_board_limits;

#[derive(Clone)]
pub struct Player {
    pub first_player: bool,
    pub ai: bool,
    pub pieces: Vec<Piece>,
    pub king_coord: (i32, i32),
}

impl Player {
    fn move_piece(self, current_coords: Option<(i32, i32)>, destination: Option<(i32, i32)>) {
        if self.ai {
        } else {
            let map = unsafe { &STATE.board };

            let (x, y) = current_coords.unwrap();
            let (dest_x, dest_y) = destination.unwrap();

            if is_within_board_limits(x, y) && is_within_board_limits(dest_x, dest_y) {
                let space = map.get(&current_coords.unwrap()).unwrap();

                if !space.occupied.is_none() {
                    let mut p = space.occupied.clone().unwrap();

                    p.navigate(destination.unwrap(), self.first_player);
                }
            }
        }
    }
}
