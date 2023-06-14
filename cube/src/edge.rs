#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub piece: EdgePiece,
    pub orientation: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EdgePiece {
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
    pub(super) fn new(piece: EdgePiece, orientation: u8) -> Self {
        Self { piece, orientation }
    }

    pub(super) fn flip(&mut self) {
        self.orientation = (self.orientation + 1) % 2;
    }
}
