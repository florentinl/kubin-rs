#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Corner {
    pub piece: Piece,
    pub orientation: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    Urf,
    Ufl,
    Ulb,
    Ubr,
    Dfr,
    Dlf,
    Dbl,
    Drb,
}

impl Corner {
    pub(super) fn new(piece: Piece, orientation: u8) -> Self {
        Self { piece, orientation }
    }

    pub(super) fn rotate(&mut self, rotation: u8) {
        self.orientation = (self.orientation + rotation) % 3;
    }
}
