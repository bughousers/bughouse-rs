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

        //get active color
        let mut activecol = 'w';
        match cl.get_white_active(true) {
            true =>{ activecol = 'w'},
            false =>{ activecol = 'b'},
        }

        //check for en passant
        let mut enpassant = "a1";
        match cl.get_pawn_in_last_turn(true) {
            None => {enpassant = "-"},
            Some((a,b)) => {
                if a == 3 || a == 4 {
                    let row = parser::ind2line(a).to_string();
                    let col = parser::ind2char(b).to_string();
                    enpassant = &[col,row].concat();
                } else{
                    enpassant = "-";
                }
            },
        }

        true
    }
}
