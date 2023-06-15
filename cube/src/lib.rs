use self::algorithms::Move;

pub mod algorithms;
pub mod corner;
pub mod edge;
pub mod subcases;

/// Edges in the following order: UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR
pub const UR: usize = 0;
pub const UF: usize = 1;
pub const UL: usize = 2;
pub const UB: usize = 3;
pub const DR: usize = 4;
pub const DF: usize = 5;
pub const DL: usize = 6;
pub const DB: usize = 7;
pub const FR: usize = 8;
pub const FL: usize = 9;
pub const BL: usize = 10;
pub const BR: usize = 11;

/// Corners in the following order: URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB
pub const URF: usize = 0;
pub const UFL: usize = 1;
pub const ULB: usize = 2;
pub const UBR: usize = 3;
pub const DFR: usize = 4;
pub const DLF: usize = 5;
pub const DBL: usize = 6;
pub const DRB: usize = 7;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube {
    /// Edges in the following order: UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR
    pub edges: [edge::Edge; 12],
    /// Corners in the following order: URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB
    pub corners: [corner::Corner; 8],
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            edges: [
                edge::Edge::new(edge::Piece::UR, 0),
                edge::Edge::new(edge::Piece::UF, 0),
                edge::Edge::new(edge::Piece::UL, 0),
                edge::Edge::new(edge::Piece::UB, 0),
                edge::Edge::new(edge::Piece::DR, 0),
                edge::Edge::new(edge::Piece::DF, 0),
                edge::Edge::new(edge::Piece::DL, 0),
                edge::Edge::new(edge::Piece::DB, 0),
                edge::Edge::new(edge::Piece::FR, 0),
                edge::Edge::new(edge::Piece::FL, 0),
                edge::Edge::new(edge::Piece::BL, 0),
                edge::Edge::new(edge::Piece::BR, 0),
            ],
            corners: [
                corner::Corner::new(corner::Piece::Urf, 0),
                corner::Corner::new(corner::Piece::Ufl, 0),
                corner::Corner::new(corner::Piece::Ulb, 0),
                corner::Corner::new(corner::Piece::Ubr, 0),
                corner::Corner::new(corner::Piece::Dfr, 0),
                corner::Corner::new(corner::Piece::Dlf, 0),
                corner::Corner::new(corner::Piece::Dbl, 0),
                corner::Corner::new(corner::Piece::Drb, 0),
            ],
        }
    }
}

impl Cube {
    pub fn execute_move(&mut self, move_: &Move) {
        match move_ {
            Move::U => self.u(),
            Move::U2 => self.u2(),
            Move::Up => self.u_prime(),
            Move::R => self.r(),
            Move::R2 => self.r2(),
            Move::Rp => self.r_prime(),
            Move::F => self.f(),
            Move::F2 => self.f2(),
            Move::Fp => self.f_prime(),
            Move::D => self.d(),
            Move::D2 => self.d2(),
            Move::Dp => self.d_prime(),
            Move::L => self.l(),
            Move::L2 => self.l2(),
            Move::Lp => self.l_prime(),
            Move::B => self.b(),
            Move::B2 => self.b2(),
            Move::Bp => self.b_prime(),
            Move::None => (),
        }
    }

    pub fn execute_algorithm(&mut self, algorithm: &[Move]) {
        for move_ in algorithm {
            self.execute_move(move_);
        }
    }

    pub(crate) fn u(&mut self) {
        // Circular shift of edges
        self.edges.swap(UR, UB);
        self.edges.swap(UB, UL);
        self.edges.swap(UL, UF);

        // Circular shift of corners
        self.corners.swap(URF, UBR);
        self.corners.swap(UBR, ULB);
        self.corners.swap(ULB, UFL);
    }

    pub(crate) fn u2(&mut self) {
        self.u();
        self.u();
    }

    pub(crate) fn u_prime(&mut self) {
        self.u();
        self.u();
        self.u();
    }

    pub(crate) fn r(&mut self) {
        // Circular shift of edges
        self.edges.swap(UR, FR);
        self.edges.swap(FR, DR);
        self.edges.swap(DR, BR);

        // Circular shift of corners
        self.corners[URF].rotate(1);
        self.corners[DRB].rotate(1);
        self.corners[DFR].rotate(2);
        self.corners[UBR].rotate(2);
        self.corners.swap(URF, UBR);
        self.corners.swap(DFR, URF);
        self.corners.swap(DRB, DFR);
    }

    pub(crate) fn r2(&mut self) {
        self.r();
        self.r();
    }

    pub(crate) fn r_prime(&mut self) {
        self.r();
        self.r();
        self.r();
    }

    pub(crate) fn f(&mut self) {
        // Circular shift of edges
        self.edges[UF].flip();
        self.edges[FR].flip();
        self.edges[DF].flip();
        self.edges[FL].flip();
        self.edges.swap(UF, FR);
        self.edges.swap(FL, UF);
        self.edges.swap(DF, FL);

        // Circular shift of corners
        self.corners[URF].rotate(2);
        self.corners[DFR].rotate(1);
        self.corners[DLF].rotate(2);
        self.corners[UFL].rotate(1);
        self.corners.swap(URF, UFL);
        self.corners.swap(UFL, DLF);
        self.corners.swap(DLF, DFR);
    }

    pub(crate) fn f2(&mut self) {
        self.f();
        self.f();
    }

    pub(crate) fn f_prime(&mut self) {
        self.f();
        self.f();
        self.f();
    }

    pub(crate) fn l(&mut self) {
        // Circular shift of edges
        self.edges.swap(UL, FL);
        self.edges.swap(BL, UL);
        self.edges.swap(DL, BL);

        // Circular shift of corners
        self.corners[ULB].rotate(1);
        self.corners[UFL].rotate(2);
        self.corners[DLF].rotate(1);
        self.corners[DBL].rotate(2);
        self.corners.swap(ULB, UFL);
        self.corners.swap(DBL, ULB);
        self.corners.swap(DLF, DBL);
    }

    pub(crate) fn l2(&mut self) {
        self.l();
        self.l();
    }

    pub(crate) fn l_prime(&mut self) {
        self.l();
        self.l();
        self.l();
    }

    pub(crate) fn b(&mut self) {
        // Circular shift of edges
        self.edges[UB].flip();
        self.edges[BR].flip();
        self.edges[DB].flip();
        self.edges[BL].flip();
        self.edges.swap(UB, BR);
        self.edges.swap(BR, DB);
        self.edges.swap(DB, BL);

        // Circular shift of corners
        self.corners[UBR].rotate(1);
        self.corners[DRB].rotate(2);
        self.corners[DBL].rotate(1);
        self.corners[ULB].rotate(2);
        self.corners.swap(UBR, DRB);
        self.corners.swap(DRB, DBL);
        self.corners.swap(DBL, ULB);
    }

    pub(crate) fn b2(&mut self) {
        self.b();
        self.b();
    }

    pub(crate) fn b_prime(&mut self) {
        self.b();
        self.b();
        self.b();
    }

    pub(crate) fn d(&mut self) {
        // Circular shift of edges
        self.edges.swap(DR, DF);
        self.edges.swap(DF, DL);
        self.edges.swap(DL, DB);

        // Circular shift of corners
        self.corners.swap(DRB, DFR);
        self.corners.swap(DFR, DLF);
        self.corners.swap(DLF, DBL);
    }

    pub(crate) fn d2(&mut self) {
        self.d();
        self.d();
    }

    pub(crate) fn d_prime(&mut self) {
        self.d();
        self.d();
        self.d();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_algorithm() {
        let mut cube = super::Cube::default();

        let sune = vec![
            super::Move::R,
            super::Move::U,
            super::Move::Rp,
            super::Move::U,
            super::Move::R,
            super::Move::U2,
            super::Move::Rp,
        ];

        // A Sune is 6-periodic (i.e. we are back to the original state after 6 repetitions)
        for _ in 0..6 {
            for move_ in &sune {
                cube.execute_move(move_);
            }
        }

        assert_eq!(cube, super::Cube::default());
    }

    #[test]
    fn two_algorithms_same_final_state() {
        let mut cube_1 = super::Cube::default();
        let mut cube_2 = super::Cube::default();

        let t_perm_1 = super::algorithms::parse_algorithm("R U R' U' R' F R2 U' R' U' R U R' F'");
        let t_perm_2 = super::algorithms::parse_algorithm(
            "R2 D' F2 U' R U R' F' R U R' U' R' F R2 U' R' F2 D R2",
        );

        cube_1.execute_algorithm(&t_perm_1);
        cube_2.execute_algorithm(&t_perm_2);

        println!("Solved cube: {:?}", super::Cube::default());
        assert_eq!(cube_1, cube_2);
    }
}
