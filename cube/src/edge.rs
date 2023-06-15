#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub piece: Piece,
    pub orientation: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    UR,
    UF,
    UL,
    UB,
    DR,
    DF,
    DL,
    DB,
    FR,
    FL,
    BL,
    BR,
}

impl Edge {
    pub(super) fn new(piece: Piece, orientation: u8) -> Self {
        Self { piece, orientation }
    }

    pub(super) fn flip(&mut self) {
        self.orientation = (self.orientation + 1) % 2;
    }
}
