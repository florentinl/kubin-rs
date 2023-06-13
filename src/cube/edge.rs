#[derive(Clone, Debug, PartialEq)]
pub(super) struct Edge {
    pub(super) piece: EdgePiece,
    pub(super) orientation: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum EdgePiece {
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
    pub(super) fn new(piece: EdgePiece, orientation: usize) -> Self {
        Self { piece, orientation }
    }

    pub(super) fn flip(&mut self) {
        self.orientation = (self.orientation + 1) % 2;
    }
}
