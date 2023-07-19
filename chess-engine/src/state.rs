use super::board::Space;
use super::piece::{Piece, PieceType};
use super::player::Player;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    white: Player,
    black: Player,
    pub board: HashMap<(i32, i32), Space>,
}

impl State {
    fn setup_players(&mut self, first_player: bool, ai: bool) {
        let mut player = Player {
            first_player: true,
            pieces: Vec::new(),
            ai: false,
            king_coord: (0, 0),
        };

        player.ai = ai;

        let mut starting_pos = if first_player { 1 } else { 8 };

        let mut pieces: Vec<Piece> = vec![
            Piece {
                current_coords: (1, starting_pos),
                piece_type: PieceType::Rook,
            },
            Piece {
                current_coords: (8, starting_pos),
                piece_type: PieceType::Rook,
            },
            Piece {
                current_coords: (2, starting_pos),
                piece_type: PieceType::Knight,
            },
            Piece {
                current_coords: (7, starting_pos),
                piece_type: PieceType::Knight,
            },
            Piece {
                current_coords: (3, starting_pos),
                piece_type: PieceType::Bishop,
            },
            Piece {
                current_coords: (6, starting_pos),
                piece_type: PieceType::Bishop,
            },
            Piece {
                current_coords: (4, starting_pos),
                piece_type: PieceType::Queen,
            },
            Piece {
                current_coords: (5, starting_pos),
                piece_type: PieceType::King,
            },
        ];

        player.king_coord = (5, starting_pos);

        if first_player {
            starting_pos += 1;
        } else {
            starting_pos -= 1;
        }

        for n in 1..=8 {
            pieces.push(Piece {
                current_coords: (n, starting_pos),
                piece_type: PieceType::Pawn,
            });
        }

        for piece in &pieces {
            self.board.insert(
                piece.current_coords,
                Space {
                    occupied: Some(piece.clone()),
                    evaluated_value: 0,
                },
            );
        }

        player.pieces = pieces;

        if first_player {
            self.white = player;
        } else {
            self.black = player;
        }
    }

    fn setup_spaces(&mut self) {
        for i in 3..=6 {
            for j in 1..=8 {
                self.board.insert(
                    (i, j),
                    Space {
                        occupied: None,
                        evaluated_value: 0,
                    },
                );
            }
        }
    }

    pub fn get_coords(&self, first_player: bool) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
        if first_player {
            return (
                self.white.pieces.iter().map(|x| x.current_coords).collect(),
                self.black.pieces.iter().map(|x| x.current_coords).collect(),
            );
        } else {
            return (
                self.black.pieces.iter().map(|x| x.current_coords).collect(),
                self.white.pieces.iter().map(|x| x.current_coords).collect(),
            );
        }
    }

    pub fn get_potential_check(&self, first_player: bool) -> ((i32, i32), Vec<Piece>) {
        if first_player {
            (self.white.king_coord, self.black.pieces.clone())
        } else {
            (self.black.king_coord, self.white.pieces.clone())
        }
    }

    pub fn capture_piece(&mut self, first_player: bool, coord: (i32, i32)) {
        let a = self.board.get_mut(&coord);
        a.unwrap().occupied = None;

        let (_, enemy_coords) = self.get_coords(first_player);

        let index = enemy_coords.iter().position(|&x| x == coord);
        let index = index.unwrap();

        let pieces = if first_player {
            &mut self.white.pieces
        } else {
            &mut self.black.pieces
        };

        pieces.remove(index);
    }

    pub fn update_move(
        &mut self,
        current_coords: (i32, i32),
        destination: (i32, i32),
        first_player: bool,
    ) {
        // Remove Space Allocation
        let a = self.board.get_mut(&current_coords).unwrap();
        a.occupied = None;

        let (player_coords, _) = self.get_coords(first_player);

        let index = player_coords.iter().position(|&x| x == current_coords);
        let index = index.unwrap();

        let pieces = if first_player {
            &mut self.white.pieces
        } else {
            &mut self.black.pieces
        };

        if let Some(piece) = pieces.get_mut(index) {
            let b = self.board.get_mut(&destination).unwrap();
            piece.current_coords = destination;
            b.occupied = Some(piece.clone());

            // Update King Position
            if piece.piece_type == PieceType::King {
                if first_player {
                    self.white.king_coord = destination;
                } else {
                    self.black.king_coord = destination;
                }
            }
        }
    }

    fn determine_endgame(&mut self, first_player: bool) -> bool {
        let (king_coord, enemy_pieces) = self.get_potential_check(first_player);

        let player_pieces = if first_player {
            self.white.pieces.clone()
        } else {
            self.black.pieces.clone()
        };

        let mut val = false;

        let checks: Vec<Piece> = enemy_pieces
            .iter()
            .cloned()
            .filter(|x| {
                let mut piece = x.clone();
                piece
                    .generate_possible_moves(!first_player)
                    .contains(&king_coord)
            })
            .collect();

        if !checks.is_empty() {
            // CheckMate
            if checks.len() == 1 {
                let legal_moves: Vec<(i32, i32)> = player_pieces
                    .iter()
                    .flat_map(|x| {
                        let mut piece = x.clone();
                        piece.generate_legal_moves(first_player)
                    })
                    .collect();

                if legal_moves.is_empty() {
                    val = true;
                }
            } else {
                let pos = self.board.get(&king_coord).unwrap();
                let mut king = pos.occupied.clone().unwrap();

                let legal_moves = king.generate_legal_moves(first_player);

                if legal_moves.is_empty() {
                    val = true;
                }
            }
        } else {
            // Stalemate
            let legal_moves: Vec<(i32, i32)> = player_pieces
                .iter()
                .flat_map(|x| {
                    let mut piece = x.clone();
                    piece.generate_legal_moves(first_player)
                })
                .collect();

            if legal_moves.is_empty() {
                val = true;
            }
        }

        val
    }
}

pub static mut STATE: Lazy<State> = Lazy::new(|| State {
    white: Player {
        first_player: true,
        pieces: Vec::new(),
        ai: false,
        king_coord: (5, 1),
    },
    black: Player {
        first_player: false,
        pieces: Vec::new(),
        ai: false,
        king_coord: (5, 8),
    },
    board: HashMap::new(),
});
