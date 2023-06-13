#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Corner {
    pub(crate) piece: CornerPiece,
    pub(crate) orientation: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CornerPiece {
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
