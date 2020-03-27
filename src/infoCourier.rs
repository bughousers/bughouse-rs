
///A module to read/generate FEN Strings
/// 
/// Bughousers has 2 Games running simultaneously
/// Therefore it generates one FEN String for each Game
pub mod infoCourier {
    use crate::logic::ChessLogic;
    use crate::logic::board;
    use crate::logic::board::ChessBoard;
    use crate::logic::board::Piece;
    use crate::parse::parser;
    use crate::parse::parser::line2line;
    use crate::parse::parser::char2ind;
    use crate::util::contains;
    use crate::logic::Winner;
    use std;

    ///# Arguments
    /// * `cl` - A pointer to a ChessLogic 
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

    /// Read from 2 FENs and 4 Deployable Piece Pools
    ///# Arguments
    /// * `s1` - FEN String board1
    /// * `s2` - FEN String board2
    /// * `p1` - list of pieces for board1,white 
    /// * `p2` - list of pieces for board1,black 
    /// * `p3` - list of pieces for board2,white 
    /// * `p4` - list of pieces for board2,black   
    ///
    /// Example fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    /// A pool string has to be String consisting of P,R,Q,N,B,p,r,q,n,b -> capital case for white, lower case for black
    /// The ordering is not important but it must be consistent (only white or only black)
    /// In normal fen --kq becomes -kq or ---- -> -, to make parsing easier this string is not trimmed
    pub fn read_fen(s1:& String,s2:& String,
        p1:&String,p2:&String,p3:&String,p4:&String ) -> Option<ChessLogic> {
       
        let mut splt1: Vec<&str> = s1.split(" ").collect();
        let mut splt2: Vec<&str> = s2.split(" ").collect();
        //positions (split on /)

        if splt1.len()!=6 || splt2.len()!=6{
            return None
        }
      
        //split the first string in "/"
        //and read the locations
        let mut ss = splt1[0];
        let mut locs1: Vec<&str> = ss.split("/").collect();
        ss = splt2[0];
        let mut locs2: Vec<&str> = ss.split("/").collect();

        
        let mut board1 = ChessBoard::new();
        let mut board2 = ChessBoard::new();

        for i in 0..7 {
            if !read_line(&(locs1[i].to_string()),&mut board1,i){
                return None;
            }
            if !read_line(&(locs2[i].to_string()),&mut board2,i){
                return None;
            }
        }

        //current turn (b/w)
        let turnstr1 = splt1[1].to_string();
        let mut turns: Vec<bool> = Vec::new();
        let turnstr2 = splt2[1].to_string();
        for i in [turnstr1,turnstr2].iter(){
            if *i=="b".to_string(){
                turns.push(false);
            }else if *i=="w".to_string(){
                turns.push(true);
            }else{
                return None;
            }
        }


        //castling rights (KQkq)
        //actually rooks can be assigned only once
        //but the match statements prevents from 
        //being it secure
        //only applies with the fen precondition 
        //so they are mutable
        let mut white_king_moved1:bool=false;
        let mut white_king_moved2:bool=false;
        let mut black_king_moved1:bool=false;
        let mut black_king_moved2:bool=false;
        let mut white_rook_k_moved1:bool=false;
        let mut white_rook_q_moved1:bool=false;
        let mut white_rook_k_moved2:bool=false;
        let mut white_rook_q_moved2:bool=false;
        let mut black_rook_k_moved1:bool=false;
        let mut black_rook_q_moved1:bool=false;
        let mut black_rook_k_moved2:bool=false;
        let mut black_rook_q_moved2:bool=false;
        println!("test1");
            for c in splt1[2].chars() {
                match c {
                    'K' => {
                        white_king_moved1=false;
                        white_rook_k_moved1=false;
                    },
                    'Q' => {
                        white_king_moved1=false;
                        white_rook_q_moved1=false;
                    },
                    'k' => {
                        black_king_moved1=false;
                        black_rook_k_moved1=false;
                    },
                    'q' => {
                        black_king_moved1=false;
                        black_rook_q_moved1=false;
                    },
                    '-' => {
                        if board1.board[7][7]==Piece::R{
                            white_king_moved1=true;
                        }else{
                            white_rook_k_moved1=true;
                        }
                        if board1.board[7][0]==Piece::R{
                            white_king_moved1=true;
                        }else{
                            white_rook_q_moved1=true;
                        }
                        if board1.board[0][7]==Piece::r{
                            black_king_moved1=true;
                        }else{
                            black_rook_k_moved1=true;
                        }
                        if board1.board[0][0]==Piece::r{
                            black_king_moved1=true;
                        }else{
                            black_rook_q_moved1=true;
                        }
                    },
                     _ =>return None,
                }
            }

            for c in splt2[2].chars() {
                match c {
                    'K' => {
                        white_king_moved2=false;
                        white_rook_k_moved2=false;
                    },
                    'Q' => {
                        white_king_moved2=false;
                        white_rook_q_moved2=false;
                    },
                    'k' => {
                        black_king_moved2=false;
                        black_rook_k_moved2=false;
                    },
                    'q' => {
                        black_king_moved2=false;
                        black_rook_q_moved2=false;
                    },
                    '-' => {
                        if board2.board[7][7]==Piece::R{
                            white_king_moved2=true;
                        }else{
                            white_rook_k_moved2=true;
                        }
                        if board2.board[7][0]==Piece::R{
                            white_king_moved2=true;
                        }else{
                            white_rook_q_moved2=true;
                        }
                        if board2.board[0][7]==Piece::r{
                            black_king_moved2=true;
                        }else{
                            black_rook_k_moved2=true;
                        }
                        if board2.board[0][0]==Piece::r{
                            black_king_moved2=true;
                        }else{
                            black_rook_q_moved2=true;
                        }
                    },
                     _ =>return None,
                }
            }

        //last pawn (e3)
        let lastpawns = [splt1[3].to_string(),splt2[3].to_string()];
        let mut _lastpawns = Vec::new();
        let mut f = true;
        let mut a = 0; let mut b = 0;
        for s in lastpawns.iter(){
            for c in s.chars(){
                if c == '-' {
                    _lastpawns.push(None);
                }else{
                    if f {
                        if let Some(i) = char2ind(c) {
                            a = i;
                        }else{
                            a = 8;
                        }
                    }else{
                        f = !f;
                        if let Some(i) = line2line(c) {
                            b = i;
                        }else{
                            b = 8;
                        }
                    }
                   
                }
            }
            if a!=8 && b!=8{
                _lastpawns.push(Some((a,b)));
            }else{
                _lastpawns.push(None);
            }
            
          
        }

        //halfturns (0)
        let halfturns1;
        if let Some(x) = parse_int_str(&splt1[4].to_string()){
            halfturns1 = x;
        }else{
            return None;
        }

        let halfturns2;
        if let Some(x) = parse_int_str(&splt2[4].to_string()){
            halfturns2 = x;
        }else{
            return None;
        }
        //fullturns (1)

        let fullturns1;
        if let Some(x) = parse_int_str(&splt2[4].to_string()) {
            fullturns1 = x;
        }else{
            return None;
        }
        let fullturns2;
        if let Some(x) = parse_int_str(&splt2[5].to_string()){
            fullturns2 =x;
        }else{
            return None;
        }

        //parse pool1 -b1w
        let pool11;
        if let Some(x) = parse_pool(p1){
            pool11 = x;
        }else{
            pool11 = [0;5];
        }
        //pool2 -b1b
        let pool12;
        if let Some(x) =parse_pool(p2) {
            pool12 =x;
        }else{
            pool12 = [0;5];
        }
        //pool3 -b2w
        let pool21;
        if let Some(x) =parse_pool(p3) {
            pool21 =x;
        }else{
            pool21 = [0;5];
        }
        //pool4 -b2b
        let pool22;
        if let Some(x) =parse_pool(p4) {
            pool22 =x;
        }else{
            pool22 = [0;5];
        }

        board1.white_k_moved=white_king_moved1;
        board1.black_k_moved=black_king_moved1;
        board1.white_rook_q_moved=white_rook_q_moved1;
        board1.white_rook_k_moved=white_rook_k_moved1;
        board1.black_rook_q_moved=black_rook_q_moved1;
        board1.black_rook_k_moved=black_rook_k_moved1;
        board2.white_k_moved=white_king_moved2;
        board2.black_k_moved=black_king_moved2;
        board2.white_rook_q_moved=white_rook_q_moved2;
        board2.white_rook_k_moved=white_rook_k_moved2;
        board2.black_rook_q_moved=black_rook_q_moved2;
        board2.black_rook_k_moved=black_rook_k_moved2;
        return Some(
            ChessLogic::resume(
            board1, board2,
            _lastpawns[0],_lastpawns[1],
            Piece::E,Piece::E,
            turns[0],turns[1],
            halfturns1,halfturns2,
            fullturns1,fullturns2,
            pool11,pool12,
            pool21,pool22,
            Winner::N)
        )
    }
    
    ///Reads a pool string, returns none if input is not legal
    ///# Arguments
    /// * `st` - A String for deployable piece pool
    fn parse_pool(st: &String) -> Option<[u8;5]> {
        let mut ar = [0;5];
        let mut upperfound = false;
        let mut lowerfound = false;
        for c in st.chars() {
            match c {
                'P' => ar[0]+=1,
                'R' => ar[1]+=1,
                'N' => ar[2]+=1,
                'B' => ar[3]+=1,
                'Q' => ar[4]+=1,
                'p' => ar[0]+=1,
                'r' => ar[1]+=1,
                'n' => ar[2]+=1,
                'n' => ar[3]+=1,
                'q' => ar[4]+=1,
                _ => return None
            }
        }
        if upperfound && lowerfound {
            return None
        }
        return Some(ar)
    }

    ///Parse a legal int string to an integer, returns None if input is legal
    /// # Arguments
    /// * `st` - A string that is a valid decimal number
    fn parse_int_str(st: &String) -> Option<usize> {
        let mut amn = st.chars().count() as u32;
        let mut intega: u32 = 1;
        for c in st.chars() {
            if let Some(x) = c.to_digit(10){
                intega += 10u32.pow(amn)*x;
                amn -=1;
            }else{
                return None
            }
        }
        return Some(intega as usize);
    }

    ///Parse a FEN substring for piece locations, false if input is not legal
    /// # Arguments
    /// * `s1` - A string that is a valid FEN Substring for pieces
    /// * `ch` - A pointer to a board, for saving the piece locations
    /// * `line` - index for the current line
    fn read_line(s1: & String,ch: &mut ChessBoard,line:usize) -> bool{
        for c in s1.chars() {
            let mut ct = 0;
            if let Some(num) = c.to_digit(10) {
                //somany emptiness
                for i in 0..num {
                    ch.board[line][ct] = Piece::E;
                    ct+=1;
                }
            }else{
                //then it has to be one of the pieces
                match c {
                    'p' => {ch.board[line][ct]=Piece::p},
                    'r' => {ch.board[line][ct]=Piece::r},
                    'q' => {ch.board[line][ct]=Piece::q},
                    'n' => {ch.board[line][ct]=Piece::n},
                    'b' => {ch.board[line][ct]=Piece::b},
                    'k' => {ch.board[line][ct]=Piece::k},
                    'P' => {ch.board[line][ct]=Piece::P},
                    'R' => {ch.board[line][ct]=Piece::R},
                    'Q' => {ch.board[line][ct]=Piece::Q},
                    'N' => {ch.board[line][ct]=Piece::N},
                    'B' => {ch.board[line][ct]=Piece::B},
                    'K' => {ch.board[line][ct]=Piece::K},
                    _ =>  return false,
                }
                ct+=1;
            }
            
        }
        return true
    }

    ///Generates string output of a pool
    ///# Arguments
    /// ´p´ - A deployable piece pool
    /// ´white´ - wether the pieces are white
    fn gen_pool(p:&[u8;5],white:bool) -> String {
        let mut st = "".to_string();
        for i in 0..p[0] {
           if white {
            st.push('P');
           } else{
               st.push('p');
           }
           
        }
        for i in 0..p[1] {
            if white {
                st.push('R');
               } else{
                   st.push('r');
               }
            
        }
        for i in 0..p[2] {
            if white {
                st.push('N');
               } else{
                   st.push('n');
               }
            
        }
        for i in 0..p[3] {
            if white {
                st.push('B');
               } else{
                   st.push('b');
               }
            
        }
        for i in 0..p[4] {
            if white {
                st.push('Q');
               } else{
                   st.push('Q');
               }
            
        }
        st
    }
}
