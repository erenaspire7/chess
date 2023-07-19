use super::piece::Piece;

#[derive(Clone)]
pub struct Space {
    pub occupied: Option<Piece>,
    pub evaluated_value: i32,
}
