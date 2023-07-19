use super::state::STATE;
use super::utils::is_within_board_limits;

#[derive(Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub current_coords: (i32, i32),
}

#[derive(Clone, PartialEq)]
pub enum PieceType {
    King = 1,
    Queen = 2,
    Rook = 3,
    Knight = 4,
    Bishop = 5,
    Pawn = 6,
}

impl Piece {
    pub fn generate_possible_moves(&mut self, first_player: bool) -> Vec<(i32, i32)> {
        match self.piece_type {
            PieceType::Pawn => {
                let (current_x, current_y) = self.current_coords;

                let mut possible_coords: Vec<(i32, i32)> = Vec::new();
                let (player_coords, enemy_coords) = unsafe { STATE.get_coords(first_player) };

                let y_direction = if first_player { 1 } else { -1 };

                let mut forward_coords = Vec::with_capacity(2);
                forward_coords.push((current_x, current_y + y_direction));

                if (current_y == 2 && first_player) || (current_y == 7 && !first_player) {
                    forward_coords.push((current_x, current_y + (y_direction * 2)));
                }

                possible_coords.extend(forward_coords.iter().filter(|&coord| {
                    is_within_board_limits(coord.0, coord.1) && !player_coords.contains(coord)
                }));

                let capture_coords = [
                    (current_x + 1, current_y + y_direction),
                    (current_x - 1, current_y + y_direction),
                ];

                possible_coords.extend(capture_coords.iter().filter(|&coord| {
                    is_within_board_limits(coord.0, coord.1) && enemy_coords.contains(coord)
                }));

                possible_coords
            }

            PieceType::Bishop => {
                let (current_x, current_y) = self.current_coords;

                let mut possible_coords: Vec<(i32, i32)> = Vec::new();
                let (player_coords, enemy_coords) = unsafe { STATE.get_coords(first_player) };

                let directions = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

                for direction in directions.iter() {
                    let mut x = current_x + direction.0;
                    let mut y = current_y + direction.1;

                    while is_within_board_limits(x, y) && !player_coords.contains(&(x, y)) {
                        possible_coords.push((x, y));

                        if enemy_coords.contains(&(x, y)) {
                            break;
                        }

                        x += direction.0;
                        y += direction.1;
                    }
                }

                possible_coords
            }

            PieceType::Rook => {
                let (current_x, current_y) = self.current_coords;

                let mut possible_coords: Vec<(i32, i32)> = Vec::new();
                let (player_coords, enemy_coords) = unsafe { STATE.get_coords(first_player) };

                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

                for direction in directions.iter() {
                    let mut x = current_x + direction.0;
                    let mut y = current_y + direction.1;

                    while is_within_board_limits(x, y) && !player_coords.contains(&(x, y)) {
                        possible_coords.push((x, y));

                        if enemy_coords.contains(&(x, y)) {
                            break;
                        }

                        x += direction.0;
                        y += direction.1;
                    }
                }

                possible_coords
            }

            PieceType::Knight => {
                let (current_x, current_y) = self.current_coords;

                let mut possible_coords: Vec<(i32, i32)> = vec![
                    (current_x + 1, current_y + 2),
                    (current_x - 1, current_y + 2),
                    (current_x + 2, current_y + 1),
                    (current_x + 2, current_y - 1),
                    (current_x + 1, current_y - 2),
                    (current_x - 1, current_y - 2),
                    (current_x + 2, current_y + 1),
                    (current_x + 2, current_y - 1),
                ];

                let (player_coords, _) = unsafe { STATE.get_coords(first_player) };

                possible_coords.retain(|&coord| {
                    is_within_board_limits(coord.0, coord.1) && !player_coords.contains(&coord)
                });

                possible_coords
            }

            PieceType::Queen => {
                let (current_x, current_y) = self.current_coords;

                let mut possible_coords: Vec<(i32, i32)> = Vec::new();
                let (player_coords, enemy_coords) = unsafe { STATE.get_coords(first_player) };

                let directions = [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                    (-1, -1),
                ];

                for direction in directions.iter() {
                    let mut x = current_x + direction.0;
                    let mut y = current_y + direction.1;

                    while is_within_board_limits(x, y) && !player_coords.contains(&(x, y)) {
                        possible_coords.push((x, y));

                        if enemy_coords.contains(&(x, y)) {
                            break;
                        }

                        x += direction.0;
                        y += direction.1;
                    }
                }

                possible_coords
            }

            PieceType::King => {
                let (current_x, current_y) = self.current_coords;

                let mut possible_coords: Vec<(i32, i32)> = Vec::new();
                let (player_coords, _) = unsafe { STATE.get_coords(first_player) };

                let directions = [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                    (-1, -1),
                ];

                for direction in directions.iter() {
                    let x = current_x + direction.0;
                    let y = current_y + direction.1;

                    if is_within_board_limits(x, y) && !player_coords.contains(&(x, y)) {
                        possible_coords.push((x, y));
                    }
                }

                possible_coords
            }
        }
    }

    pub fn generate_legal_moves(&mut self, first_player: bool) -> Vec<(i32, i32)> {
        let moves: Vec<(i32, i32)> = self.generate_possible_moves(first_player);

        let (king_coord, enemy_pieces) = unsafe { STATE.get_potential_check(first_player) };

        let mut legal_moves: Vec<(i32, i32)> = vec![];

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

        if checks.is_empty() {
            legal_moves = moves;
        } else {
            checks.iter().for_each(|x: &Piece| {
                // Kill Enemy
                let mut protected_coords = enemy_pieces.iter().flat_map(|x| {
                    let mut piece = x.clone();
                    piece.generate_possible_moves(!first_player)
                });

                // Allow Killing if Protection is Non-Existent
                if !protected_coords.any(|move_coord| move_coord == x.current_coords) {
                    legal_moves.push(x.current_coords);
                }

                let a: Vec<(i32, i32)> = protected_coords.collect();

                // Escape Check
                legal_moves.extend(moves.iter().filter(|x| !a.contains(x)));
            });

            if checks.len() == 1 {
                let piece = checks.get(0).unwrap();

                if self.piece_type != PieceType::King {
                    if moves.contains(&piece.current_coords) {
                        legal_moves.push(piece.current_coords);
                    }
                }

                // Generate Blocks
                match piece.piece_type {
                    PieceType::Bishop => {
                        let (coord1, coord2) = (king_coord, piece.current_coords);

                        let (start_x, end_x) = if coord1.0 < coord2.0 {
                            (coord1.0 + 1, coord2.0 - 1)
                        } else {
                            (coord2.0 + 1, coord1.0 - 1)
                        };

                        let (start_y, end_y) = if coord1.1 < coord2.1 {
                            (coord1.1 + 1, coord2.1 - 1)
                        } else {
                            (coord2.1 + 1, coord1.1 - 1)
                        };

                        if (coord1.0 - coord2.0).abs() == (coord1.1 - coord2.1).abs() {
                            for i in start_x..=end_x {
                                let block = (i, start_y + i - start_x);
                                if moves.contains(&block) {
                                    legal_moves.push(block);
                                }
                            }
                        }
                    }

                    PieceType::Rook => {
                        let (coord1, coord2) = (king_coord, piece.current_coords);

                        let (start, diff) = if coord1.0 == coord2.0 {
                            (coord1.1.min(coord2.1), (coord1.1 - coord2.1).abs())
                        } else {
                            (coord1.0.min(coord2.0), (coord1.0 - coord2.0).abs())
                        };

                        for i in start..diff {
                            let block = if coord1.0 == coord2.0 {
                                (coord1.0, start + i)
                            } else {
                                (start + i, coord1.1)
                            };

                            if moves.contains(&block) {
                                legal_moves.push(block);
                            }
                        }
                    }

                    PieceType::Queen => {
                        let (coord1, coord2) = (king_coord, piece.current_coords);

                        let (start_x, end_x) = if coord1.0 < coord2.0 {
                            (coord1.0 + 1, coord2.0 - 1)
                        } else {
                            (coord2.0 + 1, coord1.0 - 1)
                        };

                        let (start_y, end_y) = if coord1.1 < coord2.1 {
                            (coord1.1 + 1, coord2.1 - 1)
                        } else {
                            (coord2.1 + 1, coord1.1 - 1)
                        };

                        if (coord1.0 - coord2.0).abs() == (coord1.1 - coord2.1).abs() {
                            for i in start_x..=end_x {
                                let block = (i, start_y + i - start_x);
                                if moves.contains(&block) {
                                    legal_moves.push(block);
                                }
                            }
                        } else {
                            let (start, diff) = if coord1.0 == coord2.0 {
                                (coord1.1.min(coord2.1), (coord1.1 - coord2.1).abs())
                            } else {
                                (coord1.0.min(coord2.0), (coord1.0 - coord2.0).abs())
                            };

                            for i in start..diff {
                                let block = if coord1.0 == coord2.0 {
                                    (coord1.0, start + i)
                                } else {
                                    (start + i, coord1.1)
                                };

                                if moves.contains(&block) {
                                    legal_moves.push(block);
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }
        }

        legal_moves
    }

    pub fn navigate(&mut self, destination: (i32, i32), first_player: bool) {
        let legal_moves = self.generate_legal_moves(first_player);

        if legal_moves.contains(&destination) {
            let (_, enemy_coords) = unsafe { STATE.get_coords(first_player) };

            if enemy_coords.contains(&destination) {
                unsafe {
                    STATE.capture_piece(first_player, destination);
                }
            }

            if self.piece_type == PieceType::Pawn {
                let dest_y = if first_player { 8 } else { 1 };

                if dest_y == destination.1 {
                    // Transform
                }
            }

            unsafe {
                STATE.update_move(self.current_coords, destination, first_player);
            }
        }
    }
}
