#[cfg(test)]
use crate::logic::board::chessboard::{ChessBoard, Field, Piece, PieceType, Color};
use crate::logic::board::aux;

#[test]
fn rook_legal_moves_1(){
    let mut cl = ChessBoard::new();
    cl.all_empty();
    cl.set_piece(PieceType::Rook,Color::White,false,3,3);
    let fields = cl.legal_moves(3,3);
    let pre_calc = [(0,3),(1,3),(2,3),(4,3),(5,3),(6,3),(7,3)];

    for tup in pre_calc.iter() {
        let has = aux::contains(&fields, *tup);
        if !has {
            println!("{:?} not in vector {:?}",tup, fields);
        }
        assert!(has);
    }
}

#[test]
fn bishop_legal_moves_1(){
    let mut cl = ChessBoard::new();
    cl.all_empty();
    cl.set_piece(PieceType::Bishop,Color::White,false,7,7);
    let fields = cl.legal_moves(7,7);
    let pre_calc = [(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6)];

    for tup in pre_calc.iter() {
        let has = aux::contains(&fields, *tup);
        if !has {
            println!("{:?} not in vector {:?}",tup, fields);
        }
        assert!(has);
    }
}

#[test]
fn queen_legal_moves_1(){
    let mut cl = ChessBoard::new();
    cl.all_empty();
    cl.set_piece(PieceType::Queen,Color::White,false,7,7);
    let fields = cl.legal_moves(7,7);
    let pre_calc = [
        (0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6),
        (6,7),(5,7),(4,7),(3,7),(2,7),(1,7),(0,7),
        (7,6),(7,5),(7,4),(7,3),(7,2),(7,1),(7,0)
    ];

    for tup in pre_calc.iter() {
        let has = aux::contains(&fields, *tup);
        if !has {
            println!("{:?} not in vector {:?}",tup, fields);
        }
        assert!(has);
    }
}


