pub mod board;
pub mod tests;
use crate::logic::board::ChessBoard;
use crate::logic::board::Piece;
use std::cmp;



#[derive(Clone, Copy, PartialEq)]
pub enum Winner {
    W1,B1,N,P,W2,B2,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MoveError {
    NotLegal,NotTurn,CannotDeploy,AlreadyOver,NoPieceInPool,
    PromotionProblem,
}

pub struct ChessLogic {
    pub chess_board1: ChessBoard, //board1
    pub chess_board2: ChessBoard, //board2
    //tuple of start point offset and if bool to indicate 
    //if it has moved
    pawn_in_last_turn_b1: Option<(usize,usize)>, //location of last pawn move, board1
    pawn_in_last_turn_b2: Option<(usize,usize)>, //location of last pawn move, board2
    white_active_1: bool, //true if white is active, board1
    white_active_2: bool, //true if white is active, board2
    //boardX_color_capture ==
    //board1 or 2, the captured piece has color <color>
    //the order is P-R-N-B-Q
    pub upgrade_to1: Piece, //check if pawn should be upgrade, board1
    pub upgrade_to2: Piece, //"", board2
    half_moves_last_capture1: usize, //count of half moves since last pawn move,captur, board1
    half_moves_last_capture2: usize, //"", board2
    movectr1: usize, //count of turns, board1
    movectr2: usize, //"",board2
    board1_white_capture: [u8;5], //pieces that can be deployed on board1 (white pieces)
    board1_black_capture: [u8;5], //"",board1,black
    board2_white_capture: [u8;5], //"",board2,white
    board2_black_capture: [u8;5], //"",board2,black
    winner: Winner,
}

impl ChessLogic {

    //sets both games to initial chess state
    pub fn refresh(&mut self){
        self.chess_board1.set_init_array();
       
        self.chess_board1.white_k_moved = false;
        self.chess_board1.black_k_moved = false; 
        self.chess_board1.white_rook_q_moved = false; 
        self.chess_board1.white_rook_k_moved = false; 
        self.chess_board1.black_rook_q_moved = false; 
        self.chess_board1.black_rook_k_moved = false; 

        self.chess_board2.set_init_array();

        self.chess_board2.white_k_moved = false; 
        self.chess_board2.black_k_moved = false; 
        self.chess_board2.white_rook_q_moved = false; 
        self.chess_board2.white_rook_k_moved = false; 
        self.chess_board2.black_rook_q_moved = false; 
        self.chess_board2.black_rook_k_moved = false; 

        self.pawn_in_last_turn_b1 = None;  
        self.pawn_in_last_turn_b2 = None; 
        self.upgrade_to1 = Piece::E; 
        self.upgrade_to2 = Piece::E; 
        self.white_active_1 = true; 
        self.white_active_2 = true; 
        self.half_moves_last_capture1 = 0; 
        self.half_moves_last_capture2 = 0; 
        self.movectr1 = 1; 
        self.movectr2 = 1; 
        self.board1_white_capture = [0;5]; 
        self.board1_black_capture = [0;5]; 
        self.board2_white_capture = [0;5]; 
        self.board2_black_capture = [0;5]; 
        self.winner= Winner::N; 
    }

    //prints one of the boards
    pub fn print(&self,board1:bool){
        if board1{
            self.chess_board1.print_board();
            println!("------------------------");
        }else{
            self.chess_board2.print_board();
            println!("------------------------");
        }

    }

    //return if the king has moved
    pub fn get_if_king_moved(&self,board1:bool,white:bool) -> bool{
        if board1 {
            if white{
                self.chess_board1.white_k_moved
            }  else{
                self.chess_board1.black_k_moved
            }         
        }else{
            if white{
               self.chess_board2.white_k_moved
            }else{
                self.chess_board2.black_k_moved
            }
        }
    }

    //returns capture pools
    pub fn get_pools(&self) -> ([u8;5],[u8;5],[u8;5],[u8;5]){
        (
            self.board1_white_capture, 
            self.board1_black_capture, 
            self.board2_white_capture, 
            self.board2_black_capture
        )
    }

    //get a bool that is true if white is active
    pub fn get_white_active(&self, board1:bool) -> bool{
        match board1 {
            true => self.white_active_1,
            false => self.white_active_2,
        }
    }

    //get number of movements
    pub fn get_movectr(&self, board1:bool) -> usize {
        match board1 {
            true => self.movectr1,
            false => self.movectr2,
        }
    }

    //get half moves since last capture/pawn movement
    pub fn get_half_moves(&self, board1:bool) -> usize {
        match board1 {
            true => self.half_moves_last_capture1,
            false => self.half_moves_last_capture2,
        }
    }

    //returns captures pieces, tandem
    pub fn get_captured_piece(&self, board1:bool,white:bool, i:usize) -> u8 {
        if board1 {
            if white { 
                self.board1_white_capture[i]
            }else{
                self.board1_black_capture[i]
            }
        }else{
            if white {
                self.board2_white_capture[i]
            }else{
                self.board2_black_capture[i]
            }
        }
    }

        //returns captures pieces, tandem
        fn set_captured_piece(&mut self, board1:bool,white:bool, i:usize,inc:bool){
            if board1 {
                if white { 
                   if !inc {
                    self.board1_white_capture[i]-=1;
                   } else{
                    self.board1_white_capture[i]+=1;
                   }
                }else{
                    if !inc {
                        self.board1_black_capture[i]-=1;
                       } else{
                        self.board1_black_capture[i]+=1;
                       }
                }
            }else{
                if white {
                    if !inc {
                        self.board2_white_capture[i]-=1;
                       } else{
                        self.board2_white_capture[i]+=1;
                       }
                }else{
                    if !inc {
                        self.board2_black_capture[i]-=1;
                       } else{
                        self.board2_black_capture[i]+=1;
                       }
                }
            }
        }

    //needed for enpassant encoding
    pub fn get_pawn_in_last_turn(&self, board1:bool) -> Option<(usize,usize)> {
        match board1 {
            true => self.pawn_in_last_turn_b1,
            false => self.pawn_in_last_turn_b2,
        }
    }

    //return the winner type (white,black,none ..)
    pub fn get_winner(&self, board1:bool) -> Winner {
        self.winner
    }

    //ONLY FOR TESTING
    pub fn set_pawn_in_last_turn(&mut self, board1:bool, x:Option<(usize,usize)>) {
        match board1 {
            true => self.pawn_in_last_turn_b1 = x,
            false => self.pawn_in_last_turn_b2 = x,
        }
    }

    //print legal moves with input of legal moves
    pub fn print_w_legal(&mut self,board1:bool,locs: &Vec<(usize,usize)>){
        //its highly probably that a piece will have 8-16 legal moves
        let mut vec = Vec::with_capacity(8);
        for &(i,j) in locs.iter() {
            if self.get_piece(board1,i,j)== Piece::E {
                //legal moves are encoded with 'L'
                if board1 {
                    self.chess_board1.board[i][j] = Piece::L;
                }else{
                    self.chess_board2.board[i][j] = Piece::L;
                }
            }else{
                vec.push((i,j,self.get_piece(board1,i,j)));
                //legal moves are encoded with 'L'
                if board1{
                    self.chess_board1.board[i][j] = Piece::L;
                }else{
                    self.chess_board2.board[i][j] = Piece::L;
                }
               
            }
        }
        //print the boards with legal moves as L
        if board1 {
            self.chess_board1.print_board();
        }else{
            self.chess_board2.print_board();
        }

        //set back the legal moves 'L' with their old pieces
        for &(i,j) in locs.iter() {
            // L -> E
            if board1 {self.chess_board1.board[i][j] = Piece::E;} 
            else {self.chess_board2.board[i][j] = Piece::E;}
        }
        for &(i,j,t) in vec.iter() {
            // non L -> revert to old Piece
            if board1 {
                self.chess_board1.board[i][j] = t;
            }else{
                self.chess_board2.board[i][j] = t;
            }
          
        }
        println!("------------------------");
    }

    pub fn new() -> ChessLogic {
        ChessLogic{
            chess_board1: ChessBoard::new(), 
            chess_board2: ChessBoard::new(), 
            pawn_in_last_turn_b1: None, 
            pawn_in_last_turn_b2: None,
            upgrade_to1: Piece::E,
            upgrade_to2: Piece::E,
            white_active_1: true,
            white_active_2: true,
            half_moves_last_capture1: 0,
            half_moves_last_capture2: 0,
            movectr1: 1,
            movectr2: 1,
            board1_white_capture: [0;5],
            board1_black_capture: [0;5],
            board2_white_capture: [0;5],
            board2_black_capture: [0;5],
            winner: Winner::N,
        }
    }

    //returns a vector of legal moves for the board, and for the location
    pub fn get_legal_moves(&mut self,board1:bool, old_i:usize, old_j:usize)
    -> Vec<(usize,usize)>
    {
        match self.get_piece(board1,old_i,old_j) {
            Piece::P  => {
                match (old_i,old_j) {
                    //unmoved (insert double move too -- Double move)
                    (6,_) =>  {
                        //max 4 moves
                        let mut vec = Vec::with_capacity(2);
                        if self.is_empty(board1,old_i-1,old_j) {
                            vec.push((old_i-1,old_j));
                            // -2 only if -1 is empty
                            if self.is_empty(board1,old_i-2,old_j) {
                                vec.push((old_i-2,old_j));
                            }
                        }
                        //enemy piece on -1,-1
                        if old_j > 0 && self.is_enemy(board1,true,old_i-1,old_j-1) {
                            vec.push((old_i-1,old_j-1));
                        }
                        //enemy piece on -1,+1
                        if old_j < 7 && self.is_enemy(board1,true,old_i-1,old_j+1) {
                            vec.push((old_i-1,old_j+1));
                        }
                        vec
                    },
                    //6th lane with en passant
                    (3,_) => {
                        //max 5 moves
                        let mut vec = Vec::with_capacity(2);
                        if self.is_empty(board1,old_i-1,old_j) {
                            vec.push((old_i-1,old_j));
                        }
                        if old_j > 0 && self.is_enemy(board1,true,old_i-1,old_j-1) {
                            vec.push((old_i-1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,true,old_i-1,old_j+1) {
                            vec.push((old_i-1,old_j+1));
                        }
                        //en passant from left
                        if old_j > 0 && self.get_piece(board1,old_i,old_j-1) == Piece::p {
                            match self.get_pawn_in_last_turn(board1) {
                                Some((a,b)) => vec.push((a-1,b)),
                                _ => (),
                            }
                        }
                        //en passant from right
                        if old_j < 7 && self.get_piece(board1,old_i,old_j+1) == Piece::p {
                            match self.get_pawn_in_last_turn(board1) {
                                Some((a,b)) => vec.push((a-1,b)),
                                _ => (),
                            }
                        }
                        vec


                    },
                    //there can be no white pawns on line
                    (0,_) => {
                       let mut vec =  Vec::new();
                       println!("This indicates pawn upgrade malfunction");
                       vec
                    },
                    //no special moves
                    (_,_) => {
                        //max 4 moves
                        let mut vec = Vec::with_capacity(2);
                        if self.is_empty(board1,old_i-1,old_j) {
                            vec.push((old_i-1,old_j));
                        }
                        if old_j > 0 && self.is_enemy(board1,true,old_i-1,old_j-1) {
                            vec.push((old_i-1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,true,old_i-1,old_j+1) {
                            vec.push((old_i-1,old_j+1));
                        }
                        vec
                    },
                }
            },
            Piece::p  => {
                //same for the black
                match (old_i,old_j) {
                    //double moves
                    (1,_) =>  {
                        //max 4 moves
                        let mut vec = Vec::with_capacity(2);
                        //W double move
                        if self.is_empty(board1,old_i+1,old_j) {
                            vec.push((old_i+1,old_j));
                            if self.is_empty(board1,old_i+2,old_j) {
                                vec.push((old_i+2,old_j));
                            }
                        }
                        //enemy left above
                        if old_j > 0 && self.is_enemy(board1,false,old_i+1,old_j-1) {
                            vec.push((old_i+1,old_j-1));
                        }
                        //enemy right above
                        if old_j < 7 && self.is_enemy(board1,false,old_i+1,old_j+1) {
                            vec.push((old_i+1,old_j+1));
                        }
                        vec
                    },
                    //4th line with enpassant 
                    (4,_) => {
                            //max 5 moves
                            let mut vec = Vec::with_capacity(2);
                            //front empty
                            if self.is_empty(board1,old_i+1,old_j) {
                                vec.push((old_i+1,old_j));
                            }
                            //enemy left above
                            if old_j > 0 && self.is_enemy(board1,true,old_i+1,old_j-1) {
                                vec.push((old_i+1,old_j-1));
                            }
                            //enemy right above
                            if old_j < 7 && self.is_enemy(board1,true,old_i+1,old_j+1) {
                                vec.push((old_i+1,old_j+1));
                            }
                            //en passant, enemy left
                            if old_j > 0 && self.get_piece(board1,old_i,old_j-1) == Piece::P {
                                match self.get_pawn_in_last_turn(board1){
                                    Some((a,b)) => vec.push((a+1,b)),
                                    _ => (),
                                }
                            }
                            //en passant, enemy right
                            if old_j < 7 && self.get_piece(board1,old_i,old_j+1) == Piece::P {
                                match self.get_pawn_in_last_turn(board1) {
                                    Some((a,b)) => vec.push((a+1,b)),
                                    _ => (),
                                }
                            }
                            vec
    
                    },
                    //no pawn can be at the 7th line, promotion!
                    (7,_) => {
                        let mut vec = Vec::new();
                        println!("This indicates pawn upgrade malfunction");
                        vec
                    },  
                    //no special moves
                    (_,_) => {
                        let mut vec = Vec::new();
                        if self.is_empty(board1,old_i+1,old_j) {
                            vec.push((old_i+1,old_j));
                        }
                        if old_j > 0 && self.is_enemy(board1,false,old_i+1,old_j-1) {
                            vec.push((old_i+1,old_j-1));
                        }
                        if old_j < 7 && self.is_enemy(board1,false,old_i+1,old_j+1) {
                            vec.push((old_i+1,old_j+1));
                        }
                        vec
                    },
                }
            },
            Piece::R | Piece::r | Piece::Ur | Piece::UR => self.cross_mov(board1,old_i,old_j),
            Piece::B | Piece::b | Piece::UB | Piece::Ub => self.x_mov(board1,old_i,old_j),
            Piece::Q | Piece::q | Piece::UQ | Piece::Uq => 
            { let mut vec = self.cross_mov(board1,old_i,old_j);
              vec.append(&mut self.x_mov(board1,old_i,old_j));
              vec },
            Piece::N | Piece::n |Piece::UN | Piece::Un=> self.horse_jump(board1,old_i,old_j),
            Piece::K | Piece::k => self.king_move(board1,old_i,old_j),
            //should not come to here
             _ => {println!("This indicates a bug!"); Vec::new()},
        }
    }

    //returns if the location on the given board is empty
    fn is_empty(&self, board1:bool, i:usize, j:usize) -> bool {
        if board1 {
            match self.chess_board1.board[i][j] {
                Piece::E => true, 
                _ => false,
            }
        }else{
            match self.chess_board2.board[i][j] {
                Piece::E => true, 
                _ => false,
            }
        }
    }

    //returns if the location on the given board is an enemy piece (needs to know if the caller is white)
    fn is_enemy(&self, board1:bool,white:bool, i:usize, j:usize) -> bool {
            if white {
                //true for black, false for white
                match  self.get_board_n(board1).board[i][j] {
                    Piece::E => false,
                    Piece::L => false,
                    Piece::P  => false,
                    Piece::R | Piece::UR => false,
                    Piece::N | Piece::UN => false,
                    Piece::B | Piece::UB => false,
                    Piece::Q | Piece::UQ => false,
                    Piece::K => false,
                    _ => true,
    
                }
            }else {
                //true for white, false for black
                match self.get_board_n(board1).board[i][j] {
                    Piece::E => false,
                    Piece::L => false,
                    Piece::p  => false,
                    Piece::r | Piece::Ur => false,
                    Piece::n | Piece::Un => false,
                    Piece::b | Piece::Ub => false,
                    Piece::q | Piece::Uq => false,
                    Piece::k => false,
                    _ => true,
    
                }
            }
       
    }

    //return bool==given piece is white==true else false
    fn is_white(&self, board1:bool, i:usize, j:usize) -> bool {
            match self.get_board_n(board1).board[i][j] {
                Piece::E => false,
                Piece::L => false,
                Piece::p  => false,
                Piece::r | Piece::Ur => false,
                Piece::n | Piece::Un => false,
                Piece::b | Piece::Ub => false,
                Piece::q | Piece::Uq => false,
                Piece::k  => false,
                _ => true,
            }
    }

    //gets legal moves on a line
    fn horizontal_mov(&self,board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = Vec::with_capacity(4);
        let mut jc = j;
        while jc > 0 {
            jc -= 1;
            if self.is_empty(board1,i,jc) {
                vec.push((i,jc));
            }else {
                if self.is_enemy(board1,self.is_white(board1,i,j),i,jc) {
                    vec.push((i,jc));
                }
                break;
            }
        }
        jc = j;
        while jc < 7 {
            jc += 1;
            if self.is_empty(board1,i,jc) {
                vec.push((i,jc));
            }else {
                if self.is_enemy(board1,self.is_white(board1,i,j),i,jc) {
                    vec.push((i,jc));
                }
                break;
            }
        }
        vec
    }

      //gets legal moves on a column, horizontal_move but jc is changed with ic
    fn vertical_mov(&self,board1:bool, i:usize, j:usize)  -> Vec<(usize,usize)>{
        let mut vec = Vec::with_capacity(4);
        let mut ic = i;
        while ic > 0 {
            ic -= 1;
            if self.is_empty(board1,ic,j) {
                vec.push((ic,j));
            }else {
                if self.is_enemy(board1,self.is_white(board1,i,j),ic,j) {
                    vec.push((ic,j));
                }
                break;
            }
        }
        ic = i;
        while ic < 7 {
            ic += 1;
            if self.is_empty(board1,ic,j) {
                vec.push((ic,j));
            }else {
                if self.is_enemy(board1,self.is_white(board1,i,j),ic,j) {
                    vec.push((ic,j));
                }
                break;
            }
        }
        vec
    }

    //makes a cross with vertical and horizontal moves 
    fn cross_mov(&self,board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        let mut vec = self.vertical_mov(board1,i,j);
        vec.append(&mut self.horizontal_mov(board1,i,j));
        vec
    }

    //iterates for [+n,-n] ; [+n,-n] combinations 1..7 and gets legal moves
    fn x_mov(&self,board1:bool, i:usize, j:usize)  -> Vec<(usize,usize)>{
        let mut vec = Vec::with_capacity(4);
        let mut ic = i;
        let mut jc = j;
        let w = self.is_white(board1,i,j);

        //lower right
        while ic < 7 && jc < 7 {
            ic+=1;
            jc+=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,w,ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        //upper right
        ic=i;
        jc=j;
        while ic > 0 && jc < 7 {
            ic-=1;
            jc+=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,w,ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        //upper left 
        ic=i;
        jc=j;
        while ic > 0 && jc > 0 {
            ic-=1;
            jc-=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,w,ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        //lower left
        ic=i;
        jc=j;
        while ic < 7 && jc > 0 {
            ic+=1;
            jc-=1;
            if self.is_empty(board1,ic,jc) {
                vec.push((ic,jc));
            }else{
                if self.is_enemy(board1,w,ic,jc){
                    vec.push((ic,jc));
                }
                break;
            }
        }
        vec
    }

    //get possible horse moves, combinations of [+2,-2] ; [-1,+1] for all of the valids,
    //can jump
    fn horse_jump(&self, board1:bool, i:usize, j:usize) -> Vec<(usize,usize)> {
        //all possible combinations of +2, -2, +1, -1 for each square
        let twoway = [-2,2];
        let oneway = [1,-1];
        let mut vec = Vec::with_capacity(2);
        let w = self.is_white(board1,i,j);

        //first -2,2 then 1,-1
        for i_off in twoway.iter() {
            for j_off in oneway.iter() {
                let mut i_signed = i as i32;
                let mut j_signed = j as i32;
                i_signed += i_off;
                j_signed += j_off;
                //check if the indices are valid
                if i_signed >= 0 && i_signed <= 7 && j_signed >= 0 && j_signed <= 7 {
                    //cast back to usize
                    let itmp = i_signed as usize;
                    let jtmp = j_signed as usize;
                    
                    if self.is_empty(board1,itmp,jtmp) || 
                    self.is_enemy(board1,w,itmp,jtmp) {
                        vec.push((itmp,jtmp));
                    }
                   
                }
            }
        }
        //first 1,-1 then -2,2
        for i_off in oneway.iter() {
            for j_off in twoway.iter() {
                let mut i_signed = i as i32;
                let mut j_signed = j as i32;
                i_signed += i_off;
                j_signed += j_off;
                if i_signed >= 0 && i_signed <= 7 && j_signed >= 0 && j_signed <= 7 {
                    let itmp = i_signed as usize;
                    let jtmp = j_signed as usize;
                    if self.is_empty(board1,itmp,jtmp) || 
                    self.is_enemy(board1,w,itmp,jtmp) {
                        vec.push((itmp,jtmp));
                    }
                }
            }
        }
        vec
    }

    //returns the piece in given board and location
    pub fn get_piece(&self, chessboard1:bool, i:usize, j:usize) -> Piece {
        match chessboard1 {
            true => self.chess_board1.board[i][j],
            false => self.chess_board2.board[i][j],
        }
    }

    //converts the input piece to right color
    //also works for upgraded pieces
    pub fn get_piece_w_en(&self, iswhite:bool, piece:Piece) -> Piece {
        match piece {
            Piece::p | Piece::P => if iswhite {Piece::p} else {Piece::P} ,
            Piece::r | Piece::R | Piece::Ur | Piece::UR => if iswhite {Piece::r} else {Piece::R} ,
            Piece::b | Piece::B | Piece::UB | Piece::Ub => if iswhite {Piece::b} else {Piece::B} ,
            Piece::k | Piece::K => if iswhite {Piece::k} else {Piece::K} ,
            Piece::q | Piece::Q | Piece::Uq | Piece::UQ => if iswhite {Piece::q} else {Piece::Q} ,
            Piece::n | Piece::N | Piece::Un | Piece::UN => if iswhite {Piece::n} else {Piece::N} ,
            Piece::L => Piece::L,
            Piece::E => Piece::E,

        }
    }

    //converts the input piece to right color, but returns the upgraded if it exists
    //also works for upgraded pieces
    pub fn get_piece_w_upgrade_en(&self, iswhite:bool, piece:Piece) -> Piece {
        match piece {
            Piece::p | Piece::P => if iswhite {Piece::p} else {Piece::P} ,
            Piece::r | Piece::R | Piece::Ur | Piece::UR => if iswhite {Piece::Ur} else {Piece::UR} ,
            Piece::b | Piece::B | Piece::UB | Piece::Ub => if iswhite {Piece::Ub} else {Piece::UB} ,
            Piece::k | Piece::K => if iswhite {Piece::k} else {Piece::K} ,
            Piece::q | Piece::Q | Piece::Uq | Piece::UQ => if iswhite {Piece::Uq} else {Piece::UQ} ,
            Piece::n | Piece::N | Piece::Un | Piece::UN => if iswhite {Piece::Un} else {Piece::UN} ,
            Piece::L => Piece::L,
            Piece::E => Piece::E,

        }
    }

    //iswhite <=> the unit on the square is white
    pub fn is_attacked(&mut self, board1:bool, iswhite:bool, i:usize,j:usize) -> bool {
        //remove yourself from board (you could be blocking other places)
        //check for pawns
        //check for rook|queen on vertical
        //check for rook|queen on horizontal
        //check for bishop on x
        //check for horses
        //check for enemy king 

        let mut ic = (i as i32);
        let mut jc = (j as i32);
            //check for pawns
            if iswhite {
                if self.valid(ic-1,jc-1) {
                    let a = (ic-1) as usize;
                    let b = (jc-1) as usize;
                        if self.get_board_n(board1).board[a][b]==Piece::p
                        {
                   
                            return true
                        }
                }
                if self.valid(ic-1,jc+1) {
                    let a = (ic-1) as usize;
                    let b = (jc+1) as usize;
                        if self.get_board_n(board1).board[a][b]==Piece::p
                        {
                     
                            return true
                        }
                }
            }else{
                ic = (i as i32);
                jc = (j as i32);
                if self.valid(ic+1,jc-1) && self.valid(ic,jc) {
                    let a = (ic+1) as usize;
                    let b = (jc-1) as usize;
                        if self.get_board_n(board1).board[a][b]==Piece::P
                        {
                            println!("{},{}: is a piyon",a,b);
                            return true
                        }
                }
                if self.valid(ic+1,jc+1) && self.valid(ic,jc) {
                    let a = (ic+1) as usize;
                    let b = (jc+1) as usize;
                        if self.get_board_n(board1).board[a][b]==Piece::P
                        {
                     
                            return true
                        }
                }
            }
           
            //check for horizontal line
            jc = j as i32;
            ic = i as i32;
            let mut jx = jc+1;
            while jx < 8 {
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::r),ic,jx) || 
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jx) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::r),ic,jx) || 
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jx){
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jx){
                    //blocked
                    break;
                }
                jx+=1;
            }
            jx = jc-1;
            while jx >= 0{
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::r),ic,jx) || 
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jx) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::r),ic,jx) || 
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jx) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jx){
                    //blocked
                    break;
                }
                jx -=1;
            }
            //check for vertical
            let mut ix = ic+1;
            while ix <8 {
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::r),ix,jc) || 
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ix,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::r),ix,jc) || 
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ix,jc)
                {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ix,jc){
                    //blocked 
                    break;
                }
                ix+=1;
            }
            ix = ic-1;
            while ix >=0 {
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::r),ix,jc) || 
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ix,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::r),ix,jc) || 
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ix,jc)
                {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ix,jc){
                    break;
                }
                ix-=1;
            }
            //check for capraz
            ic=i as i32;
            jc=j as i32;
            //lower right
            while ic < 7 && jc < 7 {
                ic+=1;
                jc+=1;
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::b),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc) {
            
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
                
            }
            //upper right
            ic=i as i32;
            jc=j as i32;
            while ic > 0 && jc < 7 {
                ic-=1;
                jc+=1;
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::b),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
                
            }
            //upper left 
            ic=i as i32;
            jc=j as i32;
            while ic > 0 && jc > 0 {
                ic-=1;
                jc-=1;
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::b),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
                
            }
            //lower left
            ic=i as i32;
            jc=j as i32;
            while ic < 7 && jc > 0 {
                ic+=1;
                jc-=1;
                if self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_en(iswhite,Piece::b),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::q),ic,jc) ||
                self.check_for_piece(board1,self.get_piece_w_upgrade_en(iswhite,Piece::b),ic,jc) {
                    return true
                }else if !self.check_for_piece(board1,Piece::E, ic,jc){
                    break;
                }
            }
            //check for horses
            ic = i as i32;
            jc = j as i32;
            let a = [-2,2];
            let b = [-1,1];
            for i_off in a.iter() {
                for j_off in b.iter() {
                    if self.valid(ic+i_off,jc+j_off)  {
                        if board1 {
                            if self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_en(iswhite,Piece::n) ||
                            self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_upgrade_en(iswhite,Piece::n){
                                return true
                            }
                        }else{
                            if self.chess_board2.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_en(iswhite,Piece::n) ||
                            self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_upgrade_en(iswhite,Piece::n){
                                return true
                            }
                        }
                    }
                }
            }
            for j_off in a.iter() {
                for i_off in b.iter() {
                    if self.valid(ic+i_off,jc+j_off)  {
                        if board1 {
                            if self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_en(iswhite,Piece::n) || 
                            self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_upgrade_en(iswhite,Piece::n){
                                return true
                            }
                        }else{
                            if self.chess_board2.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_en(iswhite,Piece::n) ||
                            self.chess_board2.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                            self.get_piece_w_upgrade_en(iswhite,Piece::n) {
                                return true
                            }
                        }
                    }
                }
            }
            //check for enemy king
            let c = [-1,0,1];
            let d = [-1,0,1];
            for i_off in c.iter() {
                for j_off in d.iter() {
                    if self.valid(ic+i_off,jc+j_off) && !(*i_off==0 && *j_off==0) {
                        if self.chess_board1.board[(ic+i_off) as usize][(jc+j_off) as usize] == 
                        self.get_piece_w_en(iswhite,Piece::k) {
                            return true
                        }
                    }
                }
            }
        
        //nothing attacks this place
        false
    }

    //checks if the given index is in the legal bounds
    fn valid(&self, a: i32, b: i32) -> bool {
        if a >= 0 && a <= 7 && b >= 0 && b<= 7 {
            return true
        }else {
            return false
        }
    }

    //checks if the given location has the Piece piece
    fn check_for_piece(&self,board1:bool, piece:Piece, i : i32, j:i32) -> bool {
        if !self.valid(i,j){
            println!("The index is not valid, this idicates a bug");
            return false
        }else{
                if self.get_board_n(board1).board[i as usize][j as usize]==piece
                {
                    return true
                }else {
                    return false
                }
        }
    }

    //empties the whole board
    pub fn all_empty(&mut self,board1:bool) {
        for i in 0..8 {
            for j in 0..8 {
                if board1{
                    self.chess_board1.board[i][j] = Piece::E;
                }else{
                    self.chess_board2.board[i][j] = Piece::E;
                }
                
            }
        }
    }

  
    //sets the piece on the given location
    fn set_piece(&mut self,board1:bool,piece:Piece,i:usize,j:usize){
        if board1{
            self.chess_board1.board[i][j] = piece;
        }else{
            self.chess_board2.board[i][j] = piece;
        }
    }

    //check if the king can move
    fn king_move(&mut self, board1:bool, i:usize,j:usize) -> Vec<(usize,usize)>{
        let mut a = [-1,0,1];
        let mut b = [-1,0,1];
        let mut vec = Vec::with_capacity(2);
        let ix = i as i32;
        let jx = j as i32;
        let mut piece = Piece::E;
        let mut wayt = true;

        //remove yourself for is_attacked check if needed
        if board1 {
            piece = self.chess_board1.board[i][j];
            self.chess_board1.board[i][j] = Piece::E;
        }else{
            piece = self.chess_board2.board[i][j];
            self.chess_board2.board[i][j] = Piece::E;
        }

        //set white
        if piece == Piece::K {
            wayt = true;
        }else{
            wayt = false;
        }

        //check for all possible combinations
        for i_off in a.iter(){
            for j_off in b.iter(){
                if self.valid(i_off+ix,j_off+jx) && !(*i_off==0 && *j_off==0){
                    let mut ic = i_off+ix;
                    let mut jc = j_off+jx;
                    //if !self.is_attacked(board1,wayt,ic as usize,jc as usize)  
                    //keep this in case we want legal moves only
                    if true 
                    {
                        if board1 {
                            if self.chess_board1.board[ic as usize][jc as usize] == Piece::E 
                            || self.is_enemy(board1,wayt,ic as usize, jc as usize) {
                                vec.push((ic as usize,jc as usize));
                            }
                        }else{
                            if self.chess_board2.board[ic as usize][jc as usize] == Piece::E
                            || self.is_enemy(board1,wayt,ic as usize, jc as usize) {
                                vec.push((ic as usize,jc as usize));
                            }
                        }
                    }
                }

            }
        }

        //set yourself back on the field -- needed only for is_attacked tho
        if board1 {
            self.chess_board1.board[i][j] = piece;
        }else{
            self.chess_board2.board[i][j] = piece;
        }
      
        //check for castling
            if wayt {
                if !self.get_board(board1).white_k_moved 
                && self.get_board(board1).board[7][7] == Piece::R 
                && self.get_board(board1).board[7][6] == Piece::E
                && self.get_board(board1).board[7][5] == Piece::E
                && !self.is_attacked(board1,wayt,7,6) 
                && !self.is_attacked(board1,wayt,7,5)
                && !self.is_attacked(board1,wayt,7,4)
                && !self.get_board(board1).white_rook_k_moved
                {
                    vec.push((7,6))
                }
                if !self.get_board(board1).white_k_moved 
                && self.get_board(board1).board[7][0] == Piece::R 
                && self.get_board(board1).board[7][1] == Piece::E
                && self.get_board(board1).board[7][2] == Piece::E
                && self.get_board(board1).board[7][3] == Piece::E
                && !self.is_attacked(board1,wayt,7,3) 
                && !self.is_attacked(board1,wayt,7,2)
                && !self.is_attacked(board1,wayt,7,4)
                && !self.is_attacked(board1,wayt,7,1)
                && !self.get_board(board1).white_rook_q_moved {
                    vec.push((7,2))
                }
            }else{
                if !self.get_board(board1).black_k_moved 
                && self.get_board(board1).board[0][7] == Piece::r 
                && self.get_board(board1).board[0][6] == Piece::E
                && self.get_board(board1).board[0][5] == Piece::E
                && !self.is_attacked(board1,wayt,0,6) 
                && !self.is_attacked(board1,wayt,0,5)
                && !self.is_attacked(board1,wayt,0,4)
                && !self.get_board(board1).black_rook_k_moved
                {
                    vec.push((0,6))
                }
                if !self.get_board(board1).black_k_moved 
                && self.get_board(board1).board[0][0] == Piece::r 
                && self.get_board(board1).board[0][1] == Piece::E
                && self.get_board(board1).board[0][2] == Piece::E
                && self.get_board(board1).board[0][3] == Piece::E
                && !self.is_attacked(board1,wayt,0,3) 
                && !self.is_attacked(board1,wayt,0,2)
                && !self.is_attacked(board1,wayt,0,4)
                && !self.is_attacked(board1,wayt,0,1)
                && !self.get_board(board1).black_rook_q_moved {
                    vec.push((0,2))
                }
            }
        vec
    }

    pub fn get_board(&mut self, board1:bool )-> &mut ChessBoard{
        match board1 {
            true => &mut self.chess_board1,
            false => &mut self.chess_board2,
        }
    }

    pub fn get_board_n(&self, board1:bool )-> &ChessBoard{
        match board1 {
            true => &self.chess_board1,
            false => &self.chess_board2,
        }
    }

    //rly important detail
    //in tandem you can move your pinned piece, then the enemy can capture 
    //your king
    //checks if you can move from i_old, j_old to i,j
    pub fn legality_check(&mut self, board1:bool, i_old:usize,j_old:usize,i:usize,j:usize) -> bool{
        
        match &self.get_board(board1).board[i_old][j_old] {
            Piece::E | Piece::L => false,
            _ => { 
                let vec = self.get_legal_moves(board1,i_old,j_old);
                for (a,b) in vec.iter() {
                    if *a == i && *b == j {
                        return true
                    }
                }
                return false
            },
        }
    }

    //deploy a piece on your field as a turn 
    pub fn deploy_piece(&mut self,board1:bool,white:bool,p:Piece,i:usize,j:usize) -> Result<bool,MoveError> {
        //deploy the piece only if it is legal to play
        if self.winner!=Winner::N {
            return Ok(true)
        }

        //check for i j bounds
        if !self.valid(i as i32,j as i32) {
            return Err(MoveError::NotLegal)
        }

        //check if the location is empty
        if board1 {
            if self.get_board(board1).board[i][j] != Piece::E {
               return Err(MoveError::CannotDeploy)
            }

            if white {
                if !self.white_active_1 {  
                    return Err(MoveError::NotTurn)
                } else {
            
                    if let Some(ind) = self.box_index(p) {
                        if self.board1_white_capture[ind]>0 {
                            self.board1_white_capture[ind]-=1;
                        }else{
                            return Err(MoveError::NoPieceInPool)
                        }

                        self.chess_board1.board[i][j] = p;
                        self.white_active_1 = !self.white_active_1;

                        if p == Piece::P {
                            self.pawn_in_last_turn_b1 = Some((i,j));
                            self.half_moves_last_capture2=0;
                        }else{
                            self.pawn_in_last_turn_b1 = None;
                            self.half_moves_last_capture1+=1;
                        }

                        if p == Piece::R {
                            if i==7 && j==7 {
                                self.get_board(board1).white_rook_k_moved = false;
                            }
                            if i==7 && j==7 {
                                self.get_board(board1).white_rook_q_moved = false;
                            }
                        }

                        return Ok(true)
                    } else {return Err(MoveError::CannotDeploy)}
                }
            }else{
                if self.white_active_1 {
                     return Err(MoveError::CannotDeploy)
                }else {

                    if  let Some(ind) = self.box_index(p) {
                        if self.board1_black_capture[ind]>0 {
                            self.board1_black_capture[ind]-=1;
                        }else{
                            return Err(MoveError::NoPieceInPool)
                        }

                        self.chess_board1.board[i][j] = p;
                        self.white_active_1 = !self.white_active_1;

                        if p == Piece::p {
                            self.pawn_in_last_turn_b1 = Some((i,j));
                            self.half_moves_last_capture1=0;
                        }else{
                            self.pawn_in_last_turn_b1 = None;
                            self.half_moves_last_capture1+=1;
                        }

                        if p == Piece::r {
                            if i==0 && j==7 {
                                self.get_board(board1).black_rook_k_moved = false;
                            }
                            if i==0 && j==0 {
                                self.get_board(board1).black_rook_q_moved = false;
                            }
                        }
                        self.movectr1+=1;
                        return Ok(true)
                    } else {return Err(MoveError::CannotDeploy)}
                }   
            } 
        }else {
            if self.chess_board2.board[i][j] != Piece::E {
                return Err(MoveError::CannotDeploy)
            }

            if white {
                if !self.white_active_2 {  
                    return Err(MoveError::CannotDeploy)
                } else {
             
                    if let Some(ind) = self.box_index(p) {
                        if self.board2_white_capture[ind]>0 {
                            self.board2_white_capture[ind]-=1;
                        }else{
                           return Err(MoveError::NoPieceInPool)
                        }

                        self.chess_board2.board[i][j] = p;
                        self.white_active_2 = !self.white_active_2;

                        self.chess_board2.board[i][j] = p;
                        self.white_active_2 = !self.white_active_2;
    
                        if p == Piece::P {
                            self.pawn_in_last_turn_b2 = Some((i,j));
                            self.half_moves_last_capture2=0;
                        }else{
                            self.pawn_in_last_turn_b2 = None;
                            self.half_moves_last_capture2+=1;
                        }

                        if p == Piece::R {
                            if i==7 && j==7 {
                                self.chess_board2.white_rook_k_moved = false;
                            }
                            if i==7 && j==0 {
                                self.chess_board2.white_rook_q_moved = false;
                            }
                        }
                        return Ok(true)
                    }else { 
                        return Err(MoveError::CannotDeploy)
                    }
                }
            }else{
                if self.white_active_2 { 
                    return Err(MoveError::NotTurn)
                } else {
                  
                    if let Some(ind) = self.box_index(p) {
                        if self.board2_black_capture[ind]>0 {
                            self.board2_black_capture[ind]-=1;
                        }else{
                           return Err(MoveError::NotTurn)
                        }

                        self.chess_board2.board[i][j] = p;
                        self.white_active_2 = !self.white_active_2;
                        
                        if p == Piece::p {
                            self.pawn_in_last_turn_b2 = Some((i,j));
                            self.half_moves_last_capture2=0;
                        }else{
                            self.pawn_in_last_turn_b2 = None;
                            self.half_moves_last_capture2+=1;
                        }

                        if p == Piece::r {
                            if i==0 && j==7 {
                                self.chess_board2.black_rook_k_moved = false;
                            }
                            if i==0 && j==0 {
                                self.chess_board2.black_rook_q_moved = false;
                            }
                        }
                        self.movectr2+=1;
                       return Ok(true)
                    } else {
                        return Err(MoveError::CannotDeploy)
                    }
                }
            }
        }
        
        return Err(MoveError::CannotDeploy)
    }


    //do not forget the side effects while testing
    pub fn movemaker(&mut self, board1:bool, i_old:usize,j_old:usize,i:usize,j:usize) -> Result<bool,MoveError> {
        if self.winner!=Winner::N {
            return Err(MoveError::AlreadyOver)
        }
        
        //check of move is legal
        if self.legality_check(board1,i_old,j_old,i,j) {
            if board1 {
                match self.get_board(board1).board[i_old][j_old] {
                    Piece::R => {
                        if !self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            if i_old == 7 && j_old == 7 {
                                self.get_board(board1).white_rook_k_moved=true;
                            }
                            if i_old == 7 && j_old == 0 {
                                self.get_board(board1).white_rook_q_moved=true;
                            }
                        }
                    },
                    Piece::r => {
                        if self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            if i_old == 0 && j_old == 7 {
                                self.get_board(board1).black_rook_k_moved=true;
                            }
                            if i_old == 0 && j_old == 0 {
                                self.get_board(board1).black_rook_q_moved=true;
                            }
                        }
                    },

                    Piece::K => 
                    {
                        if !self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b1 = None;
                           
                            //check if the move is a castling move
                            if i_old==7 && j_old==4 && i== 7 && (j==6 || j==2)
                            && !self.chess_board1.white_k_moved {
                                //we are going castle
                                self.chess_board1.board[i_old][j_old]=Piece::E;
                                self.chess_board1.board[i][j] = Piece::K;
                                if j==6 {
                                    self.chess_board1.board[i][j-1] = Piece::R;
                                    self.chess_board1.board[7][7] = Piece::E;
                                }else{
                                    self.chess_board1.board[i][j+1] = Piece::R;
                                    self.chess_board1.board[7][0] = Piece::E;
                                }
                                self.chess_board1.white_k_moved = true;
                                self.white_active_1 = !self.white_active_1;

                                self.half_moves_last_capture1 += 1;
                                return Ok(true)
                            }
                            self.chess_board1.white_k_moved = true;
                        }
                    },
                    Piece::k => 
                    {
                        if self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b1 = None;

                            if i_old==0 && j_old==4 && i== 0 && (j==6 || j==2)
                            && !self.chess_board1.black_k_moved
                            {
                                //we are going castle
                                self.chess_board1.board[i_old][j_old]=Piece::E;
                                self.chess_board1.board[i][j] = Piece::K;
                                if j==6 {
                                    self.chess_board1.board[i][j-1] = Piece::R;
                                    self.chess_board1.board[0][7] = Piece::E;
                                }else{
                                    self.chess_board1.board[i][j+1] = Piece::R;
                                    self.chess_board1.board[0][0] = Piece::E;
                                }
                                self.chess_board1.black_k_moved = true;
                                self.white_active_1 = !self.white_active_1;

                                self.half_moves_last_capture1 += 1;
                                self.movectr1+=1;
                                return Ok(true)
                            }
                        
                            self.chess_board1.black_k_moved = true;
                        }
                    },
                    Piece::P => 
                    {
                        if !self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b1 = Some((i,j));
                            if i==0 {
                                if self.upgrade_to1 == Piece::Q ||  self.upgrade_to1 == Piece::R  || self.upgrade_to1 == Piece::B  ||self.upgrade_to1 == Piece::N {
                                    self.half_moves_last_capture1=0;
                                    match self.box_index(self.chess_board1.board[i][j]) {
                                        None => {},
                                        Some(x) => {self.board2_black_capture[x] += 1},
                                    }
                                }
                            
                                match self.upgrade_to1 {
                                    Piece::Q => { self.upgrade_to1 = Piece::E; self.chess_board1.board[i][j] = Piece::UQ;
                                        self.chess_board1.board[i_old][j_old] = Piece::E; 
                                        self.white_active_1 = !self.white_active_1; return Ok(true)},
                                    Piece::R => { self.upgrade_to1 = Piece::E; self.chess_board1.board[i][j] = Piece::UR;
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    Piece::B => { self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::UB;
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    Piece::N => { self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::UN;
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    _ =>  {return Err(MoveError::PromotionProblem)},
                            } 
                            }
                        }
                    },
                    Piece::p => 
                    {
                        if self.white_active_1 {
                            return Err(MoveError::NotTurn)
                        }else {
                            self.pawn_in_last_turn_b1 = Some((i,j));
                            if i==7 {
                                if self.upgrade_to1 == Piece::q ||  self.upgrade_to1 == Piece::r  || self.upgrade_to1 == Piece::b  ||self.upgrade_to1 == Piece::n {
                                    self.half_moves_last_capture1=0;
                                    match self.box_index(self.chess_board1.board[i][j]) {
                                        None => {},
                                        Some(x) => {self.board2_white_capture[x] += 1},
                                    }
                                }

                                match self.upgrade_to1 {
                                    Piece::q => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Uq;
                                    self.chess_board1.board[i_old][j_old] = Piece::E;
                                    self.white_active_1 = !self.white_active_1; return Ok(true)},
                                    Piece::r => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Ur; 
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    Piece::b => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Ub; 
                                        self.chess_board1.board[i_old][j_old] = Piece::E;
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    Piece::n => {self.upgrade_to1 = Piece::E;self.chess_board1.board[i][j] = Piece::Un;
                                        self.chess_board1.board[i_old][j_old] = Piece::E; 
                                        self.white_active_1 = !self.white_active_1;return Ok(true)},
                                    _ =>  {return Err(MoveError::PromotionProblem)},
                                } 
                            }
                        }
                    },
                    _ => {},
                }

               
                if !((self.white_active_1 && self.is_white(board1,i_old,j_old)) 
                || (!self.white_active_1 && !self.is_white(board1,i_old,j_old))) {
                    //not your turn!!!
                    //println!("Siktir git hamlen degil!");
                    return Err(MoveError::NotTurn)
                }
                //check to move location
                let tmp = self.chess_board1.board[i][j];

                //check if any piece is captured 
                if tmp!=Piece::E {
                    self.half_moves_last_capture1=0;
                }else {
                    self.half_moves_last_capture1+=1;
                }
               

                //check if pawn is moved
                if self.chess_board1.board[i_old][j_old]==Piece::P {

                    self.pawn_in_last_turn_b1 = Some((i,j));
                    self.half_moves_last_capture1=0;

                }else if self.chess_board1.board[i_old][j_old]==Piece::p {

                    self.pawn_in_last_turn_b1 = Some((i,j));
                    self.half_moves_last_capture1=0;

                //if the king has moved change king has moved flag
                }else if self.chess_board1.board[i_old][j_old]==Piece::K{
                    
                    self.pawn_in_last_turn_b1 = None;
                    self.chess_board1.white_k_moved = true;
                
                }else if self.chess_board1.board[i_old][j_old]==Piece::k{

                    self.pawn_in_last_turn_b1 = None;
                    self.chess_board1.black_k_moved = true;
           
                }else{
                    //no king no pawn movement -> pawn in last turn = None
                    self.pawn_in_last_turn_b1 = None;
                }

              

                //check if black has moved
                if !self.is_white(board1,i_old,j_old){
                    self.movectr1+=1;
                }

                //check if game  should end
                if tmp==Piece::K || tmp==Piece::k  {
                    self.finish_up(tmp,board1);
                }


                if self.check_patt(board1,true) || self.check_patt(board1,false) {
                    self.winner = Winner::P;
                    return Err(MoveError::AlreadyOver)
                }


                //send captured piece to your ally
                match self.box_index(tmp) {
                    None => (), //nothing to di
                    Some(a) => {
                        if self.is_white(board1,i,j) {
                            self.board2_white_capture[a]+=1;
                        }else{
                            self.board2_black_capture[a]+=1;
                        }
                    },
                }

                //apply move
                self.chess_board1.board[i][j]=self.chess_board1.board[i_old][j_old];
                self.chess_board1.board[i_old][j_old]=Piece::E;
                self.white_active_1 = !self.white_active_1;
                return Ok(true)

            }else{
                match self.chess_board2.board[i_old][j_old] {
                    Piece::R => {
                        if !self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            if i_old == 7 && j_old == 7 {
                                self.chess_board2.white_rook_k_moved=true;
                            }
                            if i_old == 7 && j_old == 0 {
                                self.chess_board2.white_rook_q_moved=true;
                            }
                        }
                    },
                    Piece::r => {
                        if self.white_active_2 {
                            return Err(MoveError::NotTurn) 
                        }else{
                            if i_old == 0 && j_old == 7 {
                                self.chess_board2.black_rook_k_moved=true;
                            }
                            if i_old == 0 && j_old == 0 {
                                self.chess_board2.black_rook_q_moved=true;
                            }
                        }
                    },

                    Piece::K => 
                    {
                        if !self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b2 = None;

                            if i_old==7 && j_old==4
                            && i== 7 && (j==6 || j==2)
                            && !self.chess_board2.white_k_moved
                            {
                                //we are going castle
                                self.chess_board2.board[i_old][j_old]=Piece::E;
                                self.chess_board2.board[i][j] = Piece::K;
                                if j==6 {
                                    self.chess_board2.board[i][j-1] = Piece::R;
                                    self.chess_board2.board[7][7] = Piece::E;
                                }else{
                                    self.chess_board2.board[i][j+1] = Piece::R;
                                    self.chess_board2.board[7][0] = Piece::E;
                                }
                                self.chess_board2.white_k_moved = true;
                                self.white_active_2 = !self.white_active_2;

                                self.half_moves_last_capture2 += 1;
                                return Ok(true)
                            }
                            self.chess_board2.white_k_moved = true;
                        }
                    },
                    Piece::k => 
                    {
                        if self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b2 = None;
                            if i_old==0 && j_old==4
                            && i== 0 && (j==6 || j==2)
                            && !self.chess_board2.black_k_moved
                            {
                                //we are going castle
                                self.chess_board2.board[i_old][j_old]=Piece::E;
                                self.chess_board2.board[i][j] = Piece::K;
                                if j==6 {
                                    self.chess_board2.board[i][j-1] = Piece::R;
                                    self.chess_board2.board[0][7] = Piece::E;
                                }else{
                                    self.chess_board2.board[i][j+1] = Piece::R;
                                    self.chess_board2.board[0][0] = Piece::E;
                                }
                                self.chess_board2.black_k_moved = true;
                                self.white_active_2 = !self.white_active_2;

                                self.half_moves_last_capture2 += 1;
                                self.movectr2+=1;
                                return Ok(true)
                        }
                        
                        self.chess_board2.black_k_moved = true;
                        }
                    },
                    Piece::P => 
                    {
                        if !self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            if i==0 {
                                self.pawn_in_last_turn_b2 = Some((i,j));
                                if self.upgrade_to2 == Piece::Q ||  self.upgrade_to2 == Piece::R  
                                || self.upgrade_to2 == Piece::B || self.upgrade_to2 == Piece::N {
                                    self.half_moves_last_capture2=0;
                                    match self.box_index(self.chess_board2.board[i][j]) {
                                        None => {},
                                        Some(x) => {self.board1_black_capture[x] += 1},
                                    }
                                }
                                match self.upgrade_to2 {
                                    Piece::Q => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UQ;
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::R => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UR;
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::B => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UB;
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::N => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::UN;
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    _ =>  {return Err(MoveError::PromotionProblem)},
                            } 
                            }
                        }
                    },
                    Piece::p => 
                    {
                        if self.white_active_2 {
                            return Err(MoveError::NotTurn)
                        }else{
                            self.pawn_in_last_turn_b2 = Some((i,j));
                            if i==7 {
                                if self.upgrade_to2 == Piece::q || self.upgrade_to2 == Piece::r 
                                || self.upgrade_to2 == Piece::b || self.upgrade_to2 == Piece::n {
                                    self.half_moves_last_capture2=0;
                                    match self.box_index(self.chess_board2.board[i][j]) {
                                        None => {},
                                        Some(x) => {self.board1_white_capture[x] += 1},
                                    }
                                }

                                match self.upgrade_to2 {
                                    Piece::q => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Uq; 
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::r => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Ur; 
                                            self.chess_board2.board[i_old][j_old] = Piece::E;
                                            self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::b => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Ub; 
                                        self.chess_board2.board[i_old][j_old] = Piece::E;
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    Piece::n => { self.upgrade_to2 = Piece::E;self.chess_board2.board[i][j] = Piece::Un;
                                        self.chess_board2.board[i_old][j_old] = Piece::E; 
                                        self.white_active_2 = !self.white_active_2;return Ok(true)},
                                    _ =>  {return Err(MoveError::PromotionProblem)},
                                } 
                        }
                    }
                    },
                    _ => {},
                }

                if !( (self.white_active_2 && self.is_white(board1,i_old,j_old)) 
                || (!self.white_active_2 && !self.is_white(board1,i_old,j_old))) {
                    //not your turn!!!
                    //println!("Siktir git hamlen degil!");
                    return Err(MoveError::NotTurn)
                }


                let tmp = self.chess_board2.board[i][j];

                //check if any piece is captured 
                if tmp!=Piece::E {
                    self.half_moves_last_capture2=0;
                }else {
                    self.half_moves_last_capture2+=1;
                }

                //check if pawn is moved
                if self.chess_board2.board[i_old][j_old]==Piece::P {
                    self.pawn_in_last_turn_b2 = Some((i,j));
                    self.half_moves_last_capture2=0;
                }else if self.chess_board2.board[i_old][j_old]==Piece::K {
                    self.pawn_in_last_turn_b2 = None;
                    self.chess_board2.white_k_moved = true;
                }else if self.chess_board2.board[i_old][j_old]==Piece::k {
                    self.pawn_in_last_turn_b2 = None;
                    self.chess_board2.black_k_moved = true;
                }else{
                    self.pawn_in_last_turn_b2 = None;
                }

                //check if black has moved
                if !self.is_white(board1,i_old,j_old){
                    self.movectr2+=1;
                }

 

                //check if game  should end
                if tmp==Piece::K || tmp==Piece::k  {
                    self.finish_up(tmp,board1);
                }

                if self.check_patt(board1,true) || self.check_patt(board1,false) {
                    self.winner = Winner::P;
                    return Err(MoveError::AlreadyOver)
                }
                
                match self.box_index(tmp) {
                    None => {}, //nothing to di
                    Some(a) => {
                        if self.is_white(board1,i,j) {
                            self.board1_white_capture[a]+=1;
                        }else{
                            self.board1_black_capture[a]+=1;
                        }
                    },
                }
                self.chess_board2.board[i][j]=self.chess_board2.board[i_old][j_old];
                self.chess_board2.board[i_old][j_old]=Piece::E;
                self.white_active_2 = !self.white_active_2;
                return Ok(true)
            }
        }else{
            println!("Move not legal!");
            return Err(MoveError::NotLegal)
        }
    }

    //get the index of the piece on the captured pieces array
    pub fn box_index(&self, piece:Piece) -> Option<usize> {
        //P-R-N-B-Q
        match piece {
            Piece::P | Piece::p  => Some(0),
            Piece::K | Piece::k | Piece::L  | Piece::E => None ,
            Piece::N | Piece::n | Piece::Un | Piece::UN => Some(2),
            Piece::R | Piece::r | Piece::Ur | Piece::UR => Some(1),
            Piece::B | Piece::b | Piece::Ub | Piece::UB => Some(3),
            Piece::Q | Piece::q | Piece::Uq | Piece::UQ => Some(4),

        }
    }

    //stub
    //TODO: finish_up function
    fn finish_up(&mut self, p:Piece, board1:bool){
        if p==Piece::K {
            if board1 {
                self.winner=Winner::B1;
            }else{
                self.winner=Winner::B2;
            }
        }else if p==Piece::k {
            if board1 {
                self.winner=Winner::W1;
            }else{
                self.winner=Winner::W2;
            }
        }
    }

    pub fn check_patt(&mut self,board1:bool, white:bool) -> bool {
        let mut pic = Piece::E;
        if white { pic = Piece::K; } else { pic = Piece::k; }

        for i in 0..8 {
            for j in 0..8 {
                if white {
                    if self.is_white(board1,i,j) && self.get_board(board1).board[i][j]!=Piece::K {
                        if self.get_legal_moves(board1,i,j)!=Vec::new() {
                            return false
                        }
                        
                    }
                }else{
                    if !self.is_white(board1,i,j) &&  self.get_board(board1).board[i][j]!=Piece::k {
                        if self.get_legal_moves(board1,i,j)!=Vec::new() {
                            return false
                        }
                    }
                }
            }
        }

        match self.find_piece(pic,board1) {
            None => return false,
            Some((i,j)) => {
                if !self.is_attacked(board1,white,i,j) && 
                self.pool_empty(board1,white){
                    self.winner=Winner::P;
                    return true
                }
            },
        }
        return false
    }

    fn pool_empty(&self, board1:bool,white:bool) -> bool {
        for i in 0..5 {
            if board1 {
                if white {
                   if self.board1_white_capture[i]!=0 {return false}
                }else{
                   if self.board1_black_capture[i]!=0 {return false}
                }
            }else{
                if white {
                    if self.board2_white_capture[i]!=0 {return false}
                }else{
                    if self.board2_black_capture[i]!=0 {return false}
                }
            }
        }
        true
    }

    fn recv_piece(&mut self, board1:bool, white:bool,p:Piece){
        let i = self.box_index(p);
        if let Some(x) = i { 
            if board1 {
                if white { 
                    self.board1_white_capture[x]+=1;
                }else{
                    self.board1_black_capture[x]+=1;
                }
            }else{
                if white {
                    self.board2_white_capture[x]+=1;
                }else{
                    self.board2_black_capture[x]+=1;
                }
            }
        }
    }

    pub fn find_piece(&self, p: Piece, board1:bool) -> Option<(usize,usize)> {
        for i in 0..8 {
            for j in 0..8 {
                if board1 {
                    if self.chess_board1.board[i][j] == p {
                        return Some((i,j))
                     }
                }else{
                    if self.chess_board2.board[i][j] == p {
                        return Some((i,j))
                    }
                }
              
            }
        }
        None 
    }

    pub fn get_castling_rights(&self,board1:bool) -> [bool;4] {
        let mut x = [false;4];
        x[0] = self.get_board_n(board1).white_rook_k_moved;
        x[1] = self.get_board_n(board1).white_rook_q_moved;
        x[2] = self.get_board_n(board1).black_rook_k_moved;
        x[3] = self.get_board_n(board1).black_rook_q_moved;
        return x
    }
    
}







