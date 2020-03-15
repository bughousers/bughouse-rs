pub mod xfen {
    use crate::logic::ChessLogic;
    use crate::logic::board;
    use crate::logic::board::Piece;
    use crate::parse::parser;
    use std;

    pub fn gen_xfen(cl:&ChessLogic) -> bool {
        //pieces
        //active color
        //castling rights KQkq
        //en passant : behind the the pawn that has moved 2 squares
        //halfmove clock : num halfmoves after last capture
        //number of full moves

        //create for board1

        //get pieces 

        let mut board1 = true;
        //get active color
        let mut activecol = 'w';
        match cl.get_white_active(true) {
            true =>{ activecol = 'w'},
            false =>{ activecol = 'b'},
        }
        //check for castling rights

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

        //check for castling
        let mut castling = "";
        match find_piece(Piece::K,board1,cl) {
            None => {
                 //castling.push_str("-");
                 //castling.push_str("-");
            },
            Some((a,b)) => {},
        }

        

        //get halfturns
        let mut halfturns = cl.get_half_moves(board1).to_string();
        //get fullturns
        let mut fullturns = cl.get_movectr(board1).to_string();
        true
    }

    fn contains_tpl(vec: &Vec<(usize,usize)>,(i,j): (usize,usize)) -> bool {
        for (a,b) in vec.iter() {
            if *a==i && j==*b {
                return true
            }
        }
        return false
    }

    fn find_piece(p: Piece,board1:bool, cl:&ChessLogic) -> Option<(usize,usize)> {
        for i in 0..8 {
            for j in 0..8 {
                if cl.chess_board1.board[i][j] == p {
                   return Some((i,j))
                }
            }
        }
        None 
    }
}
