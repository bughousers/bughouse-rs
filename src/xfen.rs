pub mod xfen {
    use crate::logic::ChessLogic;
    use crate::logic::board;
    use crate::logic::board::Piece;
    use crate::parse::parser;

    pub fn gen_xfen(cl:&ChessLogic) -> bool {
        //pieces
        //active color
        //castling rights KQkq
        //en passant : behind the the pawn that has moved 2 squares
        //halfmove clock : num halfmoves after last capture
        //number of full moves

        //create for board1
        let mut activecol = 'w';
        match cl.white_active_1 {
            true =>{ activecol = 'w'},
            false =>{ activecol = 'b'},
        }

        let mut enpassant = "a1";
        match cl.get_pawn_in_last_turn(true) {
            None => {},
            Some((a,b)) => {},
        }

        true
    }
}
