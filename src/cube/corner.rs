#[derive(Clone, Debug, PartialEq)]
pub(super) struct Corner {
    pub(super) piece: CornerPiece,
    pub(super) orientation: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum CornerPiece {
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
    pub(super) fn new(piece: CornerPiece, orientation: usize) -> Self {
        Self { piece, orientation }
    }

    pub(super) fn rotate(&mut self, rotation: usize) {
        self.orientation = (self.orientation + rotation) % 3;
    }
}
