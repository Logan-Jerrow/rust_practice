#[macro_export]
macro_rules! piece {
    (p) => {
        Some(Piece::new($crate::piece::Color::White, Kind::Pawn))
    };
    (r) => {
        Some(Piece::new($crate::piece::Color::White, Kind::Rook))
    };
    (b) => {
        Some(Piece::new($crate::piece::Color::White, Kind::Bishop))
    };
    (h) => {
        Some(Piece::new($crate::piece::Color::White, Kind::Knight))
    };
    (q) => {
        Some(Piece::new($crate::piece::Color::White, Kind::Queen))
    };
    (k) => {
        Some(Piece::new($crate::piece::Color::White, Kind::King))
    };

    (P) => {
        Some(Piece::new($crate::piece::Color::Black, Kind::Pawn))
    };
    (R) => {
        Some(Piece::new($crate::piece::Color::Black, Kind::Rook))
    };
    (B) => {
        Some(Piece::new($crate::piece::Color::Black, Kind::Bishop))
    };
    (H) => {
        Some(Piece::new($crate::piece::Color::Black, Kind::Knight))
    };
    (Q) => {
        Some(Piece::new($crate::piece::Color::Black, Kind::Queen))
    };
    (K) => {
        Some(Piece::new($crate::piece::Color::Black, Kind::King))
    };
    (.) => {
        None
    };
}

#[macro_export(local_inner_macros)]
macro_rules! board {
[$($piece:tt) *] => {
    Board([
    $(
        piece!($piece),
    )*
    ])};
}
