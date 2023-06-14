#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Corner {
    pub(crate) piece: CornerPiece,
    pub(crate) orientation: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub(super) fn new(piece: CornerPiece, orientation: u8) -> Self {
        Self { piece, orientation }
    }

    pub(super) fn rotate(&mut self, rotation: u8) {
        self.orientation = (self.orientation + rotation) % 3;
    }
}
