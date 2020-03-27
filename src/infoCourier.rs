
///A module to read/generate FEN Strings
/// 
/// Bughousers has 2 Games running simultaneously
/// Therefore it generates one FEN String for each Game
pub mod infoCourier {
    use crate::logic::ChessLogic;
    use crate::logic::board;
    use crate::logic::board::Piece;
    use crate::parse::parser;
    use crate::util::contains;
    use std;

    ///Generates the FEN String from a given Bughouse game: a given Chesslogic module
    pub fn gen_fen(cl:& ChessLogic) -> (String,String) {
        //pieces
        //active color
        //castling rights KQkq
        //en passant : behind the the pawn that has moved 2 squares
        //halfmove clock : num halfmoves after last capture
        //number of full moves

        //create for board1
        let mut s1 = "".to_string();
        let mut s2 = "".to_string();
        //get pieces 
       
        for board1 in [true,false].iter() {

            let mut pieces = "".to_string();
            let mut emptyblock = 0;
            let mut sum = 0;

            for i in 0..8 {
                for j in 0..8 {
                    let p = cl.get_piece(*board1,i,j);
                    if p==Piece::E {
                        emptyblock += 1;
                    }else{
                        let a = emptyblock.to_string();
                        if emptyblock!=0{
                            pieces = format!("{}{}{}",pieces,emptyblock,p);
                        }else{
                            pieces = format!("{}{}",pieces,p);
                        }
                        sum += emptyblock;
                        emptyblock=0;
                    }
                }
                if emptyblock == 8{
                    let a = emptyblock.to_string();
                    pieces = format!("{}{}",pieces,emptyblock);
                    emptyblock = 0;
                }else if emptyblock !=0{
                    let a = (8 - emptyblock).to_string();
                    pieces = format!("{}{}",pieces,emptyblock);
                    emptyblock = 0;
                }
                emptyblock=0;

                if i!=7 {
                    pieces = format!("{}{}",pieces,"/".to_string());
                }
            }
            
            
            //get active color
            let mut activecol = 'w'.to_string();
            match cl.get_white_active(*board1) {
                true =>{ activecol = 'w'.to_string();},
                false =>{ activecol = 'b'.to_string();},
            }
            //check for castling rights

            //check for en passant
            let mut enpassant = "".to_string();
            match cl.get_pawn_in_last_turn(*board1) {
                None => {enpassant = "-".to_string();},
                Some((a,b)) => {
                    if a == 3 {
                        if let Some(col) = parser::ind2char(b) {
                            col.to_string();
                            enpassant = format!("{}{}",enpassant,col);
                        }
                        if let Some(row) = parser::ind2line(a-1) {
                            row.to_string();
                            enpassant = format!("{}{}",enpassant,row);
                        }
                    }else if a == 4 {
                        if let Some(col) = parser::ind2char(b) {
                            col.to_string();
                            enpassant = format!("{}{}",enpassant,col);
                        }
                        if let Some(row) = parser::ind2line(a+1) {
                            row.to_string();
                            enpassant = format!("{}{}",enpassant,row);
                        }
                    } 
                    else{
                        enpassant = "-".to_string();
                    }
                },
            }

            //get if king has moved 
            let x1 = cl.get_castling_rights(*board1);
            let mut castling = "".to_string();
                if x1[0] {
                    castling = format!("{}{}", castling, "K".to_string());
                }else{
                    castling = format!("{}{}", castling, "-".to_string());
                }

                if x1[1] {
                    castling = format!("{}{}", castling, "Q".to_string());
                }else{
                    castling = format!("{}{}", castling, "-".to_string());
                }

                if x1[2] {
                    castling = format!("{}{}", castling, "k".to_string());
                }else{
                    castling = format!("{}{}", castling, "-".to_string());
                }

                if x1[3] {
                    castling = format!("{}{}", castling, "q".to_string());
                }else{
                    castling = format!("{}{}", castling, "-".to_string());
                }


            //get halfturns
            let mut halfturns = cl.get_half_moves(*board1).to_string();
            //get fullturns
            let mut fullturns = cl.get_movectr(*board1).to_string();

            let mut x = format!("{} {} {} {} {} {}",
            pieces, activecol,castling,enpassant,
            halfturns,fullturns);

            if *board1 { s1 = x; } else {s2 = x; }
        }

        (s1,s2)
    }


    pub fn read_fen(s1:& String,s2:& String) -> ChessLogic {
        return ChessLogic::new();
    }
}
